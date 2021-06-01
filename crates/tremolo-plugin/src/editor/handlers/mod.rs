use std::sync::Arc;

use tokio::sync::broadcast::{Receiver, Sender};

use ClientMessageInner::{AppStarted, Log, SetParameter};

use crate::editor::list_parameters;
use crate::editor::protocol::{
    ClientMessage, ClientMessageInner, MessageWrapper, PublishParametersMessage, ServerMessage,
    ServerMessageInner,
};
use crate::plugin_parameter::ParameterStore;

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
                    match parameter_store.find_parameter(&set_parameter.parameter_id) {
                        Some(parameter) => {
                            parameter.set_value(set_parameter.value);
                        }
                        None => {
                            log::error!("Front-end is asking to set unknown parameter");
                        }
                    }
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

fn app_started(
    output_messages: &Sender<MessageWrapper<ServerMessageInner>>,
    parameter_store: &Arc<ParameterStore>,
) {
    log::info!("App started message received");
    let parameters_list = list_parameters(&parameter_store);
    let result = output_messages.send(ServerMessage::notification(
        ServerMessageInner::PublishParameters(PublishParametersMessage {
            parameters: parameters_list,
        }),
    ));

    if result.is_err() {
        log::error!("Failed to send publish parameters message");
    }
}
