use std::time::{Duration, Instant};

use crate::types::work_object::WorkBlock;
use ergo_node_client::apis::{blocks_api, configuration::Configuration};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, instrument, warn};

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
                let mut success = false;
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
                                warn!(
                                    "get_full_block_by_ids returned an incomplete batch; missing ids: {:?}; retrying",
                                    missing
                                );
                                tokio::time::sleep(Duration::new(1, 0)).await;
                                continue;
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
                            error!("{}", e);
                            tokio::time::sleep(Duration::new(1, 0)).await;
                        }
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
}
