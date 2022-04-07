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
use audio_garbage_collector::{make_shared, Shared};
use std::marker::PhantomData;
use std::ops::Mul;

use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{AtomicF32, Float};

pub struct GainProcessorHandle {
    gain: AtomicF32,
}

impl GainProcessorHandle {
    fn new(gain: impl Into<f32>) -> Self {
        Self {
            gain: AtomicF32::new(gain.into()),
        }
    }

    pub fn set_gain(&self, gain: impl Into<f32>) {
        self.gain.set(gain.into());
    }

    pub fn gain(&self) -> f32 {
        self.gain.get()
    }
}

/// An `AudioProcessor` which applies gain to an input signal
pub struct GainProcessor<SampleType> {
    handle: Shared<GainProcessorHandle>,
    phantom: PhantomData<SampleType>,
}

impl<SampleType> Default for GainProcessor<SampleType> {
    /// Construct a `GainProcessor` with 1.0 gain
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl<SampleType> GainProcessor<SampleType> {
    /// Construct a `GainProcessor` with gain
    pub fn new(gain: impl Into<f32>) -> Self {
        Self::new_with_handle(make_shared(GainProcessorHandle::new(gain)))
    }

    /// Construct a `GainProcessor` with a certain `GainProcessorHandle`
    pub fn new_with_handle(handle: Shared<GainProcessorHandle>) -> Self {
        Self {
            handle,
            phantom: PhantomData::default(),
        }
    }

    /// Change the gain
    pub fn set_gain(&self, gain: impl Into<f32>) {
        self.handle.set_gain(gain)
    }

    /// Get the gain
    pub fn gain(&self) -> f32 {
        self.handle.gain()
    }
}

impl<SampleType> SimpleAudioProcessor for GainProcessor<SampleType>
where
    SampleType: Float + Send + Sync + Mul<Output = SampleType>,
{
    type SampleType = SampleType;

    fn s_process(&mut self, sample: SampleType) -> SampleType {
        SampleType::from(self.gain()).unwrap() * sample
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::simple_processor;
    use audio_processor_traits::InterleavedAudioBuffer;

    use super::*;

    #[test]
    fn test_gain_does_its_thing() {
        let mut gain = GainProcessor::new(0.8);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(1, &mut samples);

        simple_processor::process_buffer(&mut gain, &mut input);

        for sample in samples {
            assert_f_eq!(sample, 0.8);
        }
    }

    #[test]
    fn test_gain_can_be_changed() {
        let mut gain = GainProcessor::default();
        gain.set_gain(0.8);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(1, &mut samples);

        simple_processor::process_buffer(&mut gain, &mut input);

        for sample in samples {
            assert_f_eq!(sample, 0.8);
        }
    }
}
