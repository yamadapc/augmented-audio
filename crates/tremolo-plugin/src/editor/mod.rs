use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use vst::editor::Editor;

use audio_parameter_store::ParameterStore;
use webview_holder::WebviewHolder;
use webview_transport::{create_transport_runtime, WebSocketsTransport, WebviewTransport};

use crate::editor::protocol::{ClientMessage, ParameterDeclarationMessage, ServerMessage};
use webview_transport::webkit::WebkitTransport;

pub mod handlers;
pub mod protocol;

pub struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    runtime: tokio::runtime::Runtime,
    webview: Option<Arc<Mutex<WebviewHolder>>>,
    transport: Option<Box<dyn WebviewTransport<ServerMessage, ClientMessage>>>,
}

impl TremoloEditor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        let runtime = create_transport_runtime();
        TremoloEditor {
            parameters,
            webview: None,
            runtime,
            transport: None,
        }
    }

    unsafe fn initialize_webview(&mut self, parent: *mut c_void) -> Option<bool> {
        // If there's already a webkit just re-attach
        if let Some(webview) = &mut self.webview {
            let mut webview = webview.lock().unwrap();
            webview.attach_to_parent(parent);
            return Some(true);
        }

        let webview = WebviewHolder::new(self.size());
        self.webview = Some(Arc::new(Mutex::new(webview)));
        {
            let mut webview = self.webview.as_mut().unwrap().lock().unwrap();
            webview.initialize(parent);
        }

        self.initialize_transport();
        let start_result = self
            .runtime
            .block_on(self.transport.as_mut().unwrap().start());
        if let Err(err) = start_result {
            log::error!("Failed to start transport {}", err);
        }

        self.spawn_message_handler();

        Some(true)
    }

    fn initialize_transport(&mut self) {
        self.transport = Some({
            let use_websockets_transport = std::env::var("USE_WEBSOCKETS_TRANSPORT")
                .map(|v| v == "true")
                .unwrap_or(false);
            if use_websockets_transport {
                let websockets_addr = std::env::var("WEBSOCKETS_TRANSPORT_ADDR")
                    .unwrap_or_else(|_| "localhost:9510".to_string());
                Box::new(WebSocketsTransport::new(&websockets_addr))
            } else {
                Box::new(WebkitTransport::new(self.webview.as_ref().unwrap().clone()))
            }
        });
    }

    fn spawn_message_handler(&mut self) {
        let messages = self.transport.as_mut().unwrap().messages();
        let output_messages = self.transport.as_mut().unwrap().output_messages();
        let parameter_store = self.parameters.clone();

        self.runtime.spawn(async move {
            handlers::message_handler_loop(messages, output_messages, &parameter_store).await
        });
    }
}

fn list_parameters(parameters: &ParameterStore) -> Vec<ParameterDeclarationMessage> {
    let num_parameters = parameters.get_num_parameters();
    let mut output = vec![];
    for i in 0..num_parameters {
        let parameter_id = parameters.find_parameter_id(i).unwrap();
        let parameter = parameters.find_parameter(&parameter_id).unwrap();
        output.push(ParameterDeclarationMessage {
            id: parameter_id,
            name: parameter.name(),
            label: parameter.label(),
            text: parameter.text(),
            value: parameter.value(),
            value_range: parameter.value_range(),
            value_type: parameter.value_type(),
            value_precision: parameter.value_precision(),
        })
    }
    output
}

impl Editor for TremoloEditor {
    fn size(&self) -> (i32, i32) {
        (300, 250)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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
