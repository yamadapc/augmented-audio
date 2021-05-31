use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;

use crate::editor::protocol::{
    ClientMessage, ClientMessageInner, MessageWrapper, ServerMessage, ServerMessageInner,
};
use crate::editor::tokio_websockets::{
    run_websockets_transport_async, ServerHandle, ServerOptions,
};

#[async_trait]
pub trait WebviewTransport<ServerMessage, ClientMessage> {
    async fn start(&mut self) -> Result<(), Box<dyn Error>>;
    async fn stop(self) -> Result<(), Box<dyn Error>>;
    fn messages(&self) -> Receiver<ClientMessage>;
    fn output_messages(&self) -> Sender<ServerMessage>;
    async fn send(&self, message: ServerMessage);
}

pub struct WebSocketsTransport {
    addr: String,
    server_handle: Option<Arc<Mutex<ServerHandle>>>,
    broadcast_channel: (Sender<ClientMessage>, Receiver<ClientMessage>),
    output_channel: (Sender<ServerMessage>, Receiver<ServerMessage>),
}

#[async_trait]
impl WebviewTransport<MessageWrapper<ServerMessageInner>, MessageWrapper<ClientMessageInner>>
    for WebSocketsTransport
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut server_handle =
            run_websockets_transport_async(ServerOptions::new(&self.addr)).await?;

        let mut messages = server_handle.messages();
        let sender = self.broadcast_channel.0.clone();
        tokio::spawn(async move {
            loop {
                match messages.recv().await {
                    Ok(tungstenite::Message::Text(msg_str)) => {
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
                    _ => {}
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

    fn messages(&self) -> Receiver<MessageWrapper<ClientMessageInner>> {
        let (sender, _) = &self.broadcast_channel;
        sender.subscribe()
    }

    fn output_messages(&self) -> Sender<MessageWrapper<ServerMessageInner>> {
        self.output_channel.0.clone()
    }

    async fn send(&self, message: MessageWrapper<ServerMessageInner>) {
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

impl WebSocketsTransport {
    pub fn new(addr: &str) -> Self {
        WebSocketsTransport {
            addr: String::from(addr),
            server_handle: None,
            broadcast_channel: tokio::sync::broadcast::channel(1),
            output_channel: tokio::sync::broadcast::channel(1),
        }
    }
}
