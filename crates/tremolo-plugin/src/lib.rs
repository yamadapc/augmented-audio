mod editor;
mod plugin_parameter;

extern crate log;
#[macro_use]
extern crate vst;
extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate crossbeam;
extern crate darwin_webkit;
extern crate log4rs;
extern crate oscillator;
extern crate proc_macro;
extern crate serde;

use log::info;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use oscillator::Oscillator;
use plugin_parameter::{ParameterStore, PluginParameterImpl};
use std::sync::Arc;
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

fn configure_logging() -> Option<()> {
    let home_path = dirs::home_dir()?;
    let log_dir = home_path.join(".ruas");
    std::fs::create_dir_all(log_dir.clone());
    let log_path = log_dir.join("tremolo-plugin.log");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} [{l}] {M}:{L} - {m} - tid:{T}:{t} pid:{P}\n",
        )))
        .build(log_path)
        .ok()?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .ok()?;

    log4rs::init_config(config).ok()?;

    Some(())
}

static LEFT_RATE_PARAMETER_ID: &str = "left_rate";
static RIGHT_RATE_PARAMETER_ID: &str = "right_rate";
static DEPTH_PARAMETER_ID: &str = "depth";

struct TremoloParameters {}

impl PluginParameters for TremoloParameters {}

struct TremoloPlugin {
    parameters: Arc<ParameterStore>,
    oscillator_left: Oscillator<f32>,
    oscillator_right: Oscillator<f32>,
}

impl TremoloPlugin {
    fn build_parameters() -> ParameterStore {
        let mut store = ParameterStore::new();
        store.add_parameter(
            LEFT_RATE_PARAMETER_ID,
            Arc::new(PluginParameterImpl::new_with("Rate", "Hz", 0.1, true)),
        );
        store.add_parameter(
            RIGHT_RATE_PARAMETER_ID,
            Arc::new(PluginParameterImpl::new_with("Rate", "Hz", 0.1, true)),
        );
        store.add_parameter(
            DEPTH_PARAMETER_ID,
            Arc::new(PluginParameterImpl::new_with("Depth", "", 1.0, true)),
        );
        store
    }
}

impl Plugin for TremoloPlugin {
    fn new(_host: HostCallback) -> Self {
        configure_logging();
        info!("TremoloPlugin - Started");

        TremoloPlugin {
            parameters: Arc::new(TremoloPlugin::build_parameters()),
            oscillator_left: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::sine_generator,
            ),
            oscillator_right: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::sine_generator,
            ),
        }
    }

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

    fn set_sample_rate(&mut self, rate: f32) {
        info!("TremoloPlugin::set_sample_rate");
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    fn start_process(&mut self) {
        info!("TremoloPlugin::start_process");
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let left_rate = self.parameter_value(LEFT_RATE_PARAMETER_ID);
        let right_rate = self.parameter_value(RIGHT_RATE_PARAMETER_ID);
        let depth = self.parameter_value(DEPTH_PARAMETER_ID);

        self.oscillator_left.set_frequency(left_rate);
        self.oscillator_right.set_frequency(right_rate);

        let num_channels = buffer.input_count();
        let num_samples = buffer.samples();
        let (input, mut output) = buffer.split();

        for channel in 0..num_channels {
            let osc = if channel == 0 {
                &mut self.oscillator_left
            } else {
                &mut self.oscillator_right
            };

            let input_samples = input.get(channel % input.len());
            let output_samples = output.get_mut(channel % output.len());

            for sample_index in 0..num_samples {
                let volume = osc.next_sample();
                let dry_signal = input_samples[sample_index];
                let wet_signal = volume * input_samples[sample_index];
                // mixed_signal = (1.0 - depth) * dry + depth * wet
                // mixed_signal = dry - dry * depth + depth * wet
                let mixed_signal = dry_signal + depth * (wet_signal - dry_signal);

                output_samples[sample_index] = mixed_signal;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(editor::TremoloEditor::new(
            self.parameters.clone(),
        )))
    }
}

impl TremoloPlugin {
    fn parameter_value(&mut self, id: &str) -> f32 {
        self.parameters.find_parameter(id).as_ref().unwrap().value()
    }
}

plugin_main!(TremoloPlugin); // Important!
