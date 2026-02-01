use futures::SinkExt;
use tmq::{publish, Context};
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

/// A ZMQ publish message: (topic, data)
pub type ZmqMessage = (String, String);

/// Dedicated ZMQ publisher actor.
/// Owns the publish socket and drains messages from a channel.
pub async fn zmq_publisher(mut receiver: Receiver<ZmqMessage>, bind_address: String) {
    let mut socket = publish(&Context::new())
        .bind(&bind_address)
        .expect("Failed to bind ZMQ publish socket");

    info!("ZMQ publisher bound to {}", bind_address);

    while let Some((topic, data)) = receiver.recv().await {
        if let Err(e) = socket.send(vec![topic.as_str(), data.as_str()]).await {
            error!("Failed to send ZMQ message: {}", e);
        }
    }
}
