use crate::editor::protocol::ClientMessageInner;
use crossbeam::atomic::AtomicCell;
use futures_util::SinkExt;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinError;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Error::{ConnectionClosed, Protocol, Utf8};
use tokio_tungstenite::tungstenite::Message;

pub fn run_websockets_transport_main(addr: &str) {
    let runtime = create_transport_runtime();
    let _ = runtime.block_on(async move { block_on_websockets_main(addr) });
}

pub async fn block_on_websockets_main(addr: &str) -> Result<(), JoinError> {
    let handle = run_websockets_transport_async(ServerOptions { addr })
        .await
        .unwrap();
    handle.loop_handle.await
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    input_sender: tokio::sync::broadcast::Sender<tungstenite::Message>,
) -> tungstenite::Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    log::info!("New WebSocket connection: {:?}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        log::info!("Received message {:?}", msg);
        if msg.is_text() || msg.is_binary() {
            if let Err(err) = input_sender.send(msg) {
                log::error!("Failed to broadcast message {}", err);
            }
        }
    }

    Ok(())
}

async fn accept_connection(
    peer: SocketAddr,
    stream: TcpStream,
    input_sender: tokio::sync::broadcast::Sender<tungstenite::Message>,
) {
    if let Err(e) = handle_connection(peer, stream, input_sender).await {
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

#[allow(clippy::needless_lifetimes)]
async fn run_websockets_accept_loop(
    listener: TcpListener,
    input_sender: Sender<Message>,
    current_id: AtomicCell<u32>,
    connections: Arc<Mutex<HashMap<u32, ()>>>,
) {
    log::info!("Waiting for ws connections");
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        log::info!("Peer address: {}", peer);

        let connection_id = current_id.fetch_add(1);
        {
            let mut connections = connections.lock().await;
            connections.insert(connection_id, ());
        }

        {
            let connections = connections.clone();
            let input_sender = input_sender.clone();
            tokio::spawn(async move {
                accept_connection(peer, stream, input_sender).await;
                let mut connections = connections.lock().await;
                connections.remove(&connection_id);
            });
        }
    }
}

pub struct ServerHandle {
    loop_handle: tokio::task::JoinHandle<()>,
    input_broadcast: tokio::sync::broadcast::Receiver<tungstenite::Message>,
    connections: Arc<Mutex<HashMap<u32, ()>>>,
}

impl ServerHandle {
    async fn get_num_connected_clients(&self) -> usize {
        (*self.connections.lock().await).len()
    }
    pub fn abort(&self) {
        self.loop_handle.abort();
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

pub async fn run_websockets_transport_async<'a>(
    options: ServerOptions<'a>,
) -> tokio::io::Result<ServerHandle> {
    let listener: tokio::net::TcpListener = TcpListener::bind(options.addr).await?;
    log::info!("Listening on: {}", options.addr);
    let (input_sender, input_broadcast) = tokio::sync::broadcast::channel(1);

    let connections = Arc::new(Mutex::new(HashMap::new()));
    let current_id = crossbeam::atomic::AtomicCell::<u32>::new(0);

    let loop_handle = tokio::spawn(run_websockets_accept_loop(
        listener,
        input_sender,
        current_id,
        connections.clone(),
    ));

    Ok(ServerHandle {
        loop_handle,
        input_broadcast,
        connections,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::editor::protocol::AppStartedMessage;
    use log::LevelFilter;
    use tokio::time::Duration;

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
        let app_started = ClientMessageInner::AppStarted(AppStartedMessage {}).notification();
        ws_stream
            .send(tungstenite::Message::Text(
                serde_json::to_string(&app_started).unwrap(),
            ))
            .await
            .unwrap();

        let msg = transport.input_broadcast.recv().await.unwrap();
        assert!(msg.is_text());
        assert_eq!(
            msg.into_text().unwrap(),
            "{\"id\":null,\"channel\":\"default\",\"message\":{\"type\":\"AppStarted\"}}"
        );
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
