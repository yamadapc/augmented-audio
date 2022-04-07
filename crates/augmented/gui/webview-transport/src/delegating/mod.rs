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
use crate::WebviewTransport;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub type TransportRef<ServerMessage, ClientMessage> =
    Box<dyn WebviewTransport<ServerMessage, ClientMessage> + Send + Sync>;
pub type TransportsList<ServerMessage, ClientMessage> =
    Vec<TransportRef<ServerMessage, ClientMessage>>;
pub type TransportsListRef<ServerMessage, ClientMessage> =
    Arc<Mutex<TransportsList<ServerMessage, ClientMessage>>>;

pub struct DelegatingTransport<ServerMessage, ClientMessage> {
    transports: TransportsListRef<ServerMessage, ClientMessage>,
    client_channel: (Sender<ClientMessage>, Receiver<ClientMessage>),
    server_channel: (Sender<ServerMessage>, Receiver<ServerMessage>),
    server_forward_handle: Option<JoinHandle<()>>,
    client_forward_handles: Vec<JoinHandle<()>>,
}

impl<ServerMessage, ClientMessage> DelegatingTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    pub fn new(
        transports: TransportsListRef<ServerMessage, ClientMessage>,
        client_channel: (Sender<ClientMessage>, Receiver<ClientMessage>),
        server_channel: (Sender<ServerMessage>, Receiver<ServerMessage>),
    ) -> Self {
        DelegatingTransport {
            transports,
            client_channel,
            server_channel,
            server_forward_handle: None,
            client_forward_handles: Vec::new(),
        }
    }

    pub fn from_transports(transports: TransportsList<ServerMessage, ClientMessage>) -> Self {
        DelegatingTransport::new(Arc::new(Mutex::new(transports)), channel(1), channel(1))
    }
}

#[async_trait]
impl<ServerMessage, ClientMessage> WebviewTransport<ServerMessage, ClientMessage>
    for DelegatingTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("Starting delegating transport");
        {
            let mut transports = self.transports.lock().await;
            for transport in transports.iter_mut() {
                log::info!("Starting transport");
                transport.start().await?;
            }
        }

        let server_message_receiver = self.server_channel.0.subscribe();
        let transports = self.transports.clone();
        self.server_forward_handle = Some(tokio::spawn(async move {
            run_forward_server_message_loop(server_message_receiver, transports).await
        }));

        {
            let transports = self.transports.lock().await;
            for transport in transports.iter() {
                let client_messages = transport.messages();
                let client_messages_sender = self.client_channel.0.clone();
                let client_handle = tokio::spawn(async move {
                    run_message_forwarder_loop(client_messages, client_messages_sender).await
                });
                self.client_forward_handles.push(client_handle);
            }
        }

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        {
            let transports = self.transports.lock().await;
            for transport in transports.iter() {
                transport.stop().await?;
            }
        }

        if let Some(handle) = &self.server_forward_handle {
            handle.abort();
        }
        Ok(())
    }

    fn messages(&self) -> Receiver<ClientMessage> {
        let (sender, _) = &self.client_channel;
        sender.subscribe()
    }

    fn output_messages(&self) -> Sender<ServerMessage> {
        let (sender, _) = &self.server_channel;
        sender.clone()
    }

    async fn send(&self, message: ServerMessage) {
        let (sender, _) = &self.server_channel;
        let _ = sender.send(message);
    }
}

async fn run_message_forwarder_loop<ClientMessage>(
    mut client_messages: Receiver<ClientMessage>,
    client_messages_sender: Sender<ClientMessage>,
) where
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    loop {
        if let Err(err) = forward_message(&mut client_messages, &client_messages_sender).await {
            log::error!("Failed to forward message {}", err);
        }
    }
}

async fn forward_message<ClientMessage>(
    client_messages: &mut Receiver<ClientMessage>,
    client_messages_sender: &Sender<ClientMessage>,
) -> Result<(), Box<dyn Error>>
where
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    let message = client_messages.recv().await?;
    log::debug!("Forwarding message from transport {:?}", message);
    client_messages_sender.send(message)?;
    Ok(())
}

async fn run_forward_server_message_loop<ServerMessage, ClientMessage>(
    mut server_message_receiver: Receiver<ServerMessage>,
    transports: TransportsListRef<ServerMessage, ClientMessage>,
) -> !
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    loop {
        if let Err(err) = forward_server_message(&mut server_message_receiver, &transports).await {
            log::error!("Failed to forward server message {}", err);
        }
    }
}

async fn forward_server_message<ServerMessage, ClientMessage>(
    server_message_receiver: &mut Receiver<ServerMessage>,
    transports: &TransportsListRef<ServerMessage, ClientMessage>,
) -> Result<(), Box<dyn Error>>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    let message = server_message_receiver.recv().await?;
    let transports = transports.lock().await;
    for transport in transports.iter() {
        log::debug!("Forwarding messages to each transport {:?}", message);
        let message = message.clone();
        transport.send(message).await;
    }

    Ok(())
}
