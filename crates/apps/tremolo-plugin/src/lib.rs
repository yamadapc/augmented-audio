extern crate augmented_oscillator;
extern crate crossbeam;
extern crate log;
extern crate log4rs;
extern crate proc_macro;
extern crate serde;
extern crate tungstenite;
#[macro_use]
extern crate vst;

use std::sync::Arc;

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

use audio_parameter_store::{ParameterStore, PluginParameter};

use crate::config::get_configuration_root_path;
use crate::config::logging::configure_logging;
use crate::constants::{
    BUNDLE_IDENTIFIER, DEPTH_PARAMETER_ID, INDEX_HTML_RESOURCE, PHASE_PARAMETER_ID,
    RATE_PARAMETER_ID,
};
use crate::processor::Processor;
use generic_parameters_editor::{GenericParametersEditor, GenericParametersEditorOptions};

mod config;
pub mod constants;
pub mod processor;

struct TremoloPlugin {
    parameters: Arc<ParameterStore>,
    processor: Processor,
}

impl TremoloPlugin {
    fn build_parameters() -> ParameterStore {
        let mut store = ParameterStore::new();
        store.add_parameter(
            RATE_PARAMETER_ID,
            Arc::new(
                PluginParameter::builder()
                    .name("Rate")
                    .label("Hz")
                    .initial_value(1.0)
                    .value_precision(1)
                    // Really fun sounds when the modulation is at audio rate (over 30Hz)
                    .value_range(0.05, 10.0)
                    .build(),
            ),
        );
        store.add_parameter(
            DEPTH_PARAMETER_ID,
            Arc::new(
                PluginParameter::builder()
                    .name("Depth")
                    .initial_value(100.0)
                    .label("%")
                    .value_precision(0)
                    .value_range(0.0, 100.0)
                    .build(),
            ),
        );
        store.add_parameter(
            PHASE_PARAMETER_ID,
            Arc::new(
                PluginParameter::builder()
                    .name("Phase")
                    .initial_value(0.0)
                    .label("º")
                    .value_precision(0)
                    .value_range(0.0, 360.0)
                    .build(),
            ),
        );
        store
    }
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

        let parameters = Arc::new(TremoloPlugin::build_parameters());
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
        Some(Box::new(GenericParametersEditor::new(
            GenericParametersEditorOptions::new(
                String::from(BUNDLE_IDENTIFIER),
                String::from(INDEX_HTML_RESOURCE),
            ),
            self.parameters.clone(),
        )))
    }
}

impl Drop for TremoloPlugin {
    fn drop(&mut self) {
        log::info!("Shutting-down tremolo plugin");
    }
}

plugin_main!(TremoloPlugin); // Important!
