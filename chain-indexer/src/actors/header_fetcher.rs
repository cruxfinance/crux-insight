use std::{
    cmp::min,
    future::Future,
    time::{Duration, Instant},
};

use ergo_node_client::apis::{blocks_api, configuration::Configuration, info_api};
use ergo_node_client::models::BlockHeader;
use futures::StreamExt;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tmq::{subscribe, subscribe::Subscribe, Context};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::database::CIDatabase;
use crate::entities::blocks;
use crate::settings::Settings;
use crate::types::work_object::WorkBlock;

/// Number of headers requested per `/blocks/chainSlice` call.
const SLICE_LIMIT: i32 = 10;
/// Upper bound on how far below a mismatching header we search for the fork point.
const MAX_FORK_WALK: i32 = 1000;
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Header id stored in the DB at `height`, or `None` if missing or on DB error.
async fn header_id_in_db(db: &DatabaseConnection, height: i32) -> Option<String> {
    match blocks::Entity::find()
        .filter(blocks::Column::Height.eq(height))
        .one(db)
        .await
    {
        Ok(Some(block)) => Some(block.header_id),
        Ok(None) => {
            warn!("No block at height {} in database", height);
            None
        }
        Err(e) => {
            error!("Failed to look up header at height {}: {}", height, e);
            None
        }
    }
}

async fn get_header_id(settings: &Settings, height: i32) -> Option<String> {
    debug!("Looking for header with height: {}", height);
    let db = CIDatabase {
        settings: settings.to_owned(),
    }
    .connect()
    .await;
    let res = header_id_in_db(&db, height).await;
    let _ = db.close().await;
    res
}

/// Best-chain header id the node has at `height` (first element of
/// `/blocks/at/{height}`), with a few retries for transient REST errors.
pub(crate) async fn node_best_header_id(node_conf: &Configuration, height: i32) -> Option<String> {
    for attempt in 1..=3 {
        match blocks_api::get_full_block_at(node_conf, height).await {
            Ok(ids) => return ids.into_iter().next(),
            Err(e) => {
                warn!(
                    "get_full_block_at({}) failed (attempt {}/3): {}",
                    height, attempt, e
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    None
}

/// Walks down from `start` until the node's best header id and the DB's header
/// id agree, returning that height. Gives up after `max_walk` heights.
/// Missing ids on either side count as a mismatch.
async fn find_fork_height<NF, NFut, DF, DFut>(
    start: i32,
    max_walk: i32,
    mut node_id_at: NF,
    mut db_id_at: DF,
) -> Option<i32>
where
    NF: FnMut(i32) -> NFut,
    NFut: Future<Output = Option<String>>,
    DF: FnMut(i32) -> DFut,
    DFut: Future<Output = Option<String>>,
{
    let lowest = (start - max_walk).max(0);
    let mut height = start;
    while height >= lowest {
        let node_id = node_id_at(height).await;
        let db_id = db_id_at(height).await;
        if let (Some(n), Some(d)) = (node_id, db_id) {
            if n == d {
                return Some(height);
            }
        }
        height -= 1;
    }
    None
}

/// Cursor of the header fetcher: `offset` is the height of the last header
/// handed downstream (== the DB tip once inserted) and `last_header_id` its id.
struct FetchState {
    offset: i32,
    last_header_id: String,
}

fn work_block(header: BlockHeader, zmq_mode: bool) -> WorkBlock {
    WorkBlock {
        zmq_mode,
        header: Some(header),
        transactions: None,
        rollback_height: None,
    }
}

/// Outcome of a downstream send: `Err` means the receiver is gone, which is
/// fatal for this actor (the supervisor turns the return into a restart).
type SendResult = Result<(), ()>;

async fn send_work(sender: &Sender<Vec<WorkBlock>>, work: Vec<WorkBlock>) -> SendResult {
    if work.is_empty() {
        return Ok(());
    }
    sender.send(work).await.map_err(|e| {
        error!("Header receiver closed: {}", e);
    })
}

/// Parent mismatch at `header`: re-check once (the node's header chain can be
/// momentarily ahead of its full-block chain), then locate the real fork point
/// and emit a rollback to it. Replaces the old `height - 10` heuristic that
/// deleted 9–10 blocks for every 1-block fork.
async fn handle_fork(
    node_conf: &Configuration,
    settings: &Settings,
    state: &mut FetchState,
    sender: &Sender<Vec<WorkBlock>>,
    header: &BlockHeader,
    zmq_mode: bool,
) -> SendResult {
    warn!(
        "Parent mismatch at height {}: header {} has parent {}, expected {}; re-checking in 2s",
        header.height, header.id, header.parent_id, state.last_header_id
    );
    sleep(Duration::from_secs(2)).await;

    if let Ok(recheck) = blocks_api::get_chain_slice(
        node_conf,
        Some(state.offset),
        Some(state.offset + SLICE_LIMIT),
    )
    .await
    {
        if let Some(first) = recheck.iter().find(|h| h.id != state.last_header_id) {
            if first.parent_id == state.last_header_id {
                info!(
                    "Parent mismatch at height {} was transient; continuing",
                    header.height
                );
                return Ok(());
            }
        }
    }

    let db = CIDatabase {
        settings: settings.to_owned(),
    }
    .connect()
    .await;
    let node_conf_cl = node_conf.clone();
    let db_cl = db.clone();
    let fork_height = find_fork_height(
        header.height - 1,
        MAX_FORK_WALK,
        move |h| {
            let conf = node_conf_cl.clone();
            async move { node_best_header_id(&conf, h).await }
        },
        move |h| {
            let db = db_cl.clone();
            async move { header_id_in_db(&db, h).await }
        },
    )
    .await;
    let fork_header_id = match fork_height {
        Some(h) => header_id_in_db(&db, h).await,
        None => None,
    };
    let _ = db.close().await;

    match (fork_height, fork_header_id) {
        (Some(fork_height), Some(fork_header_id)) => {
            warn!(
                "Fork confirmed: header {} at height {} diverges from indexed chain; rolling back from {} to {} ({} blocks)",
                header.id,
                header.height,
                state.offset,
                fork_height,
                state.offset - fork_height
            );
            state.offset = fork_height;
            state.last_header_id = fork_header_id;
            send_work(
                sender,
                vec![WorkBlock {
                    zmq_mode,
                    header: None,
                    transactions: None,
                    rollback_height: Some(fork_height),
                }],
            )
            .await
        }
        _ => {
            error!(
                "Could not find fork point within {} blocks below height {}; giving up",
                MAX_FORK_WALK, header.height
            );
            Err(())
        }
    }
}

/// Hands a chain slice downstream. Contiguous headers are sent as one batch;
/// on a parent mismatch the batch so far is flushed, the fork is handled and
/// the remainder of the slice is dropped (the caller re-fetches from the new
/// cursor). Returns `Ok(true)` if the cursor moved (new headers or a rollback)
/// and `Ok(false)` if the slice contained nothing new.
///
/// The node's `/blocks/chainSlice` is exclusive of `fromHeight` — except when
/// `fromHeight` is the best height, in which case it returns the tip itself.
/// Headers we already hold are therefore skipped rather than treated as forks
/// (the old code turned every such response into a spurious rollback).
async fn process_slice(
    node_conf: &Configuration,
    settings: &Settings,
    state: &mut FetchState,
    sender: &Sender<Vec<WorkBlock>>,
    headers: Vec<BlockHeader>,
    zmq_mode: bool,
) -> Result<bool, ()> {
    let mut batch = Vec::with_capacity(headers.len());
    let mut advanced = false;
    for header in headers {
        if header.id == state.last_header_id {
            debug!("Skipping already-indexed header {} at {}", header.id, header.height);
            continue;
        }
        if header.parent_id == state.last_header_id {
            state.last_header_id = header.id.clone();
            state.offset = header.height;
            batch.push(work_block(header, zmq_mode));
            advanced = true;
        } else {
            send_work(sender, std::mem::take(&mut batch)).await?;
            handle_fork(node_conf, settings, state, sender, &header, zmq_mode).await?;
            return Ok(true);
        }
    }
    send_work(sender, batch).await?;
    Ok(advanced)
}

/// Pulls headers from the node via REST from `state.offset` until the node has
/// nothing newer. Used both when a `newBlock` event arrives and when the ZMQ
/// subscription goes quiet for too long.
async fn sync_via_rest(
    node_conf: &Configuration,
    settings: &Settings,
    state: &mut FetchState,
    sender: &Sender<Vec<WorkBlock>>,
) -> SendResult {
    loop {
        let headers = match blocks_api::get_chain_slice(
            node_conf,
            Some(state.offset),
            Some(state.offset + SLICE_LIMIT),
        )
        .await
        {
            Ok(headers) => headers,
            Err(e) => {
                error!("get_chain_slice from {} failed: {}", state.offset, e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
        if headers.is_empty() {
            return Ok(());
        }
        if !process_slice(node_conf, settings, state, sender, headers, true).await? {
            // Only already-indexed headers came back: caught up.
            return Ok(());
        }
    }
}

/// Connects to the node's ZMQ publisher and subscribes to `newBlock`, retrying
/// with exponential backoff instead of panicking.
async fn connect_new_block_socket(zmq_url: &str) -> Subscribe {
    let mut delay = Duration::from_secs(1);
    loop {
        match subscribe(&Context::new())
            .connect(zmq_url)
            .and_then(|s| s.subscribe(b"newBlock"))
        {
            Ok(socket) => {
                info!("Subscribed to newBlock ZMQ events at {}", zmq_url);
                return socket;
            }
            Err(e) => {
                error!(
                    "Failed to connect newBlock ZMQ subscriber to {}: {}; retrying in {:?}",
                    zmq_url, e, delay
                );
                sleep(delay).await;
                delay = (delay * 2).min(MAX_BACKOFF);
            }
        }
    }
}

pub async fn fetch_headers(
    node_conf: &Configuration,
    from: i32,
    to: i32,
    sender: Sender<Vec<WorkBlock>>,
) {
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load settings for header fetcher: {}", e);
            return;
        }
    };
    let Some(last_header_id) = get_header_id(&settings, from).await else {
        error!("No header in database at starting height {}", from);
        return;
    };
    let mut state = FetchState {
        offset: from,
        last_header_id,
    };

    // Phase 1: initial catch-up via REST up to the node height seen at startup.
    while state.offset < to {
        let fetch_start = Instant::now();
        let upper = min(state.offset + SLICE_LIMIT, to);
        match blocks_api::get_chain_slice(node_conf, Some(state.offset), Some(upper)).await {
            Ok(headers) => {
                if state.offset % 100 == 0 {
                    debug!(
                        "fetch_headers at {} took {:?}",
                        state.offset,
                        fetch_start.elapsed()
                    );
                }
                if headers.is_empty() {
                    warn!(
                        "Node returned no headers above {} (target {}); switching to zmq mode early",
                        state.offset, to
                    );
                    break;
                }
                match process_slice(node_conf, &settings, &mut state, &sender, headers, false)
                    .await
                {
                    Ok(true) => {}
                    Ok(false) => {
                        info!(
                            "Node returned nothing new above {} (target {}); switching to zmq mode",
                            state.offset, to
                        );
                        break;
                    }
                    Err(()) => return,
                }
            }
            Err(e) => {
                error!("{}", e);
                sleep(Duration::from_millis(1000)).await;
            }
        }
    }

    info!("Done fetching headers, switching to zmq mode");
    // Phase 2: follow the chain via ZMQ `newBlock` events, with a liveness
    // timeout so a silently dead subscription is detected and repaired.
    let zmq_timeout = Duration::from_secs(settings.chain_indexer.zmq_timeout_secs);
    let zmq_url = settings.ergo_node.zmq_url.clone();
    let mut socket = connect_new_block_socket(&zmq_url).await;
    let mut backoff = Duration::from_secs(1);

    loop {
        if sender.is_closed() {
            error!("No more receivers for headers; stopping header fetcher");
            return;
        }
        match tokio::time::timeout(zmq_timeout, socket.next()).await {
            Ok(Some(Ok(msg))) => {
                backoff = Duration::from_secs(1);
                let frames: Vec<&str> = msg
                    .iter()
                    .map(|item| item.as_str().unwrap_or("<non-utf8>"))
                    .collect();
                debug!("newBlock event: {:?}", frames);
                if sync_via_rest(node_conf, &settings, &mut state, &sender)
                    .await
                    .is_err()
                {
                    return;
                }
            }
            Ok(Some(Err(e))) => {
                error!(
                    "newBlock ZMQ socket error: {}; reconnecting in {:?}",
                    e, backoff
                );
                sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                socket = connect_new_block_socket(&zmq_url).await;
            }
            Ok(None) => {
                error!(
                    "newBlock ZMQ stream ended; reconnecting in {:?}",
                    backoff
                );
                sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                socket = connect_new_block_socket(&zmq_url).await;
            }
            Err(_elapsed) => {
                warn!(
                    "No newBlock ZMQ event for {}s; checking node height via REST",
                    zmq_timeout.as_secs()
                );
                match info_api::get_node_info(node_conf).await {
                    Ok(node_info) => {
                        let node_height = node_info.full_height.unwrap_or(0);
                        if node_height > state.offset {
                            warn!(
                                "ZMQ subscription appears dead: indexed height {} < node height {}; syncing via REST",
                                state.offset, node_height
                            );
                            if sync_via_rest(node_conf, &settings, &mut state, &sender)
                                .await
                                .is_err()
                            {
                                return;
                            }
                        } else {
                            info!(
                                "Chain idle: node height {} == indexed height {}",
                                node_height, state.offset
                            );
                        }
                    }
                    Err(e) => error!("Failed to query node info: {}", e),
                }
                // A silently dead socket is indistinguishable from an idle
                // chain, so always recreate it after a timeout.
                drop(socket);
                socket = connect_new_block_socket(&zmq_url).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(
        map: HashMap<i32, &'static str>,
    ) -> impl FnMut(i32) -> std::future::Ready<Option<String>> {
        move |h| std::future::ready(map.get(&h).map(|s| s.to_string()))
    }

    #[tokio::test]
    async fn fork_height_is_first_agreeing_height_below_start() {
        // DB has a, b, c at 1..3; node replaced 3 with c' and built 4 on it.
        let node = HashMap::from([(1, "a"), (2, "b"), (3, "c'"), (4, "d")]);
        let db = HashMap::from([(1, "a"), (2, "b"), (3, "c")]);
        let fork = find_fork_height(3, 100, lookup(node), lookup(db)).await;
        assert_eq!(fork, Some(2));
    }

    #[tokio::test]
    async fn one_block_fork_rolls_back_exactly_one_block() {
        // Only the tip (11) differs. The caller starts the walk at
        // `mismatch_height - 1`, so both entry points must land on 10.
        let node = HashMap::from([(10, "x"), (11, "y'")]);
        let db = HashMap::from([(10, "x"), (11, "y")]);
        assert_eq!(
            find_fork_height(11, 100, lookup(node.clone()), lookup(db.clone())).await,
            Some(10)
        );
        assert_eq!(
            find_fork_height(10, 100, lookup(node), lookup(db)).await,
            Some(10)
        );
    }

    #[tokio::test]
    async fn missing_ids_count_as_mismatch_and_walk_continues() {
        let node = HashMap::from([(5, "e"), (6, "f'")]);
        let db = HashMap::from([(5, "e")]); // height 6 missing in DB
        assert_eq!(find_fork_height(6, 100, lookup(node), lookup(db)).await, Some(5));
    }

    fn header(height: i32, id: &str, parent: &str) -> BlockHeader {
        BlockHeader {
            height,
            id: id.to_string(),
            parent_id: parent.to_string(),
            ..Default::default()
        }
    }

    fn test_env() -> (Configuration, Settings, Sender<Vec<WorkBlock>>, tokio::sync::mpsc::Receiver<Vec<WorkBlock>>) {
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        (Configuration::default(), test_settings(), tx, rx)
    }

    fn test_settings() -> Settings {
        serde_json::from_value(serde_json::json!({
            "database": {"user": "", "password": "", "host": "", "port": "", "db": ""},
            "ergo_node": {"url": "", "zmq_url": ""},
            "chain_indexer": {"tx_par": 1},
            "crux": {"pubsubport": 0}
        }))
        .expect("test settings")
    }

    #[tokio::test]
    async fn slice_containing_only_the_held_tip_is_not_progress() {
        let (conf, settings, tx, mut rx) = test_env();
        let mut state = FetchState {
            offset: 100,
            last_header_id: "tip".to_string(),
        };
        let advanced = process_slice(&conf, &settings, &mut state, &tx, vec![header(100, "tip", "p")], true)
            .await
            .unwrap();
        assert!(!advanced);
        assert_eq!(state.offset, 100);
        assert_eq!(state.last_header_id, "tip");
        assert!(rx.try_recv().is_err(), "nothing must be sent");
    }

    #[tokio::test]
    async fn held_tip_is_skipped_and_new_headers_are_batched() {
        let (conf, settings, tx, mut rx) = test_env();
        let mut state = FetchState {
            offset: 100,
            last_header_id: "tip".to_string(),
        };
        let headers = vec![
            header(100, "tip", "p"),
            header(101, "a", "tip"),
            header(102, "b", "a"),
        ];
        let advanced = process_slice(&conf, &settings, &mut state, &tx, headers, true)
            .await
            .unwrap();
        assert!(advanced);
        assert_eq!(state.offset, 102);
        assert_eq!(state.last_header_id, "b");
        let batch = rx.try_recv().expect("one batch sent");
        let heights: Vec<i32> = batch.iter().map(|w| w.header.as_ref().unwrap().height).collect();
        assert_eq!(heights, vec![101, 102]);
        assert!(batch.iter().all(|w| w.zmq_mode && w.rollback_height.is_none()));
    }

    #[tokio::test]
    async fn gives_up_after_max_walk() {
        let node = HashMap::from([(1, "n1"), (2, "n2"), (3, "n3")]);
        let db = HashMap::from([(1, "d1"), (2, "d2"), (3, "d3")]);
        assert_eq!(find_fork_height(3, 1, lookup(node), lookup(db)).await, None);
    }
}
