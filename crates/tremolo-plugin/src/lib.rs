extern crate cocoa;
extern crate crossbeam;
extern crate darwin_webkit;
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate objc;
extern crate oscillator;
extern crate proc_macro;
extern crate serde;
extern crate tungstenite;
#[macro_use]
extern crate vst;

use std::sync::Arc;

use audio_parameter_store::{ParameterStore, PluginParameter};
use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

use crate::constants::{DEPTH_PARAMETER_ID, PHASE_PARAMETER_ID, RATE_PARAMETER_ID};
use crate::processor::Processor;

pub mod constants;
pub mod editor;
// pub mod parameter_store;
pub mod processor;

fn configure_logging() -> Option<()> {
    let home_path = dirs::home_dir()?;
    let log_dir = home_path.join(".ruas");
    std::fs::create_dir_all(log_dir.clone()).ok()?;
    let log_path = log_dir.join("tremolo-plugin.log");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} [{l}] {M}:{L} - {m} - tid:{T}:{t} pid:{P}\n",
        )))
        .build(log_path)
        .ok()?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder().build())))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .ok()?;

    log4rs::init_config(config).ok()?;

    Some(())
}

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
        configure_logging();
        info!("TremoloPlugin - Started");

        let parameters = Arc::new(TremoloPlugin::build_parameters());
        let processor = Processor::new(parameters.clone());

        TremoloPlugin {
            parameters,
            processor,
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        info!("TremoloPlugin::set_sample_rate");
        self.processor.set_sample_rate(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.processor.process(buffer);
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn start_process(&mut self) {
        info!("TremoloPlugin::start_process");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(editor::TremoloEditor::new(
            self.parameters.clone(),
        )))
    }
}

plugin_main!(TremoloPlugin); // Important!
