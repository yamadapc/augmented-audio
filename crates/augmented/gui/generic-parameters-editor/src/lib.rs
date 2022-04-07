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
use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use audio_parameter_store::ParameterStore;
use macos_bundle_resources::has_bundle;
use serde::de::DeserializeOwned;
use serde::Serialize;
use vst::editor::Editor;
use webview_holder::WebviewHolder;
#[cfg(target_os = "macos")]
use webview_transport::webkit::WebkitTransport;
use webview_transport::{
    create_transport_runtime, DelegatingTransport, WebSocketsTransport, WebviewTransport,
};

use crate::protocol::{ClientMessage, ParameterDeclarationMessage, ServerMessage};

pub mod handlers;
pub mod protocol;

#[cfg(not(target_os = "macos"))]
fn initialize_transport<ServerMessage, ClientMessage>(
    webview: Arc<Mutex<WebviewHolder>>,
) -> Box<dyn WebviewTransport<ServerMessage, ClientMessage>>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    log::info!("Using websockets transport");
    let websockets_addr =
        std::env::var("WEBSOCKETS_TRANSPORT_ADDR").unwrap_or_else(|_| "localhost:9510".to_string());
    Box::new(DelegatingTransport::from_transports(vec![Box::new(
        WebSocketsTransport::new(&websockets_addr),
    )]))
}

#[cfg(target_os = "macos")]
fn initialize_transport<ServerMessage, ClientMessage>(
    webview: Arc<Mutex<WebviewHolder>>,
) -> Box<dyn WebviewTransport<ServerMessage, ClientMessage>>
where
    ServerMessage: Serialize + Send + Clone + Debug + 'static,
    ClientMessage: DeserializeOwned + Send + Clone + Debug + 'static,
{
    let use_websockets_transport = std::env::var("USE_WEBSOCKETS_TRANSPORT")
        .map(|v| v == "true")
        .unwrap_or(false);
    if use_websockets_transport {
        log::info!("Using websockets transport");
        let websockets_addr = std::env::var("WEBSOCKETS_TRANSPORT_ADDR")
            .unwrap_or_else(|_| "localhost:9510".to_string());
        Box::new(DelegatingTransport::from_transports(vec![
            Box::new(WebSocketsTransport::new(&websockets_addr)),
            Box::new(WebkitTransport::new(webview)),
        ]))
    } else {
        log::info!("Using webkit transport");
        Box::new(WebkitTransport::new(webview))
    }
}

pub struct GenericParametersEditorOptions {
    bundle_identifier: String,
    resource_name: String,
}

impl GenericParametersEditorOptions {
    pub fn new(bundle_identifier: String, resource_name: String) -> Self {
        GenericParametersEditorOptions {
            bundle_identifier,
            resource_name,
        }
    }
}

pub struct GenericParametersEditor {
    options: GenericParametersEditorOptions,
    parameters: Arc<ParameterStore>,
    runtime: tokio::runtime::Runtime,
    webview: Option<Arc<Mutex<WebviewHolder>>>,
    transport: Option<Box<dyn WebviewTransport<ServerMessage, ClientMessage>>>,
}

impl GenericParametersEditor {
    pub fn new(options: GenericParametersEditorOptions, parameters: Arc<ParameterStore>) -> Self {
        log::info!("Creating editor");
        let runtime = create_transport_runtime();
        GenericParametersEditor {
            options,
            parameters,
            webview: None,
            runtime,
            transport: None,
        }
    }

    fn initialize_webview(&mut self, parent: *mut c_void) -> Option<bool> {
        // If there's already a webkit just re-attach
        if let Some(webview) = &mut self.webview {
            let mut webview = webview.lock().unwrap();
            unsafe {
                webview.attach_to_parent(parent);
            }
            return Some(true);
        }

        let webview = unsafe { WebviewHolder::new(self.size()) };
        self.webview = Some(Arc::new(Mutex::new(webview)));

        self.transport = Some(initialize_transport::<ServerMessage, ClientMessage>(
            self.webview.clone().unwrap(),
        ));

        self.load_webview_url(parent);

        let start_result = self
            .runtime
            .block_on(self.transport.as_mut().unwrap().start());
        if let Err(err) = start_result {
            log::error!("Failed to start transport {}", err);
        }

        self.spawn_message_handler();

        Some(true)
    }

    fn load_webview_url(&mut self, parent: *mut c_void) {
        let mut webview = self.webview.as_mut().unwrap().lock().unwrap();

        if !has_bundle(&self.options.bundle_identifier) {
            log::warn!(
                "Plug-in does not have main bundle. Will be unable to run in production mode."
            );
            let frontend_url = "http://127.0.0.1:3000";
            log::warn!(
                "Initializing the front-end in development mode: {}",
                frontend_url
            );
            unsafe {
                webview.initialize(parent, frontend_url);
            }
        } else {
            log::info!("Plug-in has main bundle. Trying to find front-end resources");
            if let Some(frontend_path) = macos_bundle_resources::get_path(
                &self.options.bundle_identifier,
                &self.options.resource_name,
                None,
                None,
            ) {
                log::info!(
                    "Found front-end directory at \"{}\"",
                    frontend_path.to_str().unwrap()
                );
                unsafe {
                    webview.initialize(parent, frontend_path.to_str().unwrap());
                }
            } else {
                log::warn!("Did not find front-end directory");
            }
        }
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

impl Editor for GenericParametersEditor {
    fn size(&self) -> (i32, i32) {
        (300, 250)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn close(&mut self) {
        log::info!("Editor::close");
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn open(&mut self, parent: *mut c_void) -> bool {
        log::info!("Editor::open");
        self.initialize_webview(parent).unwrap_or(false)
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}
