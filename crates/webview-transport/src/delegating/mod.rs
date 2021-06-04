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
    forward_handle: Option<JoinHandle<()>>,
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
            forward_handle: None,
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
                transport.start().await.unwrap();
            }
        }

        let mut server_message_receiver = self.server_channel.0.subscribe();
        let transports = self.transports.clone();
        self.forward_handle = Some(tokio::spawn(async move {
            loop {
                let message = server_message_receiver.recv().await.unwrap();
                let transports = transports.lock().await;
                for transport in transports.iter() {
                    log::debug!("Forwarding messages to each transport {:?}", message);
                    let message = message.clone();
                    transport.send(message).await;
                }
            }
        }));

        {
            let transports = self.transports.lock().await;
            for transport in transports.iter() {
                let mut client_messages = transport.messages();
                let client_messages_sender = self.client_channel.0.clone();
                tokio::spawn(async move {
                    loop {
                        let message = client_messages.recv().await.unwrap();
                        log::debug!("Forwarding message from transport {:?}", message);
                        client_messages_sender.send(message).unwrap();
                    }
                });
            }
        }

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        {
            let transports = self.transports.lock().await;
            for transport in transports.iter() {
                transport.stop().await.unwrap();
            }
        }

        if let Some(handle) = &self.forward_handle {
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
