use ergo_node_client::models::ErgoTransaction;

/// Work items for the mempool inserter actor
#[derive(Debug)]
pub enum MempoolWork {
    /// New transaction arrived in mempool
    NewTransaction(ErgoTransaction),
    /// Block confirmed - need to clean up mempool
    BlockConfirmed {
        block_height: i32,
        confirmed_tx_ids: Vec<String>,
    },
    /// Full mempool resync requested
    FullResync,
}
