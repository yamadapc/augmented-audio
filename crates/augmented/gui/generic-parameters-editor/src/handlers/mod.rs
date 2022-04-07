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
use std::sync::Arc;

use tokio::sync::broadcast::{Receiver, Sender};

use audio_parameter_store::ParameterStore;
use ClientMessageInner::{AppStarted, Log, SetParameter};

use crate::list_parameters;
use crate::protocol::{
    ClientMessage, ClientMessageInner, MessageWrapper, PublishParametersMessage, ServerMessage,
    ServerMessageInner, SetParameterMessage,
};

pub async fn message_handler_loop(
    mut messages: Receiver<ClientMessage>,
    output_messages: Sender<ServerMessage>,
    parameter_store: &Arc<ParameterStore>,
) {
    loop {
        if let Ok(message) = messages.recv().await {
            let MessageWrapper { message, .. } = message;
            match message {
                AppStarted(_) => app_started(&output_messages, parameter_store),
                SetParameter(set_parameter) => {
                    handle_set_parameter(&output_messages, parameter_store, &set_parameter)
                }
                Log(log_message) => {
                    log::info!(
                        "FE - LogMessage - level={} message={}",
                        log_message.level,
                        log_message.message
                    )
                }
            }
        }
    }
}

fn handle_set_parameter(
    _output_messages: &Sender<ServerMessage>,
    parameter_store: &Arc<ParameterStore>,
    set_parameter: &SetParameterMessage,
) {
    match parameter_store.find_parameter(&set_parameter.parameter_id) {
        Some(parameter) => {
            parameter.set_value(set_parameter.value);

            // Broadcast of messages is disabled here.
            // output_messages
            //     .send(ServerMessage::notification(
            //         ServerMessageInner::ParameterValue(ParameterValueMessage {
            //             id: set_parameter.parameter_id.clone(),
            //             value: set_parameter.value,
            //         }),
            //     ))
            //     .unwrap();
        }
        None => {
            log::error!("Front-end is asking to set unknown parameter");
        }
    }
}

fn app_started(
    output_messages: &Sender<MessageWrapper<ServerMessageInner>>,
    parameter_store: &Arc<ParameterStore>,
) {
    log::info!("App started message received");
    let parameters_list = list_parameters(parameter_store);
    let result = output_messages.send(ServerMessage::notification(
        ServerMessageInner::PublishParameters(PublishParametersMessage {
            parameters: parameters_list,
        }),
    ));

    if result.is_err() {
        log::error!("Failed to send publish parameters message");
    }
}
