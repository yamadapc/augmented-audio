use crate::editor::protocol::{ClientMessageInner, MessageWrapper, ServerMessageInner};
use crate::editor::tokio_websockets::{
    block_on_websockets_main, run_websockets_transport_async, ServerHandle, ServerOptions,
};
use async_trait::async_trait;
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Error::{ConnectionClosed, Protocol, Utf8};
use tungstenite::WebSocket;

#[async_trait]
pub trait WebviewTransport<ServerMessage, ClientMessage> {
    async fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(self) -> Result<(), Box<dyn Error>>;
}

pub struct WebSocketsTransport {
    addr: String,
    server_handle: Option<ServerHandle>,
}

#[async_trait]
impl WebviewTransport<MessageWrapper<ServerMessageInner>, MessageWrapper<ClientMessageInner>>
    for WebSocketsTransport
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let server_handle = run_websockets_transport_async(ServerOptions::new(&self.addr)).await?;
        self.server_handle = Some(server_handle);

        Ok(())
    }

    fn stop(self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = self.server_handle {
            handle.abort();
        }
        Ok(())
    }
}

impl WebSocketsTransport {
    pub fn new(addr: &str) -> Self {
        WebSocketsTransport {
            addr: String::from(addr),
            server_handle: None,
        }
    }
}
