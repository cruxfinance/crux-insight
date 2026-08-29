use std::time::Duration;

use ergo_node_client::apis::{configuration::Configuration, transactions_api};
use futures::StreamExt;
use tmq::{subscribe, subscribe::Subscribe, Context};
use tokio::sync::mpsc::Sender;
use tracing::{debug, error, info, warn};

use crate::{settings::Settings, types::mempool_work::MempoolWork};

const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Connects to the node's ZMQ publisher and subscribes to the `mempool` topic,
/// retrying with exponential backoff instead of panicking.
async fn connect_mempool_socket(zmq_url: &str) -> Subscribe {
    let mut delay = Duration::from_secs(1);
    loop {
        match subscribe(&Context::new())
            .connect(zmq_url)
            .and_then(|s| s.subscribe(b"mempool"))
        {
            Ok(socket) => {
                info!("Mempool listener connected to ZMQ at {}", zmq_url);
                return socket;
            }
            Err(e) => {
                error!(
                    "Failed to connect mempool ZMQ subscriber to {}: {}; retrying in {:?}",
                    zmq_url, e, delay
                );
                tokio::time::sleep(delay).await;
                delay = (delay * 2).min(MAX_BACKOFF);
            }
        }
    }
}

/// Listens to ZMQ "mempool" topic and fetches full transaction data.
/// Reconnects on socket errors; only returns when the inserter is gone.
pub async fn mempool_listener(sender: Sender<MempoolWork>, node_conf: Configuration) {
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load settings for mempool listener: {}", e);
            return;
        }
    };

    info!("Starting mempool listener, connecting to ZMQ...");
    let mut socket = connect_mempool_socket(&settings.ergo_node.zmq_url).await;
    let mut backoff = Duration::from_secs(1);

    loop {
        match socket.next().await {
            Some(Ok(msg)) => {
                backoff = Duration::from_secs(1);
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
                            return;
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
                error!(
                    "ZMQ mempool socket error: {}; reconnecting in {:?}",
                    e, backoff
                );
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                socket = connect_mempool_socket(&settings.ergo_node.zmq_url).await;
            }
            None => {
                error!(
                    "ZMQ mempool socket stream ended; reconnecting in {:?}",
                    backoff
                );
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
                socket = connect_mempool_socket(&settings.ergo_node.zmq_url).await;
            }
        }
    }
}
