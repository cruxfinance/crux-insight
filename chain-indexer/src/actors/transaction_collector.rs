use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};
use tracing::instrument;

use crate::types::work_object::WorkBlock;

#[instrument]
pub async fn collect_transactions(
    starting_height: i32,
    mut receiver: Receiver<WorkBlock>,
    sender: Sender<WorkBlock>,
) {
    let mut collected_transactions = HashMap::new();
    let mut current_height = starting_height;
    while let Some(work_block) = receiver.recv().await {
        match work_block.rollback_height {
            None => {
                collected_transactions.insert(
                    work_block.header.clone().unwrap().height,
                    work_block.clone(),
                );
                while collected_transactions.contains_key(&current_height) {
                    let next_block = collected_transactions.remove(&current_height).unwrap();
                    match sender.send(next_block.clone()).await {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    }
                    current_height += 1;
                }
            }
            Some(rollback_height) => {
                current_height = rollback_height;
                match sender.send(work_block.clone()).await {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}
