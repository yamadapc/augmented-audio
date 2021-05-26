mod editor;
mod plugin_parameter;

extern crate log;
#[macro_use]
extern crate vst;
extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate darwin_webkit;
extern crate oscillator;

use oscillator::Oscillator;
use plugin_parameter::{ParameterStore, PluginParameterImpl};
use std::sync::{Arc, Mutex};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

static RATE_PARAMETER_ID: &str = "rate";
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
            String::from(RATE_PARAMETER_ID),
            Arc::new(Mutex::new(PluginParameterImpl::new_with(
                String::from("Rate"),
                String::from("Hz"),
                0.1,
                true,
            ))),
        );
        store.add_parameter(
            String::from(DEPTH_PARAMETER_ID),
            Arc::new(Mutex::new(PluginParameterImpl::new_with(
                String::from("Depth"),
                String::from(""),
                1.0,
                true,
            ))),
        );
        store
    }
}

impl Plugin for TremoloPlugin {
    fn new(_host: HostCallback) -> Self {
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
        println!("TremoloPlugin - set_sample_rate");
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    // TODO - why isn't this called?
    fn start_process(&mut self) {
        println!("TremoloPlugin - start_process");
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if buffer.input_count() != buffer.output_count() {
            panic!("Unsupported input/output mismatch");
        }

        let num_channels = buffer.input_count();
        let num_samples = buffer.samples();
        let (input, mut output) = buffer.split();

        for channel in 0..num_channels {
            if channel > 2 {
                break;
            }

            let osc = if channel == 0 {
                &mut self.oscillator_left
            } else {
                &mut self.oscillator_right
            };
            let input_samples = input.get(channel);
            let output_samples = output.get_mut(channel);

            for sample_index in 0..num_samples {
                let volume = osc.next_sample();
                output_samples[sample_index] = volume * input_samples[sample_index];
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

plugin_main!(TremoloPlugin); // Important!
