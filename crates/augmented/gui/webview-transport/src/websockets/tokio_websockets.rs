// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use crossbeam::atomic::AtomicCell;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Error::{ConnectionClosed, Protocol, Utf8};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};

type ConnectionId = u32;
type ConnectionSink = SplitSink<WebSocketStream<TcpStream>, Message>;
type ConnectionMap = Arc<Mutex<HashMap<ConnectionId, ConnectionSink>>>;

async fn handle_connection(
    peer: SocketAddr,
    mut ws_stream: SplitStream<WebSocketStream<TcpStream>>,
    input_sender: Sender<Message>,
) -> tungstenite::Result<()> {
    log::info!("New WebSocket connection: {:?}", peer);
    loop {
        let maybe_message = ws_stream.next().await;
        if maybe_message.is_none() {
            break;
        }

        let msg = maybe_message.unwrap()?;
        log::debug!("Received message {:?}", msg);
        if msg.is_text() || msg.is_binary() {
            if let Err(err) = input_sender.send(msg) {
                log::error!("Failed to broadcast message {}", err);
            }
        }
    }

    Ok(())
}

async fn connection_loop(
    peer: SocketAddr,
    input_sender: Sender<Message>,
    ws_stream: SplitStream<WebSocketStream<TcpStream>>,
) {
    if let Err(e) = handle_connection(peer, ws_stream, input_sender).await {
        match e {
            ConnectionClosed | Protocol(_) | Utf8 => (),
            err => log::error!("Error processing connection: {:?}", err),
        }
    }
}

pub fn create_transport_runtime() -> Runtime {
    log::info!("Creating tokio event-loop");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("ws-transport-tokio")
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    runtime
}

async fn run_websockets_accept_loop(
    listener: TcpListener,
    input_sender: Sender<Message>,
    current_id: AtomicCell<u32>,
    connections: ConnectionMap,
) {
    log::info!("Waiting for ws connections");
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        log::info!("Peer address: {}", peer);

        let connection_id = current_id.fetch_add(1);

        {
            let connections = connections.clone();
            let input_sender = input_sender.clone();

            tokio::spawn(async move {
                let ws_stream = accept_async(stream).await.expect("Failed to accept");
                let (ws_write, ws_read) = ws_stream.split();

                {
                    let mut connections = connections.lock().await;
                    connections.insert(connection_id, ws_write);
                }

                connection_loop(peer, input_sender, ws_read).await;

                {
                    log::info!("Cleaning-up connection {}", connection_id);
                    let mut connections = connections.lock().await;
                    connections.remove(&connection_id);
                    log::info!("Cleaned-up connection {}", connection_id);
                }
            });
        }
    }
}

pub struct ServerHandle {
    loop_handle: JoinHandle<()>,
    input_sender: Sender<Message>,
    #[allow(dead_code)]
    input_broadcast: Receiver<Message>,
    connections: ConnectionMap,
}

impl ServerHandle {
    #[allow(dead_code)]
    async fn get_num_connected_clients(&self) -> usize {
        (*self.connections.lock().await).len()
    }

    pub fn messages(&mut self) -> Receiver<Message> {
        self.input_sender.subscribe()
    }

    pub fn abort(&self) {
        self.loop_handle.abort();
    }

    pub async fn send(&self, message: String) {
        let mut connections = self.connections.lock().await;
        log::debug!(
            "Flushing text message to {} connections : {}",
            connections.len(),
            message
        );
        for (_, connection) in connections.iter_mut() {
            if let Err(err) = connection.send(Message::Text(message.clone())).await {
                log::error!("Failed to send message {}", err);
            }
        }
    }
}

pub struct ServerOptions<'a> {
    addr: &'a str,
}

impl<'a> ServerOptions<'a> {
    pub fn new(addr: &'a str) -> Self {
        ServerOptions { addr }
    }
}

pub async fn run_websockets_transport_async(
    options: ServerOptions<'_>,
) -> tokio::io::Result<ServerHandle> {
    let listener: TcpListener = TcpListener::bind(options.addr).await?;
    log::info!("Listening on: {}", options.addr);
    let (input_sender, input_broadcast) = tokio::sync::broadcast::channel(1);

    let connections = Arc::new(Mutex::new(HashMap::new()));
    let current_id = AtomicCell::<u32>::new(0);

    let loop_handle = tokio::spawn(run_websockets_accept_loop(
        listener,
        input_sender.clone(),
        current_id,
        connections.clone(),
    ));

    Ok(ServerHandle {
        loop_handle,
        input_sender,
        input_broadcast,
        connections,
    })
}

#[cfg(test)]
mod test {
    use log::LevelFilter;
    use tokio::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_start_websockets_server() {
        setup_test_logger();
        let url = "127.0.0.1:9510";
        let mut transport = run_websockets_transport_async(ServerOptions::new(url))
            .await
            .unwrap();

        let (mut ws_stream, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:9510")
            .await
            .unwrap();
        let app_started = "message";
        ws_stream
            .send(Message::Text(serde_json::to_string(&app_started).unwrap()))
            .await
            .unwrap();

        let msg = transport.input_broadcast.recv().await.unwrap();
        assert!(msg.is_text());
        assert_eq!(msg.into_text().unwrap(), "\"message\"");
        assert_eq!(transport.get_num_connected_clients().await, 1);
        ws_stream.close(None).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(transport.get_num_connected_clients().await, 0);
        transport.loop_handle.abort();
    }

    fn setup_test_logger() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Info)
            .try_init();
    }
}
