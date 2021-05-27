mod protocol;
mod transport;
mod webview;

use editor::protocol::{
    ClientMessage, ClientMessageInner, MessageWrapper, ParameterDeclarationMessage,
    PublishParametersMessage, ServerMessage, ServerMessageInner,
};
use editor::transport::{WebSocketsTransport, WebviewTransport};
use editor::webview::WebviewHolder;
use plugin_parameter::ParameterStore;
use std::ffi::c_void;
use std::sync::Arc;
use vst::editor::Editor;

pub struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    webview: Option<WebviewHolder>,
    transport: Box<dyn WebviewTransport<ServerMessage, ClientMessage>>,
}

impl TremoloEditor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        TremoloEditor {
            parameters,
            webview: None,
            // TODO - WebSockets is just for development
            transport: Box::new(WebSocketsTransport::new_with_addr("localhost:9510")),
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

        if let Err(err) = self.transport.start() {
            log::error!("Failed to start transport {}", err);
        } else {
            let input_channel = self.transport.get_input_channel();
            let output_channel = self.transport.get_output_channel();
            let parameters = self.parameters.clone();
            std::thread::spawn(move || {
                log::info!("Handling App messages");
                loop {
                    let MessageWrapper { message, .. } = input_channel.recv().unwrap();
                    match message {
                        ClientMessageInner::AppStarted(_) => {
                            log::info!("App is started. Publishing parameters");
                            output_channel.send(MessageWrapper {
                                id: None,
                                channel: "default".to_string(),
                                message: ServerMessageInner::PublishParameters(
                                    PublishParametersMessage {
                                        parameters: list_parameters(parameters.clone()),
                                    },
                                ),
                            });
                        }
                        _ => {}
                    }
                }
            });
        }

        Some(true)
    }
}

fn list_parameters(parameters: Arc<ParameterStore>) -> Vec<ParameterDeclarationMessage> {
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
