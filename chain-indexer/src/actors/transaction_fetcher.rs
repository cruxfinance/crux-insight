use std::time::Duration;

use ergo_node_client::apis::{blocks_api, configuration::Configuration};
use tokio::sync::mpsc::Sender;
use tracing::{debug, error, instrument};

use crate::types::work_object::WorkBlock;

#[instrument]
pub async fn fetch_transactions(
    node_conf: &Configuration,
    receiver: async_channel::Receiver<WorkBlock>,
    sender: Sender<WorkBlock>,
) {
    loop {
        let work_block_res = receiver.recv().await;
        match work_block_res {
            Ok(work_block) => match work_block.header {
                Some(header) => {
                    if work_block.zmq_mode {
                        debug!("fetch_transactions: {}", header.id);
                    }
                    if header.height % 1000 == 0 {
                        debug!(
                            "Transaction channel size: {}",
                            (&sender.max_capacity() - &sender.capacity())
                        );
                    }
                    let mut success = false;
                    while !success {
                        let transactions_res = blocks_api::get_block_transactions_by_id(
                            &node_conf,
                            header.id.as_str(),
                        )
                        .await;
                        match transactions_res {
                            Ok(transactions) => {
                                match sender
                                    .send(WorkBlock {
                                        zmq_mode: work_block.zmq_mode,
                                        header: Some(header.clone()),
                                        transactions: Some(transactions.transactions.clone()),
                                        rollback_height: work_block.rollback_height,
                                    })
                                    .await
                                {
                                    Ok(_) => (),
                                    Err(e) => panic!("{}", e),
                                }

                                success = true;
                            }
                            Err(e) => {
                                error!("{}", e);
                                tokio::time::sleep(Duration::new(1, 0)).await;
                            }
                        }
                    }
                }
                None => match sender
                    .send(WorkBlock {
                        zmq_mode: work_block.zmq_mode,
                        header: None,
                        transactions: None,
                        rollback_height: work_block.rollback_height,
                    })
                    .await
                {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                },
            },
            Err(e) => panic!("{}", e),
        }
    }
}
