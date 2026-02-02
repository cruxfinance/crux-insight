use std::time::{Duration, Instant};

use crate::types::work_object::WorkBlock;
use ergo_node_client::apis::{blocks_api, configuration::Configuration};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, instrument};

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
                let mut success = false;
                while !success {
                    let full_blocks_res = blocks_api::get_full_block_by_ids(
                        &node_conf,
                        work_blocks
                            .iter()
                            .map(|wb| wb.header.clone().unwrap().id)
                            .collect(),
                    )
                    .await;
                    match full_blocks_res {
                        Ok(full_blocks) => {
                            let mut block_stream = async_std::stream::from_iter(full_blocks);
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
