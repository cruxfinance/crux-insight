use std::time::{Duration, Instant};

use crate::actors::header_fetcher::node_best_header_id;
use crate::types::work_object::WorkBlock;
use ergo_node_client::apis::{blocks_api, configuration::Configuration};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, instrument, warn};

/// Grace period of persistent misses before we start asking the node whether
/// a missing header was orphaned.
const GRACE_PERIOD: Duration = Duration::from_secs(30);
/// Hard cap on how long we'll keep retrying a single batch, orphan or not.
const HARD_CAP: Duration = Duration::from_secs(10 * 60);
/// Ceiling on the exponential retry backoff.
const MAX_RETRY_BACKOFF: Duration = Duration::from_secs(10);

/// Requested header ids that are missing from the response (each checked for
/// exactly one match). The node's `/blocks/headerIds` endpoint silently omits
/// any id it can't serve a full block for, so a response can be a strict
/// subset of what was requested without ever returning an error.
fn missing_ids(requested: &[String], returned_ids: &[String]) -> Vec<String> {
    requested
        .iter()
        .filter(|id| returned_ids.iter().filter(|r| r == id).count() != 1)
        .cloned()
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
/// with the node's current canonical header id at that height. `canonical`
/// is `None` when it hasn't been looked up yet (still within the grace
/// period) or the lookup itself failed.
#[derive(Debug, Clone)]
struct MissingHeader {
    id: String,
    height: i32,
    canonical: Option<String>,
}

/// Retry/give-up policy for an incomplete `/blocks/headerIds` batch:
/// exponential backoff (1s, 2s, 4s, ... capped at 10s) for the first 30s of
/// persistent misses; past that, give up immediately if the node's canonical
/// header at a missing id's height differs from what was requested (the
/// header was orphaned by a fork); otherwise keep retrying until a 10-minute
/// hard cap, then give up too.
fn incomplete_batch_action(
    elapsed: Duration,
    attempts: u32,
    missing: &[MissingHeader],
) -> BatchAction {
    if elapsed >= HARD_CAP {
        return BatchAction::GiveUp(format!(
            "get_full_block_by_ids still incomplete after {:?} ({} attempts); giving up",
            elapsed, attempts
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

    let backoff_secs = 1u64.checked_shl(attempts).unwrap_or(u64::MAX);
    BatchAction::Retry(Duration::from_secs(backoff_secs).min(MAX_RETRY_BACKOFF))
}

/// Looks up the node's canonical header id for each missing id, at the
/// height it was requested at. Skipped (all `None`) while still within the
/// grace period, to avoid hammering the node with extra REST calls on every
/// 1s retry.
async fn resolve_missing_headers(
    node_conf: &Configuration,
    missing: &[String],
    requested_ids: &[String],
    requested_heights: &[i32],
    check_canonical: bool,
) -> Vec<MissingHeader> {
    let mut out = Vec::with_capacity(missing.len());
    for id in missing {
        let height = requested_ids
            .iter()
            .position(|r| r == id)
            .map(|i| requested_heights[i])
            .unwrap_or(-1);
        let canonical = if check_canonical {
            node_best_header_id(node_conf, height).await
        } else {
            None
        };
        out.push(MissingHeader {
            id: id.clone(),
            height,
            canonical,
        });
    }
    out
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
                let fetch_start = Instant::now();
                let requested_ids: Vec<String> = work_blocks
                    .iter()
                    .map(|wb| wb.header.clone().unwrap().id)
                    .collect();
                let requested_heights: Vec<i32> = work_blocks
                    .iter()
                    .map(|wb| wb.header.clone().unwrap().height)
                    .collect();
                let mut success = false;
                let mut give_up = false;
                let mut attempts: u32 = 0;
                while !success {
                    let full_blocks_res =
                        blocks_api::get_full_block_by_ids(node_conf, requested_ids.clone()).await;
                    match full_blocks_res {
                        Ok(full_blocks) => {
                            let returned_ids: Vec<String> = full_blocks
                                .iter()
                                .map(|b| b.header.id.clone())
                                .collect();
                            let missing = missing_ids(&requested_ids, &returned_ids);
                            if !missing.is_empty() {
                                let elapsed = fetch_start.elapsed();
                                warn!(
                                    "get_full_block_by_ids returned an incomplete batch (attempt {}, elapsed {:?}); missing ids: {:?}",
                                    attempts + 1,
                                    elapsed,
                                    missing
                                );
                                let missing_headers = resolve_missing_headers(
                                    node_conf,
                                    &missing,
                                    &requested_ids,
                                    &requested_heights,
                                    elapsed >= GRACE_PERIOD,
                                )
                                .await;
                                match incomplete_batch_action(elapsed, attempts, &missing_headers)
                                {
                                    BatchAction::Retry(delay) => {
                                        tokio::time::sleep(delay).await;
                                        attempts += 1;
                                        continue;
                                    }
                                    BatchAction::GiveUp(reason) => {
                                        error!("{}", reason);
                                        give_up = true;
                                        break;
                                    }
                                }
                            }

                            // Build the outgoing sequence in requested order,
                            // looking up each block by header id rather than
                            // trusting the response order.
                            let ordered_blocks: Vec<_> = requested_ids
                                .iter()
                                .map(|id| {
                                    full_blocks
                                        .iter()
                                        .find(|b| &b.header.id == id)
                                        .expect("id validated as present by missing_ids")
                                        .clone()
                                })
                                .collect();

                            let mut block_stream = async_std::stream::from_iter(ordered_blocks);
                            while let Some(block) = block_stream.next().await {
                                match sender
                                    .send(WorkBlock {
                                        zmq_mode: first_work_block.zmq_mode,
                                        header: Some(*block.header.clone()),
                                        transactions: Some(
                                            block.block_transactions.transactions.clone(),
                                        ),
                                        rollback_height: None,
                                    })
                                    .await
                                {
                                    Ok(_) => (),
                                    Err(e) => panic!("{}", e),
                                }
                            }

                            success = true;
                        }
                        Err(e) => {
                            let elapsed = fetch_start.elapsed();
                            error!("{}", e);
                            if elapsed >= HARD_CAP {
                                error!(
                                    "get_full_block_by_ids still failing after {:?} ({} attempts); giving up: {}",
                                    elapsed,
                                    attempts + 1,
                                    e
                                );
                                give_up = true;
                                break;
                            }
                            tokio::time::sleep(Duration::new(1, 0)).await;
                            attempts += 1;
                        }
                    }
                }
                if give_up {
                    return;
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
    fn complete_response_has_no_missing_ids() {
        let requested = ids(&["a", "b", "c"]);
        let returned = ids(&["a", "b", "c"]);
        assert_eq!(missing_ids(&requested, &returned), Vec::<String>::new());
    }

    #[test]
    fn missing_one_id_is_reported() {
        let requested = ids(&["a", "b", "c"]);
        let returned = ids(&["a", "c"]);
        assert_eq!(missing_ids(&requested, &returned), ids(&["b"]));
    }

    #[test]
    fn duplicate_id_in_response_is_reported() {
        // "a" comes back twice and "b" not at all; a returned id is not
        // "exactly once" so it is flagged same as an outright miss.
        let requested = ids(&["a", "b"]);
        let returned = ids(&["a", "a"]);
        assert_eq!(missing_ids(&requested, &returned), ids(&["a", "b"]));
    }

    #[test]
    fn empty_response_reports_everything_missing() {
        let requested = ids(&["a", "b", "c"]);
        let returned: Vec<String> = Vec::new();
        assert_eq!(missing_ids(&requested, &returned), ids(&["a", "b", "c"]));
    }

    fn missing_header(id: &str, height: i32, canonical: Option<&str>) -> MissingHeader {
        MissingHeader {
            id: id.to_string(),
            height,
            canonical: canonical.map(|s| s.to_string()),
        }
    }

    #[test]
    fn retries_with_growing_backoff_before_grace_period() {
        let missing = vec![missing_header("a", 100, None)];
        let elapsed = Duration::from_secs(5);
        assert_eq!(
            incomplete_batch_action(elapsed, 0, &missing),
            BatchAction::Retry(Duration::from_secs(1))
        );
        assert_eq!(
            incomplete_batch_action(elapsed, 1, &missing),
            BatchAction::Retry(Duration::from_secs(2))
        );
        assert_eq!(
            incomplete_batch_action(elapsed, 2, &missing),
            BatchAction::Retry(Duration::from_secs(4))
        );
        assert_eq!(
            incomplete_batch_action(elapsed, 3, &missing),
            BatchAction::Retry(Duration::from_secs(8))
        );
        // Backoff caps at 10s from here on, well before the grace period.
        assert_eq!(
            incomplete_batch_action(elapsed, 4, &missing),
            BatchAction::Retry(Duration::from_secs(10))
        );
        assert_eq!(
            incomplete_batch_action(elapsed, 10, &missing),
            BatchAction::Retry(Duration::from_secs(10))
        );
    }

    #[test]
    fn gives_up_when_canonical_id_differs_after_grace_period() {
        let missing = vec![missing_header("ccd962c0", 1864378, Some("other-canonical-id"))];
        let elapsed = GRACE_PERIOD + Duration::from_secs(1);
        match incomplete_batch_action(elapsed, 5, &missing) {
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
        let elapsed = GRACE_PERIOD + Duration::from_secs(60);
        assert_eq!(
            incomplete_batch_action(elapsed, 2, &missing),
            BatchAction::Retry(Duration::from_secs(4))
        );
    }

    #[test]
    fn still_retries_when_canonical_id_not_yet_known_and_under_hard_cap() {
        // Canonical lookup itself failed (or wasn't attempted); we can't
        // confirm an orphan, so keep retrying rather than giving up.
        let missing = vec![missing_header("a", 100, None)];
        let elapsed = GRACE_PERIOD + Duration::from_secs(60);
        assert_eq!(
            incomplete_batch_action(elapsed, 0, &missing),
            BatchAction::Retry(Duration::from_secs(1))
        );
    }

    #[test]
    fn gives_up_when_over_hard_cap_even_if_canonical_id_matches() {
        let missing = vec![missing_header("a", 100, Some("a"))];
        let elapsed = HARD_CAP + Duration::from_secs(1);
        match incomplete_batch_action(elapsed, 50, &missing) {
            BatchAction::GiveUp(reason) => assert!(reason.contains("giving up")),
            other => panic!("expected GiveUp, got {:?}", other),
        }
    }
}
