use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;

pub use tokio_websockets::create_transport_runtime;
use tokio_websockets::run_websockets_transport_async;
use tokio_websockets::ServerHandle;
use tokio_websockets::ServerOptions;

use crate::transport::WebviewTransport;
use tokio::task::JoinHandle;

mod tokio_websockets;

type ServerRef = Arc<Mutex<ServerHandle>>;

pub struct WebSocketsTransport<ServerMessage, ClientMessage> {
    addr: String,
    server_handle: Option<ServerRef>,
    broadcast_channel: (Sender<ClientMessage>, Receiver<ClientMessage>),
    output_channel: (Sender<ServerMessage>, Receiver<ServerMessage>),
}

impl<ServerMessage, ClientMessage> WebSocketsTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug,
    ClientMessage: DeserializeOwned + Send + Clone + Debug,
{
    pub fn new(addr: &str) -> Self {
        WebSocketsTransport {
            addr: String::from(addr),
            server_handle: None,
            broadcast_channel: tokio::sync::broadcast::channel(1),
            output_channel: tokio::sync::broadcast::channel(1),
        }
    }
}

#[async_trait]
impl<ServerMessage, ClientMessage> WebviewTransport<ServerMessage, ClientMessage>
    for WebSocketsTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("Starting websockets transport");
        let mut server_handle =
            run_websockets_transport_async(ServerOptions::new(&self.addr)).await?;
        let messages = server_handle.messages();
        start_client_broadcast_loop(messages, self.broadcast_channel.0.clone());

        let server_handle = Arc::new(Mutex::new(server_handle));
        self.start_server_broadcast_loop(server_handle.clone());

        self.server_handle = Some(server_handle);
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &self.server_handle {
            handle.lock().await.abort();
        }
        Ok(())
    }

    fn messages(&self) -> Receiver<ClientMessage> {
        let (sender, _) = &self.broadcast_channel;
        sender.subscribe()
    }

    fn output_messages(&self) -> Sender<ServerMessage> {
        self.output_channel.0.clone()
    }

    async fn send(&self, message: ServerMessage) {
        if let Some(handle) = &self.server_handle {
            let result = serde_json::to_string(&message);
            match result {
                Ok(str) => {
                    let handle = handle.lock().await;
                    handle.send(str).await
                }
                Err(err) => {
                    log::error!("Failed to send message {}", err);
                }
            }
        }
    }
}

impl<ServerMessage, ClientMessage> WebSocketsTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    /// Start task to send output messages from
    fn start_server_broadcast_loop(&mut self, server_handle: ServerRef) -> JoinHandle<()> {
        let (output_sender, _) = &self.output_channel;
        let output_receiver = output_sender.subscribe();
        tokio::spawn(forward_messages(output_receiver, server_handle))
    }
}

fn start_client_broadcast_loop<ClientMessage>(
    mut messages: Receiver<tungstenite::Message>,
    sender: Sender<ClientMessage>,
) -> JoinHandle<()>
where
    ClientMessage: DeserializeOwned + Debug + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            if let Ok(tungstenite::Message::Text(msg_str)) = messages.recv().await {
                if let Ok(message) = serde_json::from_str(&msg_str) {
                    let message: ClientMessage = message;
                    log::debug!("WebSocketsTransport parsed message - {:?}", message);
                    match sender.send(message) {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Failed to forward message for handling {}", err)
                        }
                    }
                }
            }
        }
    })
}

/// Read messages from the 'server output channel' and push them onto the WebSockets server ref.
async fn forward_messages<ServerMessage>(
    mut output_receiver: Receiver<ServerMessage>,
    server_handle: ServerRef,
) where
    ServerMessage: Serialize + Clone,
{
    loop {
        let message = output_receiver.recv().await.unwrap();
        let message = serde_json::to_string(&message);
        match message {
            Ok(message) => {
                let handle = server_handle.lock().await;
                handle.send(message).await;
            }
            Err(err) => {
                log::error!("Failed to serialize message {}", err)
            }
        }
    }
}
