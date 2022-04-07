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
use serde::{Deserialize, Serialize};

use audio_parameter_store::parameter::ParameterType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWrapper<Message> {
    pub id: Option<String>,
    pub channel: String,
    pub message: Message,
}

impl<Message> MessageWrapper<Message> {
    pub fn new(id: Option<String>, channel: String, message: Message) -> Self {
        MessageWrapper {
            id,
            channel,
            message,
        }
    }

    pub fn notification(message: Message) -> Self {
        Self::new(None, String::from("default"), message)
    }

    pub fn request(id: &str, message: Message) -> Self {
        Self::new(None, String::from(id), message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessageInner {
    PublishParameters(PublishParametersMessage),
    ParameterValue(ParameterValueMessage),
}
pub type ServerMessage = MessageWrapper<ServerMessageInner>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishParametersMessage {
    pub parameters: Vec<ParameterDeclarationMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValueMessage {
    pub id: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDeclarationMessage {
    pub id: String,
    pub name: String,
    pub label: String,
    pub text: String,
    pub value: f32,
    pub value_precision: u32,
    pub value_range: (f32, f32),
    pub value_type: ParameterType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessageInner {
    AppStarted(AppStartedMessage),
    SetParameter(SetParameterMessage),
    Log(LogMessage),
}

impl ClientMessageInner {
    pub fn notification(self) -> ClientMessage {
        ClientMessage::notification(self)
    }
}

pub type ClientMessage = MessageWrapper<ClientMessageInner>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStartedMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetParameterMessage {
    pub parameter_id: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub level: String,
    pub message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serde_enum() {
        let client_message = ClientMessageInner::AppStarted(AppStartedMessage {});
        let result = serde_json::to_string(&client_message).unwrap();
        assert_eq!(result, "{\"type\":\"AppStarted\"}")
    }
}
