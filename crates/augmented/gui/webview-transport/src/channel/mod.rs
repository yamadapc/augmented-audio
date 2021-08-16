use crate::WebviewTransport;
use std::error::Error;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct ChannelTransport<ServerMessage, ClientMessage> {
    client_sender: Sender<ClientMessage>,
    server_sender: Sender<ServerMessage>,
}

impl<ServerMessage, ClientMessage> ChannelTransport<ServerMessage, ClientMessage> {
    pub fn new(client_sender: Sender<ClientMessage>, server_sender: Sender<ServerMessage>) -> Self {
        ChannelTransport {
            client_sender,
            server_sender,
        }
    }
}

impl<ServerMessage, ClientMessage> ChannelTransport<ServerMessage, ClientMessage> {
    pub fn client_sender(&self) -> &Sender<ClientMessage> {
        &self.client_sender
    }

    pub fn server_sender(&self) -> &Sender<ServerMessage> {
        &self.server_sender
    }

    pub fn set_client_sender(&mut self, client_sender: Sender<ClientMessage>) {
        self.client_sender = client_sender;
    }

    pub fn set_server_sender(&mut self, server_sender: Sender<ServerMessage>) {
        self.server_sender = server_sender;
    }
}

impl<ServerMessage, ClientMessage> Default for ChannelTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Clone,
    ClientMessage: Clone,
{
    fn default() -> Self {
        let (client_sender, _) = tokio::sync::broadcast::channel(1);
        let (server_sender, _) = tokio::sync::broadcast::channel(1);
        ChannelTransport::new(client_sender, server_sender)
    }
}

#[async_trait::async_trait]
impl<ServerMessage, ClientMessage> WebviewTransport<ServerMessage, ClientMessage>
    for ChannelTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Send,
    ClientMessage: Send,
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn messages(&self) -> Receiver<ClientMessage> {
        self.client_sender.subscribe()
    }

    fn output_messages(&self) -> Sender<ServerMessage> {
        self.server_sender.clone()
    }

    async fn send(&self, message: ServerMessage) {
        match self.server_sender.send(message) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to send message {}", err);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_sanity_with_low_capacity_broadcast_channel() {
        let (sender, mut receiver) = tokio::sync::broadcast::channel(5);
        sender.send(1).unwrap();
        sender.send(1).unwrap();
        sender.send(1).unwrap();
        sender.send(1).unwrap();
        sender.send(1).unwrap();

        let r = receiver.recv().await.unwrap();
        assert_eq!(r, 1);
        let r = receiver.recv().await.unwrap();
        assert_eq!(r, 1);
        let r = receiver.recv().await.unwrap();
        assert_eq!(r, 1);
        let r = receiver.recv().await.unwrap();
        assert_eq!(r, 1);
        let r = receiver.recv().await.unwrap();
        assert_eq!(r, 1);
    }
}
