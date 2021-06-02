use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;

use tokio_websockets::run_websockets_transport_async;
use tokio_websockets::ServerHandle;
use tokio_websockets::ServerOptions;

pub mod tokio_websockets;

#[async_trait]
pub trait WebviewTransport<ServerMessage, ClientMessage> {
    async fn start(&mut self) -> Result<(), Box<dyn Error>>;
    async fn stop(self) -> Result<(), Box<dyn Error>>;
    fn messages(&self) -> Receiver<ClientMessage>;
    fn output_messages(&self) -> Sender<ServerMessage>;
    async fn send(&self, message: ServerMessage);
}

pub struct WebSocketsTransport<ServerMessage, ClientMessage> {
    addr: String,
    server_handle: Option<Arc<Mutex<ServerHandle>>>,
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
    ServerMessage: Serialize + Send + Clone + 'static,
    ClientMessage: DeserializeOwned + Send + Debug + 'static,
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut server_handle =
            run_websockets_transport_async(ServerOptions::new(&self.addr)).await?;

        let mut messages = server_handle.messages();
        let sender = self.broadcast_channel.0.clone();
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
        });

        let server_handle = Arc::new(Mutex::new(server_handle));
        {
            let server_handle = server_handle.clone();

            let (output_sender, _) = &self.output_channel;
            let mut output_receiver = output_sender.subscribe();
            tokio::spawn(async move {
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
            });
        }

        self.server_handle = Some(server_handle);
        Ok(())
    }

    async fn stop(self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = self.server_handle {
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
