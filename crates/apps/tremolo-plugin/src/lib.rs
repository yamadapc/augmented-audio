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

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

use audio_parameter_store::ParameterStore;
use generic_parameters_editor::{GenericParametersEditor, GenericParametersEditorOptions};

use crate::config::get_configuration_root_path;
use crate::config::logging::configure_logging;
use crate::constants::{BUNDLE_IDENTIFIER, INDEX_HTML_RESOURCE};
use crate::processor::Processor;

/// Logging configuration
pub mod config;
/// Constants such as parameter IDs
pub mod constants;
/// Parameters and parameter store
pub mod parameters;
/// AudioProcessor implementation for volume modulation
pub mod processor;

/// VST plugin implementation for [`vst`], wraps the [`processor::Processor`].
///
/// This VST has a web GUI. The VST plugin will try to open the UI inside a webview.
/// When bundled, it's expected that the HTML and assets for the web GUI are inside of the macOS
/// extension bundle. The VST will try to use macOS bundle resource APIs to find these files.
///
/// If there is no bundle, the VST will try to load a local address `http://localhost:3000`. It will
/// also start a WebSockets server in this case so that the GUI can connect.
///
/// See: [`generic_parameters_editor`]
struct TremoloPlugin {
    parameters: Arc<ParameterStore>,
    processor: Processor,
}

impl Plugin for TremoloPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "TasV2".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2501, // Used by hosts to differentiate between plugins.
            parameters: self.parameters.get_num_parameters(),
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self {
        let config_root_path = get_configuration_root_path();
        if let Err(err) = configure_logging(&config_root_path) {
            eprintln!("ERROR: Logging set-up has failed {:?}", err);
        }
        log::info!("TremoloPlugin - Started");

        let parameters = Arc::new(crate::parameters::build_parameters());
        let processor = Processor::new(parameters.clone());

        TremoloPlugin {
            parameters,
            processor,
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        log::info!("TremoloPlugin::set_sample_rate: {}", rate);
        self.processor.set_sample_rate(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.processor.process(buffer);
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn start_process(&mut self) {
        log::info!("TremoloPlugin::start_process");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(build_parameters_editor(&self.parameters)))
    }
}

/// Build the webview based editor.
pub fn build_parameters_editor(parameters: &Arc<ParameterStore>) -> GenericParametersEditor {
    GenericParametersEditor::new(
        GenericParametersEditorOptions::new(
            String::from(BUNDLE_IDENTIFIER),
            String::from(INDEX_HTML_RESOURCE),
        ),
        parameters.clone(),
    )
}

impl TremoloPlugin {}

impl Drop for TremoloPlugin {
    fn drop(&mut self) {
        log::info!("Shutting-down tremolo plugin");
    }
}

vst::plugin_main!(TremoloPlugin); // Important!
