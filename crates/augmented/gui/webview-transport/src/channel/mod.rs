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
