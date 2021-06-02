use std::error::Error;
use tokio::sync::broadcast::{Receiver, Sender};

/// Abstraction for messaging with a webview.
#[async_trait]
pub trait WebviewTransport<ServerMessage, ClientMessage> {
    /// Start transport and its tasks or threads
    async fn start(&mut self) -> Result<(), Box<dyn Error>>;
    /// Stop transport, abort all tasks or threads
    async fn stop(self) -> Result<(), Box<dyn Error>>;
    /// Messages from JavaScript
    fn messages(&self) -> Receiver<ClientMessage>;
    /// Messages into JavaScript
    fn output_messages(&self) -> Sender<ServerMessage>;
    /// Helper for sending a message into JavaScript
    async fn send(&self, message: ServerMessage);
}
