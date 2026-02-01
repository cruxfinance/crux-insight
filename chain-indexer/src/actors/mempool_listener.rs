use ergo_node_client::apis::{configuration::Configuration, transactions_api};
use futures::StreamExt;
use tmq::{subscribe, Context};
use tokio::sync::mpsc::Sender;
use tracing::{debug, error, info, warn};

use crate::{settings::Settings, types::mempool_work::MempoolWork};

/// Listens to ZMQ "mempool" topic and fetches full transaction data
pub async fn mempool_listener(sender: Sender<MempoolWork>, node_conf: Configuration) {
    let settings = Settings::new().unwrap();

    info!("Starting mempool listener, connecting to ZMQ...");
    let mut socket = subscribe(&Context::new())
        .connect(&settings.ergo_node.zmq_url)
        .unwrap()
        .subscribe(b"mempool")
        .unwrap();

    info!("Mempool listener connected to ZMQ, waiting for transactions...");

    loop {
        match socket.next().await {
            Some(Ok(msg)) => {
                // Message format: ["mempool", "<tx_id>"]
                let parts: Vec<&str> = msg.iter().filter_map(|item| item.as_str()).collect();

                if parts.len() < 2 {
                    warn!("Received malformed mempool message: {:?}", parts);
                    continue;
                }

                let tx_id = parts[1];
                debug!("Received mempool transaction: {}", tx_id);

                // Fetch full transaction from node
                match transactions_api::get_unconfirmed_transaction_by_id(&node_conf, tx_id).await {
                    Ok(tx) => {
                        if let Err(e) = sender.send(MempoolWork::NewTransaction(tx)).await {
                            error!("Failed to send mempool transaction to inserter: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        // Transaction might have been confirmed or evicted between notification and fetch
                        debug!(
                            "Failed to fetch mempool transaction {}: {} (may have been confirmed)",
                            tx_id, e
                        );
                    }
                }
            }
            Some(Err(e)) => {
                error!("ZMQ mempool socket error: {}", e);
                break;
            }
            None => {
                error!("ZMQ mempool socket closed unexpectedly");
                break;
            }
        }
    }
}
