use std::ffi::c_void;
use std::sync::Arc;
use std::thread;

use vst::editor::Editor;

use crate::editor::protocol::{
    ClientMessage, ClientMessageInner, MessageWrapper, ParameterDeclarationMessage,
    PublishParametersMessage, ServerMessage, ServerMessageInner,
};
use crate::editor::tokio_websockets::create_transport_runtime;
use crate::editor::transport::{WebSocketsTransport, WebviewTransport};
use crate::editor::webview::WebviewHolder;
use crate::plugin_parameter::ParameterStore;
use tokio::sync::broadcast::error::SendError;

mod handlers;
mod protocol;
mod tokio_websockets;
mod transport;
mod webview;

pub struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    webview: Option<WebviewHolder>,
    runtime: tokio::runtime::Runtime,
    transport: Box<dyn WebviewTransport<ServerMessage, ClientMessage>>,
}

impl TremoloEditor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        let runtime = create_transport_runtime();
        TremoloEditor {
            parameters,
            webview: None,
            runtime,
            // TODO - WebSockets is just for development
            transport: Box::new(WebSocketsTransport::new("localhost:9510")),
        }
    }

    unsafe fn initialize_webview(&mut self, parent: *mut c_void) -> Option<bool> {
        // If there's already a webview just re-attach
        if let Some(webview) = &mut self.webview {
            webview.attach_to_parent(parent);
            return Some(true);
        }

        let webview = WebviewHolder::new(self.size());
        self.webview = Some(webview);
        self.webview.as_mut().unwrap().initialize(parent);

        let start_result = self.runtime.block_on(self.transport.start());
        if let Err(err) = start_result {
            log::error!("Failed to start transport {}", err);
        }

        {
            let mut messages = self.transport.messages();
            let output_messages = self.transport.output_messages();
            let parameter_store = self.parameters.clone();
            self.runtime.spawn(async move {
                loop {
                    if let Ok(message) = messages.recv().await {
                        match message {
                            MessageWrapper { message, .. } => match message {
                                ClientMessageInner::AppStarted(_) => {
                                    let parameters_list = list_parameters(&parameter_store);
                                    let result = output_messages.send(ServerMessage::notification(
                                        ServerMessageInner::PublishParameters(
                                            PublishParametersMessage {
                                                parameters: parameters_list,
                                            },
                                        ),
                                    ));

                                    match result {
                                        Err(_) => {
                                            log::error!(
                                                "Failed to send publish parameters message"
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                                ClientMessageInner::SetParameter(_) => {}
                                ClientMessageInner::Log(_) => {}
                            },
                        }
                    }
                }
            });
        }

        Some(true)
    }
}

fn list_parameters(parameters: &ParameterStore) -> Vec<ParameterDeclarationMessage> {
    let num_parameters = parameters.get_num_parameters();
    let mut output = vec![];
    for i in 0..num_parameters {
        let (parameter_id, parameter) = parameters.find_parameter_by_index(i).unwrap();
        output.push(ParameterDeclarationMessage {
            id: parameter_id,
            name: parameter.name(),
            label: parameter.label(),
            text: parameter.text(),
            value: parameter.value(),
        })
    }
    output
}

impl Editor for TremoloEditor {
    fn size(&self) -> (i32, i32) {
        (500, 500)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        log::info!("Editor::open");
        unsafe { self.initialize_webview(parent).unwrap_or(false) }
    }

    fn close(&mut self) {
        log::info!("Editor::close");
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}
