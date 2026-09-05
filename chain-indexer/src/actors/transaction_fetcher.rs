use std::future::Future;
use std::time::Duration;

use crate::actors::header_fetcher::node_best_header_id;
use crate::types::work_object::WorkBlock;
use ergo_node_client::apis::{blocks_api, configuration::Configuration, info_api};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Instant;
use tracing::{debug, error, instrument, warn};

/// Grace period of persistent misses, measured from the moment the node
/// first confirms it actually has all the missing bodies, before we start
/// asking whether a missing header was orphaned.
const GRACE_PERIOD: Duration = Duration::from_secs(30);
/// Hard cap on how long we'll keep retrying a single batch once the node has
/// confirmed it has the missing bodies (whether the batch is truly orphaned
/// or not). Time spent waiting for the node's full-block height to catch up
/// to its header chain never counts toward this.
const HARD_CAP: Duration = Duration::from_secs(10 * 60);
/// Ceiling on the exponential retry backoff.
const MAX_RETRY_BACKOFF: Duration = Duration::from_secs(10);

fn backoff_for(attempts: u32) -> Duration {
    let backoff_secs = 1u64.checked_shl(attempts).unwrap_or(u64::MAX);
    Duration::from_secs(backoff_secs).min(MAX_RETRY_BACKOFF)
}

/// Requested (id, height) pairs missing from the response (each requested id
/// must appear in the response exactly once). `missing` is by construction a
/// subset of `requested_ids`, built directly from a zip of the requested ids
/// and heights (heights are unique within a batch, so there's nothing to
/// dedupe) rather than looking each id's height up separately.
fn missing_pairs(
    requested_ids: &[String],
    requested_heights: &[i32],
    returned_ids: &[String],
) -> Vec<(String, i32)> {
    requested_ids
        .iter()
        .zip(requested_heights.iter())
        .filter(|(id, _)| returned_ids.iter().filter(|r| r == id).count() != 1)
        .map(|(id, height)| (id.clone(), *height))
        .collect()
}

/// What to do about a batch that `/blocks/headerIds` keeps returning
/// incomplete.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BatchAction {
    /// Wait this long, then re-request the batch.
    Retry(Duration),
    /// Stop retrying and return from `fetch_transactions` (the reason is
    /// logged by the caller); this lets the supervisor exit the process and
    /// resync from the DB tip.
    GiveUp(String),
}

/// One requested header id still missing from the batch response, together
/// with the node's canonical header id at that height. `canonical` is `None`
/// when it hasn't been looked up (we only look once the batch is past the
/// grace period, and stop at the first mismatch rather than resolving every
/// missing id on every retry) or the lookup itself failed.
#[derive(Debug, Clone)]
struct MissingHeader {
    id: String,
    height: i32,
    canonical: Option<String>,
}

/// Retry/give-up policy for an incomplete `/blocks/headerIds` batch.
///
/// `stalled` is `None` while the node's own `full_height` hasn't yet reached
/// the highest missing header's height: `/blocks/chainSlice` follows the
/// node's *header* chain, which can legitimately run ahead of its full
/// blocks for seconds (a block burst) or hours (a node restart/resync), and
/// none of that time should count against the batch, so we always keep
/// retrying with backoff and never give up in that state.
///
/// Once the node has confirmed it has the missing bodies, `stalled` becomes
/// `Some(elapsed)` measuring time since that confirmation: exponential
/// backoff (1s, 2s, 4s, ... capped at 10s) for the first 30s; past that, give
/// up immediately if any missing id's height now resolves to a different
/// canonical header (it was orphaned by a fork); otherwise keep retrying
/// until a 10-minute hard cap (checked first, before any canonical lookups),
/// then give up too.
fn incomplete_batch_action(
    stalled: Option<Duration>,
    attempts: u32,
    missing: &[MissingHeader],
) -> BatchAction {
    let backoff = backoff_for(attempts);

    let Some(elapsed) = stalled else {
        // The node itself hasn't caught up to the missing heights yet; this
        // isn't a stall, so never give up here.
        return BatchAction::Retry(backoff);
    };

    if elapsed >= HARD_CAP {
        return BatchAction::GiveUp(format!(
            "get_full_block_by_ids still incomplete {:?} after the node caught up (attempt {}); giving up",
            elapsed,
            attempts + 1
        ));
    }

    if elapsed >= GRACE_PERIOD {
        for m in missing {
            if let Some(canonical) = &m.canonical {
                if canonical != &m.id {
                    return BatchAction::GiveUp(format!(
                        "header {} at height {} was orphaned (node now has {}); exiting so the supervisor resyncs from the DB tip",
                        m.id, m.height, canonical
                    ));
                }
            }
        }
    }

    BatchAction::Retry(backoff)
}

/// Looks up the node's canonical header id for each missing header in turn,
/// stopping as soon as one doesn't match (an orphan) instead of resolving
/// the whole batch on every retry. Entries after the first mismatch (or all
/// of them, if none mismatches) keep `canonical: None`.
async fn resolve_missing_headers<CF, CFut>(
    missing: &[(String, i32)],
    mut canonical_id_at: CF,
) -> Vec<MissingHeader>
where
    CF: FnMut(i32) -> CFut,
    CFut: Future<Output = Option<String>>,
{
    let mut out = Vec::with_capacity(missing.len());
    let mut found_orphan = false;
    for (id, height) in missing {
        let canonical = if found_orphan {
            None
        } else {
            let canonical = canonical_id_at(*height).await;
            if let Some(c) = &canonical {
                if c != id {
                    found_orphan = true;
                }
            }
            canonical
        };
        out.push(MissingHeader {
            id: id.clone(),
            height: *height,
            canonical,
        });
    }
    out
}

/// Fetches one batch of full blocks under the incomplete-batch retry policy,
/// with every bit of IO injected so the policy can be exercised in tests
/// without a real node or real waiting.
///
/// `fetch_blocks` performs one `/blocks/headerIds`-equivalent call and
/// returns the (header id, block) pairs actually served; `node_full_height`
/// queries the node's current `full_height` (`None` on error); `canonical_id_at`
/// resolves the node's canonical header id at a height (`None` on error or
/// miss); `sleep_fn` is the backoff/retry sleep.
///
/// `fetch_blocks` returning `Err` means the node itself couldn't be reached
/// (connection refused, timeout, ...), as opposed to an `Ok` response that's
/// silently incomplete. That case retries forever on a flat 1s sleep and
/// never counts toward the incomplete-batch backoff or hard cap: giving up
/// while the node is down just makes the container flap, since startup
/// (`main.rs`) itself blocks on reaching the node.
async fn fetch_batch_with_retry<B, FF, FFut, E, NF, NFut, CF, CFut, SF, SFut>(
    requested_ids: &[String],
    requested_heights: &[i32],
    mut fetch_blocks: FF,
    mut node_full_height: NF,
    mut canonical_id_at: CF,
    mut sleep_fn: SF,
) -> Result<Vec<B>, String>
where
    FF: FnMut() -> FFut,
    FFut: Future<Output = Result<Vec<(String, B)>, E>>,
    E: std::fmt::Display,
    NF: FnMut() -> NFut,
    NFut: Future<Output = Option<i32>>,
    CF: FnMut(i32) -> CFut,
    CFut: Future<Output = Option<String>>,
    SF: FnMut(Duration) -> SFut,
    SFut: Future<Output = ()>,
{
    let mut attempts: u32 = 0;
    let mut stalled_since: Option<Instant> = None;

    loop {
        match fetch_blocks().await {
            Err(e) => {
                warn!(
                    "get_full_block_by_ids failed (attempt {}): {}",
                    attempts + 1,
                    e
                );
                sleep_fn(Duration::from_secs(1)).await;
                // Node-unreachable errors never count toward the
                // incomplete-batch backoff or hard cap.
                continue;
            }
            Ok(pairs) => {
                let returned_ids: Vec<String> = pairs.iter().map(|(id, _)| id.clone()).collect();
                let missing = missing_pairs(requested_ids, requested_heights, &returned_ids);

                if missing.is_empty() {
                    return Ok(pairs.into_iter().map(|(_, b)| b).collect());
                }

                warn!(
                    "get_full_block_by_ids returned an incomplete batch (attempt {}); missing ids: {:?}",
                    attempts + 1,
                    missing.iter().map(|(id, _)| id).collect::<Vec<_>>()
                );

                let max_missing_height = missing
                    .iter()
                    .map(|(_, height)| *height)
                    .max()
                    .expect("missing is non-empty");

                match node_full_height().await {
                    Some(full_height) if full_height >= max_missing_height => {
                        stalled_since.get_or_insert_with(Instant::now);
                    }
                    Some(full_height) => {
                        warn!(
                            "node full_height {} is below missing height {}; waiting",
                            full_height, max_missing_height
                        );
                        stalled_since = None;
                    }
                    None => {
                        warn!(
                            "failed to query node full_height (attempt {}); not counting toward the stall",
                            attempts + 1
                        );
                        // Leave stalled_since untouched: we can't tell
                        // whether the node is still behind or not, so
                        // neither confirm nor cancel a stall in progress.
                    }
                }

                let stalled = stalled_since.map(|since| since.elapsed());

                // Check the hard cap before doing any canonical-id lookups,
                // and only bother looking at all once past the grace period.
                let needs_orphan_check = matches!(stalled, Some(e) if e >= GRACE_PERIOD && e < HARD_CAP);
                let missing_headers = if needs_orphan_check {
                    resolve_missing_headers(&missing, &mut canonical_id_at).await
                } else {
                    missing
                        .iter()
                        .map(|(id, height)| MissingHeader {
                            id: id.clone(),
                            height: *height,
                            canonical: None,
                        })
                        .collect()
                };

                match incomplete_batch_action(stalled, attempts, &missing_headers) {
                    BatchAction::Retry(delay) => {
                        sleep_fn(delay).await;
                        attempts += 1;
                        continue;
                    }
                    BatchAction::GiveUp(reason) => return Err(reason),
                }
            }
        }
    }
}

#[instrument]
pub async fn fetch_transactions(
    node_conf: &Configuration,
    mut receiver: Receiver<Vec<WorkBlock>>,
    sender: Sender<WorkBlock>,
) {
    while let Some(work_blocks) = receiver.recv().await {
        let first_work_block = work_blocks.first().unwrap();
        match first_work_block.header.clone() {
            Some(header) => {
                if first_work_block.zmq_mode {
                    debug!("fetch_transactions: {}", header.id);
                }
                let fetch_start = std::time::Instant::now();
                let requested_ids: Vec<String> = work_blocks
                    .iter()
                    .map(|wb| wb.header.clone().unwrap().id)
                    .collect();
                let requested_heights: Vec<i32> = work_blocks
                    .iter()
                    .map(|wb| wb.header.clone().unwrap().height)
                    .collect();

                let result = fetch_batch_with_retry(
                    &requested_ids,
                    &requested_heights,
                    || {
                        let ids = requested_ids.clone();
                        async move {
                            blocks_api::get_full_block_by_ids(node_conf, ids)
                                .await
                                .map(|blocks| {
                                    blocks
                                        .into_iter()
                                        .map(|b| (b.header.id.clone(), b))
                                        .collect::<Vec<_>>()
                                })
                        }
                    },
                    || async move {
                        info_api::get_node_info(node_conf)
                            .await
                            .ok()
                            .and_then(|info| info.full_height)
                    },
                    |height| async move { node_best_header_id(node_conf, height).await },
                    |delay| tokio::time::sleep(delay),
                )
                .await;

                let full_blocks = match result {
                    Ok(full_blocks) => full_blocks,
                    Err(reason) => {
                        error!("{}", reason);
                        return;
                    }
                };

                // Build the outgoing sequence in requested order, looking up
                // each block by header id rather than trusting the response
                // order.
                let ordered_blocks: Vec<_> = requested_ids
                    .iter()
                    .map(|id| {
                        full_blocks
                            .iter()
                            .find(|b| &b.header.id == id)
                            .expect("id validated as present by fetch_batch_with_retry")
                            .clone()
                    })
                    .collect();

                let mut block_stream = async_std::stream::from_iter(ordered_blocks);
                while let Some(block) = block_stream.next().await {
                    match sender
                        .send(WorkBlock {
                            zmq_mode: first_work_block.zmq_mode,
                            header: Some(*block.header.clone()),
                            transactions: Some(block.block_transactions.transactions.clone()),
                            rollback_height: None,
                        })
                        .await
                    {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    }
                }

                if header.height % 1000 == 0 {
                    debug!(
                        "Transaction channel size: {}",
                        (&sender.max_capacity() - &sender.capacity())
                    );
                }
                if first_work_block.zmq_mode || header.height % 100 == 0 {
                    debug!(
                        "fetch_transactions batch at height {} took {:?}",
                        header.height,
                        fetch_start.elapsed()
                    );
                }
            }
            None => match sender
                .send(WorkBlock {
                    zmq_mode: first_work_block.zmq_mode,
                    header: None,
                    transactions: None,
                    rollback_height: first_work_block.rollback_height,
                })
                .await
            {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(vals: &[&str]) -> Vec<String> {
        vals.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn complete_response_has_no_missing_pairs() {
        let requested = ids(&["a", "b", "c"]);
        let heights = vec![1, 2, 3];
        let returned = ids(&["a", "b", "c"]);
        assert_eq!(missing_pairs(&requested, &heights, &returned), Vec::new());
    }

    #[test]
    fn missing_one_id_is_reported_with_its_height() {
        let requested = ids(&["a", "b", "c"]);
        let heights = vec![10, 20, 30];
        let returned = ids(&["a", "c"]);
        assert_eq!(
            missing_pairs(&requested, &heights, &returned),
            vec![("b".to_string(), 20)]
        );
    }

    #[test]
    fn duplicate_id_in_response_is_reported() {
        // "a" comes back twice and "b" not at all; a returned id is not
        // "exactly once" so it is flagged same as an outright miss.
        let requested = ids(&["a", "b"]);
        let heights = vec![1, 2];
        let returned = ids(&["a", "a"]);
        assert_eq!(
            missing_pairs(&requested, &heights, &returned),
            vec![("a".to_string(), 1), ("b".to_string(), 2)]
        );
    }

    #[test]
    fn empty_response_reports_everything_missing() {
        let requested = ids(&["a", "b", "c"]);
        let heights = vec![1, 2, 3];
        let returned: Vec<String> = Vec::new();
        assert_eq!(
            missing_pairs(&requested, &heights, &returned),
            vec![
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 3)
            ]
        );
    }

    fn missing_header(id: &str, height: i32, canonical: Option<&str>) -> MissingHeader {
        MissingHeader {
            id: id.to_string(),
            height,
            canonical: canonical.map(|s| s.to_string()),
        }
    }

    #[test]
    fn node_behind_never_gives_up_and_does_not_accrue() {
        // Even with a huge attempt count and a missing header that would
        // read as orphaned if checked, `stalled: None` (node hasn't reached
        // the missing height yet) must always retry, never give up.
        let missing = vec![missing_header("a", 100, Some("some-other-id"))];
        assert_eq!(
            incomplete_batch_action(None, 999, &missing),
            BatchAction::Retry(MAX_RETRY_BACKOFF)
        );
        assert_eq!(
            incomplete_batch_action(None, 0, &missing),
            BatchAction::Retry(Duration::from_secs(1))
        );
    }

    #[test]
    fn grace_period_starts_fresh_once_node_catches_up() {
        // The node may have been behind for hours before this; once it
        // reports caught up, the elapsed clock for grace/hard-cap purposes
        // restarts from there rather than reflecting that history.
        let missing = vec![missing_header("a", 100, Some("some-other-id"))];
        assert_eq!(
            incomplete_batch_action(Some(Duration::from_secs(1)), 50, &missing),
            BatchAction::Retry(MAX_RETRY_BACKOFF)
        );
    }

    #[test]
    fn retries_with_growing_backoff_before_grace_period() {
        let missing = vec![missing_header("a", 100, None)];
        let stalled = Some(Duration::from_secs(5));
        assert_eq!(
            incomplete_batch_action(stalled, 0, &missing),
            BatchAction::Retry(Duration::from_secs(1))
        );
        assert_eq!(
            incomplete_batch_action(stalled, 1, &missing),
            BatchAction::Retry(Duration::from_secs(2))
        );
        assert_eq!(
            incomplete_batch_action(stalled, 2, &missing),
            BatchAction::Retry(Duration::from_secs(4))
        );
        assert_eq!(
            incomplete_batch_action(stalled, 3, &missing),
            BatchAction::Retry(Duration::from_secs(8))
        );
        // Backoff caps at 10s from here on, well before the grace period.
        assert_eq!(
            incomplete_batch_action(stalled, 4, &missing),
            BatchAction::Retry(Duration::from_secs(10))
        );
    }

    #[test]
    fn backoff_cap_holds_for_a_consistent_elapsed_attempts_pair() {
        // 1 + 2 + 4 + 8 + 10 = 25s is the cumulative sleep time actually
        // elapsed by the time attempt 5 (0-indexed) is made, so this
        // (elapsed, attempts) pair is one the loop could really reach —
        // unlike the old (5s, 10 attempts) case, which required ~50s of
        // sleeping to have happened within 5 elapsed seconds.
        let missing = vec![missing_header("a", 100, None)];
        let stalled = Some(Duration::from_secs(25));
        assert_eq!(
            incomplete_batch_action(stalled, 5, &missing),
            BatchAction::Retry(Duration::from_secs(10))
        );
    }

    #[test]
    fn gives_up_when_canonical_id_differs_after_grace_period() {
        let missing = vec![missing_header("ccd962c0", 1864378, Some("other-canonical-id"))];
        let stalled = Some(GRACE_PERIOD + Duration::from_secs(1));
        match incomplete_batch_action(stalled, 5, &missing) {
            BatchAction::GiveUp(reason) => {
                assert!(reason.contains("ccd962c0"));
                assert!(reason.contains("1864378"));
                assert!(reason.contains("other-canonical-id"));
                assert!(reason.contains("orphaned"));
            }
            other => panic!("expected GiveUp, got {:?}", other),
        }
    }

    #[test]
    fn still_retries_when_canonical_id_matches_and_under_hard_cap() {
        let missing = vec![missing_header("a", 100, Some("a"))];
        let stalled = Some(GRACE_PERIOD + Duration::from_secs(60));
        assert_eq!(
            incomplete_batch_action(stalled, 2, &missing),
            BatchAction::Retry(Duration::from_secs(4))
        );
    }

    #[test]
    fn still_retries_when_canonical_id_not_yet_known_and_under_hard_cap() {
        // Canonical lookup itself failed (or wasn't attempted); we can't
        // confirm an orphan, so keep retrying rather than giving up.
        let missing = vec![missing_header("a", 100, None)];
        let stalled = Some(GRACE_PERIOD + Duration::from_secs(60));
        assert_eq!(
            incomplete_batch_action(stalled, 0, &missing),
            BatchAction::Retry(Duration::from_secs(1))
        );
    }

    #[test]
    fn gives_up_when_over_hard_cap_even_if_canonical_id_matches() {
        let missing = vec![missing_header("a", 100, Some("a"))];
        let stalled = Some(HARD_CAP + Duration::from_secs(1));
        match incomplete_batch_action(stalled, 50, &missing) {
            BatchAction::GiveUp(reason) => assert!(reason.contains("giving up")),
            other => panic!("expected GiveUp, got {:?}", other),
        }
    }

    // --- loop-wiring tests: fetch_batch_with_retry with injected IO ---

    #[tokio::test(start_paused = true)]
    async fn orphan_gives_up_after_grace_period() {
        let requested_ids = ids(&["a", "b"]);
        let requested_heights = vec![10, 11];

        let result = fetch_batch_with_retry(
            &requested_ids,
            &requested_heights,
            || async { Ok::<_, String>(vec![("a".to_string(), ())]) },
            // The node already has the missing body's height covered.
            || async { Some(11) },
            |height| async move {
                if height == 11 {
                    Some("orphan-canonical".to_string())
                } else {
                    None
                }
            },
            |delay| tokio::time::sleep(delay),
        )
        .await;

        match result {
            Err(reason) => {
                assert!(reason.contains("orphaned"), "{reason}");
                assert!(reason.contains('b'), "{reason}");
                assert!(reason.contains("11"), "{reason}");
                assert!(reason.contains("orphan-canonical"), "{reason}");
            }
            Ok(_) => panic!("expected the batch to give up as orphaned"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn node_behind_for_a_while_then_serving_the_block_succeeds() {
        let requested_ids = ids(&["a", "b"]);
        let requested_heights = vec![10, 11];
        let calls = std::cell::Cell::new(0u32);

        let result = fetch_batch_with_retry(
            &requested_ids,
            &requested_heights,
            || {
                let ready = calls.get() >= 5;
                async move {
                    if ready {
                        Ok::<_, String>(vec![("a".to_string(), ()), ("b".to_string(), ())])
                    } else {
                        Ok::<_, String>(vec![("a".to_string(), ())])
                    }
                }
            },
            || {
                let n = calls.get();
                calls.set(n + 1);
                // Behind (height 10 < missing height 11) for the first three
                // checks, then caught up -- but the block only actually
                // shows up a couple of retries after that.
                async move { if n < 3 { Some(10) } else { Some(11) } }
            },
            |_height| async { None },
            |delay| tokio::time::sleep(delay),
        )
        .await;

        assert!(result.is_ok(), "expected success, got {:?}", result);
    }

    #[tokio::test(start_paused = true)]
    async fn http_errors_retry_forever_and_never_give_up() {
        let requested_ids = ids(&["a"]);
        let requested_heights = vec![10];
        let calls = std::cell::Cell::new(0u32);
        // Comfortably more than the 600 one-second retries the old hard cap
        // on the error branch would have allowed.
        const ERROR_ATTEMPTS: u32 = 700;

        let result = fetch_batch_with_retry(
            &requested_ids,
            &requested_heights,
            || {
                let n = calls.get();
                calls.set(n + 1);
                async move {
                    if n < ERROR_ATTEMPTS {
                        Err::<Vec<(String, ())>, _>("connection refused".to_string())
                    } else {
                        Ok(vec![("a".to_string(), ())])
                    }
                }
            },
            || async { Some(10) },
            |_height| async { None },
            |delay| tokio::time::sleep(delay),
        )
        .await;

        assert!(result.is_ok(), "HTTP errors must never give up: {:?}", result);
    }
}
