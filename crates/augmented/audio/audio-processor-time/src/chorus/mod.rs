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
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};
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

    fn prepare(&mut self, context: &mut AudioContext) {
        self.mono_delay_processor.resize_with(
            context.settings.output_channels(),
            MonoDelayProcessor::default,
        );
        for processor in &mut self.mono_delay_processor {
            processor.m_prepare(context);
            processor.handle().set_feedback(0.0);
            processor.handle().set_delay_time_secs(0.01);
        }

        self.oscillator
            .set_sample_rate(context.settings.sample_rate());
        self.oscillator.set_frequency(3.0);
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        for frame_num in 0..data.num_samples() {
            let time = self.oscillator.next_sample();

            for (channel_num, delay) in self.mono_delay_processor.iter_mut().enumerate() {
                let sample = &mut data.channels_mut()[channel_num][frame_num];
                delay.handle().set_delay_time_secs(0.02 + time * 0.001);
                *sample = *sample + 0.4 * delay.m_process(context, *sample)
            }
        }
    }
}
