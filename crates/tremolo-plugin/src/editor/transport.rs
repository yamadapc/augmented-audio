use crate::editor::protocol::{ClientMessageInner, MessageWrapper, ServerMessageInner};
use crate::editor::tokio_websockets::run_websockets_transport_main;
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

pub trait WebviewTransport<ServerMessage, ClientMessage> {
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(self) -> Result<(), Box<dyn Error>>;
    fn get_output_channel(&self) -> Sender<ServerMessage>;
    fn get_input_channel(&self) -> Receiver<ClientMessage>;
}

pub struct WebSocketsTransport {
    addr: String,
    inputs: (
        Sender<MessageWrapper<ClientMessageInner>>,
        Receiver<MessageWrapper<ClientMessageInner>>,
    ),
    outputs: (
        Sender<MessageWrapper<ServerMessageInner>>,
        Receiver<MessageWrapper<ServerMessageInner>>,
    ),
    thread_handle: Option<JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
}

impl WebviewTransport<MessageWrapper<ServerMessageInner>, MessageWrapper<ClientMessageInner>>
    for WebSocketsTransport
{
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.start_server_thread();
        Ok(())
    }

    fn stop(self) -> Result<(), Box<dyn Error>> {
        let mut is_running = self.running.lock().unwrap();
        *is_running = false;
        Ok(())
    }

    fn get_output_channel(&self) -> Sender<MessageWrapper<ServerMessageInner>> {
        self.outputs.0.clone()
    }

    fn get_input_channel(&self) -> Receiver<MessageWrapper<ClientMessageInner>> {
        self.inputs.1.clone()
    }
}

impl WebSocketsTransport {
    pub fn new(
        addr: &str,
        inputs: (
            Sender<MessageWrapper<ClientMessageInner>>,
            Receiver<MessageWrapper<ClientMessageInner>>,
        ),
        outputs: (
            Sender<MessageWrapper<ServerMessageInner>>,
            Receiver<MessageWrapper<ServerMessageInner>>,
        ),
    ) -> Self {
        WebSocketsTransport {
            addr: String::from(addr),
            inputs,
            outputs,
            thread_handle: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn new_with_addr(addr: &str) -> Self {
        WebSocketsTransport::new(addr, channel::unbounded(), channel::unbounded())
    }
}

impl WebSocketsTransport {
    pub fn inputs(
        &self,
    ) -> &(
        Sender<MessageWrapper<ClientMessageInner>>,
        Receiver<MessageWrapper<ClientMessageInner>>,
    ) {
        &self.inputs
    }

    pub fn outputs(
        &self,
    ) -> &(
        Sender<MessageWrapper<ServerMessageInner>>,
        Receiver<MessageWrapper<ServerMessageInner>>,
    ) {
        &self.outputs
    }
}

impl WebSocketsTransport {
    fn send_message(&self, msg: MessageWrapper<ServerMessageInner>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn start_server_thread(&mut self) {
        if self.thread_handle.is_some() {
            return;
        }

        log::info!("WebSocketsTransport - Starting TCP server thread");
        let addr = self.addr.clone();
        let running = self.running.clone();

        {
            let mut is_running = running.lock().unwrap();
            *is_running = true;
        }

        self.thread_handle = Some(thread::spawn(move || {
            run_websockets_transport_main(&addr);
        }));
    }
}
