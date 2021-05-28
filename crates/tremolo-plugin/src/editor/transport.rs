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
    connections: Arc<Mutex<Vec<WebSocketConnection>>>,
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
            connections: Arc::new(Mutex::new(vec![])),
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
        // let lock = self.connections.lock().unwrap();
        //
        // let serialized_message = serde_json::to_string(&msg)?;
        // log::info!(
        //     "Broadcasting message: {} num_clients={}",
        //     serialized_message,
        //     lock.deref().len()
        // );
        //
        // for (id, connection) in lock.deref().iter().enumerate() {
        //     log::info!("Client {} sending message", id);
        //     let mut websocket = connection.websocket.lock().unwrap();
        //
        //     let result =
        //         websocket.write_message(tungstenite::Message::Text(serialized_message.clone()));
        //     if let Err(write_err) = result {
        //         log::error!("Client {} Failed to send message {:?}", id, write_err);
        //     } else {
        //         websocket.write_pending();
        //         log::info!("Client {} received message", id);
        //     }
        // }

        Ok(())
    }

    fn start_server_thread(&mut self) {
        if self.thread_handle.is_some() {
            return;
        }

        log::info!("WebSocketsTransport - Starting TCP server thread");
        let addr = self.addr.clone();
        let input_sender = self.inputs.0.clone();
        let connections = self.connections.clone();
        let running = self.running.clone();

        {
            let mut is_running = running.lock().unwrap();
            *is_running = true;
        }

        self.thread_handle = Some(thread::spawn(move || {
            run_websockets_transport_main(&addr);

            // let server = TcpListener::bind(addr).unwrap();
            // for either_stream in server.incoming() {
            //     let is_running = running.lock().ok().map(|v| *v).unwrap_or(false);
            //     if !is_running {
            //         return;
            //     }
            //
            //     let run = || -> Result<(), Box<dyn Error>> {
            //         let stream = either_stream?;
            //         log::info!("WebSocketsTransport - Accepting connection");
            //
            //         let connections_ref = connections.clone();
            //         let mut connections = connections.lock()?;
            //         let connection = WebSocketConnection::start(
            //             connections.len(),
            //             stream,
            //             input_sender.clone(),
            //             move |id| {
            //                 log::info!("WebSocketsTransport - Cleaning-upp connection");
            //                 connections_ref.lock().unwrap().remove(id);
            //             },
            //         )?;
            //         connections.push(connection);
            //         Ok(())
            //     };
            //
            //     if let Err(err) = run() {
            //         log::error!("WebSocketsTransport - Error {}", err);
            //     }
            // }
        }));

        // TODO - manage writer thread properly - maybe don't use threads :D
        // clean-up this mess
        let output_receiver = self.outputs.1.clone();
        let connections = self.connections.clone();
        // thread::spawn(move || loop {
        //     loop {
        //         let msg = output_receiver.recv().unwrap();
        //         let lock = connections.lock().unwrap();
        //         let serialized_message = serde_json::to_string(&msg).unwrap();
        //         log::info!(
        //             "Broadcasting message: {} num_clients={}",
        //             serialized_message,
        //             lock.deref().len()
        //         );
        //
        //         for (id, connection) in lock.deref().iter().enumerate() {
        //             log::info!("Client {} sending message", id);
        //             let mut websocket = connection.websocket.lock().unwrap();
        //
        //             let result = websocket
        //                 .write_message(tungstenite::Message::Text(serialized_message.clone()));
        //             if let Err(write_err) = result {
        //                 log::error!("Client {} Failed to send message {:?}", id, write_err);
        //             } else {
        //                 websocket.write_pending();
        //                 log::info!("Client {} received message", id);
        //             }
        //         }
        //     }
        // });
    }
}

struct WebSocketConnection {
    running: Arc<Mutex<bool>>,
    thread_id: JoinHandle<()>,
    websocket: Arc<Mutex<WebSocket<TcpStream>>>,
}

// impl WebSocketConnection {
//     fn start<Func>(
//         id: usize,
//         connection: TcpStream,
//         input_sender: Sender<MessageWrapper<ClientMessageInner>>,
//         terminate: Func,
//     ) -> Result<Self, Box<dyn Error>>
//     where
//         Func: 'static + Fn(usize) + Send,
//     {
//         let websocket = Arc::new(Mutex::new(tungstenite::server::accept(connection)?));
//         let websocket_thread_copy = websocket.clone();
//         let running = Arc::new(Mutex::new(true));
//         let running_thread_copy = running.clone();
//         let thread_id = thread::spawn(move || {
//             // Self::run_loop(websocket_thread_copy, input_sender, running_thread_copy).unwrap_or(());
//             terminate(id);
//         });
//
//         Ok(WebSocketConnection {
//             thread_id,
//             running,
//             websocket,
//         })
//     }
//
//     // fn run_loop(
//     //     websocket: Arc<Mutex<WebSocket<TcpStream>>>,
//     //     input_sender: Sender<MessageWrapper<ClientMessageInner>>,
//     //     running: Arc<Mutex<bool>>,
//     // ) -> Option<()> {
//     //     loop {
//     //         {
//     //             if *running.lock().ok()? == false {
//     //                 return Some(());
//     //             }
//     //         }
//     //
//     //         let mut websocket = websocket.lock().ok()?;
//     //         let message = websocket.read_message();
//     //         if message.is_err() {
//     //             log::error!("Reading message from socket failed. Connection will be dropped");
//     //             websocket.close(None);
//     //             // TODO - the connections vector needs to be updated
//     //             return Some(());
//     //         }
//     //         let message = message.unwrap();
//     //
//     //         let run = || -> Result<(), Box<dyn Error>> {
//     //             let text = message.into_text()?;
//     //
//     //             log::info!("WebSocketConnection - Received message {}", text);
//     //             let client_message: MessageWrapper<ClientMessageInner> =
//     //                 serde_json::from_str(&text)?;
//     //             input_sender.send(client_message)?;
//     //             Ok(())
//     //         };
//     //
//     //         match run() {
//     //             Err(err) => {
//     //                 log::error!("WebSocketConnection Error - {}", err);
//     //             }
//     //             _ => {}
//     //         }
//     //     }
//     //
//     //     Some(())
//     // }
//
//     fn stop(self) -> Result<(), Box<dyn Error>> {
//         let mut running = self.running.lock().unwrap();
//         *running = false;
//         self.thread_id
//             .join()
//             .expect("Failed to join connection thread");
//         Ok(())
//     }
// }
