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
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

use webview_holder::WebviewHolder;

use crate::WebviewTransport;

// TODO fix error handling / logging
pub struct WebkitTransport<ServerMessage, ClientMessage> {
    client_channel: (Sender<ClientMessage>, Receiver<ClientMessage>),
    server_channel: (Sender<ServerMessage>, Receiver<ServerMessage>),
    webview: Arc<Mutex<WebviewHolder>>,
    input_task: Option<JoinHandle<()>>,
    output_task: Option<JoinHandle<()>>,
}

impl<ServerMessage, ClientMessage> WebkitTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug,
    ClientMessage: DeserializeOwned + Send + Clone + Debug,
{
    pub fn new(webview: Arc<Mutex<WebviewHolder>>) -> Self {
        let client_channel = channel(1);
        let server_channel = channel(1);

        WebkitTransport {
            client_channel,
            server_channel,
            webview,
            input_task: None,
            output_task: None,
        }
    }
}

fn parse_and_forward_message<ClientMessage>(
    msg: String,
    client_message_sender: &Sender<ClientMessage>,
) where
    ClientMessage: DeserializeOwned,
{
    match serde_json::from_str(&msg) {
        Ok(msg) => {
            if let Err(err) = client_message_sender.send(msg) {
                log::error!("Failed to forward JS message {}", err);
            }
        }
        Err(err) => {
            log::error!("Failed to parse JS message {}", err);
        }
    }
}

fn serialize_and_forward_message<ServerMessage>(
    msg: ServerMessage,
    webview_holder: Arc<Mutex<WebviewHolder>>,
) where
    ServerMessage: Serialize,
{
    let webview_holder = webview_holder.lock().unwrap();
    let _ = webview_holder.send_message(&msg);
}

#[async_trait]
impl<ServerMessage, ClientMessage> WebviewTransport<ServerMessage, ClientMessage>
    for WebkitTransport<ServerMessage, ClientMessage>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("Starting webkit transport");

        self.input_task = Some({
            log::info!("Starting client message loop");
            let (client_string_sender, mut client_string_receiver) = channel(1);
            let mut webview = self.webview.lock().unwrap();
            let client_message_sender = self.client_channel.0.clone();
            webview.set_on_message_callback(client_string_sender);

            tokio::spawn(async move {
                loop {
                    match client_string_receiver.recv().await {
                        Ok(client_message) => {
                            parse_and_forward_message(client_message, &client_message_sender)
                        }
                        Err(err) => {
                            log::error!("Failed to receive client message {}", err);
                        }
                    }
                }
            })
        });

        self.output_task = Some({
            log::info!("Starting server message loop");
            let webview_ref = self.webview.clone();
            let mut server_message_receiver = self.server_channel.0.subscribe();
            tokio::spawn(async move {
                loop {
                    let server_message = server_message_receiver.recv().await.unwrap();
                    serialize_and_forward_message(server_message, webview_ref.clone());
                }
            })
        });

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn Error>> {
        {
            let mut webview = self.webview.lock().unwrap();
            webview.clear_on_message_callback();
        }

        if let Some(task) = &self.output_task {
            task.abort();
        }
        if let Some(task) = &self.input_task {
            task.abort();
        }
        Ok(())
    }

    fn messages(&self) -> Receiver<ClientMessage> {
        self.client_channel.0.subscribe()
    }

    fn output_messages(&self) -> Sender<ServerMessage> {
        self.server_channel.0.clone()
    }

    async fn send(&self, message: ServerMessage) {
        let webview_holder = self.webview.lock().unwrap();
        let _ = webview_holder.send_message(&message);
    }
}
