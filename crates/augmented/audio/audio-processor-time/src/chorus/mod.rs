use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor,
};
use augmented_oscillator::Oscillator;

use crate::MonoDelayProcessor;

pub struct ChorusProcessor {
    mono_delay_processor: Vec<MonoDelayProcessor<f32>>,
    oscillator: Oscillator<f32>,
}

impl Default for ChorusProcessor {
    fn default() -> Self {
        Self {
            mono_delay_processor: vec![],
            oscillator: Oscillator::sine(44100.0),
        }
    }
}

impl AudioProcessor for ChorusProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.mono_delay_processor
            .resize_with(settings.output_channels(), || MonoDelayProcessor::default());
        for processor in &mut self.mono_delay_processor {
            processor.s_prepare(settings);
            processor.handle().set_feedback(0.0);
            processor.handle().set_delay_time_secs(0.01);
        }

        self.oscillator.set_sample_rate(settings.sample_rate());
        self.oscillator.set_frequency(3.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            let time = self.oscillator.next_sample();
            for (sample, delay) in frame.iter_mut().zip(&mut self.mono_delay_processor) {
                delay.handle().set_delay_time_secs(0.02 + time * 0.001);
                *sample = *sample + 0.4 * delay.s_process(*sample)
            }
        }
    }
}
