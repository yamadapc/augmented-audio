#[macro_use]
extern crate vst;
extern crate oscillator;

use oscillator::Oscillator;
use vst::buffer::AudioBuffer;
use vst::plugin::{Category, HostCallback, Info, Plugin};

struct TremoloPlugin {
    oscillator_left: Oscillator<f32>,
    oscillator_right: Oscillator<f32>,
}

impl Plugin for TremoloPlugin {
    fn new(_host: HostCallback) -> Self {
        TremoloPlugin {
            oscillator_left: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::square_generator,
            ),
            oscillator_right: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::square_generator,
            ),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "TasV2".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2501, // Used by hosts to differentiate between plugins.
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        println!("TremoloPlugin - set_sample_rate");
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
        self.oscillator_left.set_frequency(10.);
        self.oscillator_right.set_frequency(10.);
    }

    // TODO - why isn't this called?
    fn start_process(&mut self) {
        println!("TremoloPlugin - start_process");
        self.oscillator_left.set_frequency(5.0);
        self.oscillator_right.set_frequency(5.0);
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
                let volume = osc.next();
                output_samples[sample_index] = volume * input_samples[sample_index];
            }
        }
    }
}

plugin_main!(TremoloPlugin); // Important!
