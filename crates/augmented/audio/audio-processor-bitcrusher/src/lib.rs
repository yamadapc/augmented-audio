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

//! Implements a simple bitcrusher based on "sample-and-hold".
//!
//! [`BitCrusherProcessor`] is the [`audio_processor_traits::AudioProcessor`] implementation.
//!
//! [`BitCrusherHandle`] is the handle with which to change parameters from any thread. A generic
//! handle is implemented to generate generic GUIs.

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::atomic_float::{AtomicFloatRepresentable, AtomicValue};
use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor, Float};
pub use generic_handle::BitCrusherHandleRef;

mod generic_handle;

pub type BitCrusherHandle = BitCrusherHandleImpl<f32>;

pub struct BitCrusherHandleImpl<ST>
where
    ST: AtomicFloatRepresentable + Float,
{
    sample_rate: ST::AtomicType,
    bit_rate: ST::AtomicType,
}

impl<ST> BitCrusherHandleImpl<ST>
where
    ST: Float + AtomicFloatRepresentable,
    ST: From<f32>,
    f32: From<ST>,
{
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate.get().into()
    }

    pub fn bit_rate(&self) -> f32 {
        self.bit_rate.get().into()
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.sample_rate.set(sample_rate.into());
    }

    pub fn set_bit_rate(&self, bit_rate: f32) {
        self.bit_rate.set(bit_rate.into());
    }
}

impl<ST> Default for BitCrusherHandleImpl<ST>
where
    ST: Float + AtomicFloatRepresentable,
    ST: From<f32>,
{
    fn default() -> Self {
        Self {
            sample_rate: ST::AtomicType::from(44100.0.into()),
            bit_rate: ST::AtomicType::from(44100.0.into()),
        }
    }
}

pub type BitCrusherProcessor = BitCrusherProcessorImpl<f32>;

pub struct BitCrusherProcessorImpl<ST = f32>
where
    ST: AtomicFloatRepresentable + Float,
{
    handle: Shared<BitCrusherHandleImpl<ST>>,
}

impl<ST> AudioProcessorHandleProvider for BitCrusherProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync,
    ST: From<f32>,
    f32: From<ST>,
{
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(BitCrusherHandleRef::<ST>::new(self.handle.clone()))
    }
}

impl<ST> BitCrusherProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync,
    ST: From<f32>,
    f32: From<ST>,
{
    pub fn new(handle: Shared<BitCrusherHandleImpl<ST>>) -> Self {
        BitCrusherProcessorImpl { handle }
    }

    pub fn handle(&self) -> &Shared<BitCrusherHandleImpl<ST>> {
        &self.handle
    }

    fn step_size(&self) -> usize {
        (self.handle.sample_rate() / self.handle.bit_rate()) as usize
    }
}

impl<ST> Default for BitCrusherProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync,
    ST: From<f32>,
    f32: From<ST>,
{
    fn default() -> Self {
        Self::new(make_shared(BitCrusherHandleImpl::default()))
    }
}

impl<ST> AudioProcessor for BitCrusherProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync,
    ST: From<f32>,
    f32: From<ST>,
{
    type SampleType = ST;

    fn prepare(&mut self, context: &mut AudioContext) {
        let settings = context.settings;
        self.handle.set_sample_rate(settings.sample_rate());
        if (self.handle.sample_rate() - self.handle.bit_rate()).abs() < f32::EPSILON {
            self.handle.set_bit_rate(settings.sample_rate());
        }
    }

    fn process(&mut self, _context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        let step_size = self.step_size();

        let mut sample_index = 0;
        let buffer_size = data.num_samples();

        while sample_index < buffer_size {
            let first_index = sample_index;
            let limit_index = (sample_index + step_size).min(buffer_size);

            while sample_index < limit_index {
                for channel_index in 0..data.num_channels() {
                    let value = *data.get(channel_index, first_index);
                    data.set(channel_index, sample_index, value);
                }
                sample_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::sine_buffer;

    use audio_processor_traits::AudioProcessorSettings;

    use super::*;

    #[test]
    fn test_construct_bitcrusher() {
        let _processor = BitCrusherProcessor::default();
    }

    #[test]
    fn test_step_size_is_1_on_passthrough() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(&mut context);
        assert_eq!(processor.step_size(), 1);
    }

    #[test]
    fn test_step_size_is_2_on_lower_bitrate() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(&mut context);
        processor
            .handle()
            .set_bit_rate(settings.sample_rate() / 2.0);
        assert_eq!(processor.step_size(), 2);
    }

    #[test]
    fn test_passthrough_bitcrusher() {
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(&mut context);

        let input_buffer = AudioBuffer::from_interleaved(
            1,
            &sine_buffer(settings.sample_rate(), 440.0, Duration::from_millis(10)),
        );
        let mut output_buffer = input_buffer.clone();
        processor.process(&mut context, &mut output_buffer);

        assert_eq!(input_buffer.channel(0), output_buffer.channel(0));
    }
}
