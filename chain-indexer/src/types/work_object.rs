use ergo_node_client::models::{BlockHeader, ErgoTransaction};

pub struct WorkBlock {
    pub zmq_mode: bool,
    pub header: Option<BlockHeader>,
    pub transactions: Option<Vec<ErgoTransaction>>,
    pub rollback_height: Option<i32>,
}

impl WorkBlock {
    pub fn clone(&self) -> WorkBlock {
        WorkBlock {
            zmq_mode: self.zmq_mode.clone(),
            header: self.header.clone(),
            transactions: self.transactions.clone(),
            rollback_height: self.rollback_height.clone(),
        }
    }
}
