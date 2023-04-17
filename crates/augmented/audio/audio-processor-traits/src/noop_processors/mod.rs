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

use std::marker::PhantomData;

use num::Float;

use crate::{AudioBuffer, AudioContext, AudioProcessor};

/// An audio-processor which doesn't do any work.
pub struct NoopAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> Default for NoopAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType> NoopAudioProcessor<SampleType> {
    pub fn new() -> Self {
        NoopAudioProcessor(PhantomData::default())
    }
}

impl<SampleType: Send + Copy> AudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;
    fn process(&mut self, _context: &mut AudioContext, _frame: &mut AudioBuffer<Self::SampleType>) {
    }
}

/// An audio-processor which mutes all channels.
pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> SilenceAudioProcessor<SampleType> {
    pub fn new() -> Self {
        SilenceAudioProcessor(PhantomData)
    }
}

impl<SampleType> Default for SilenceAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType: Float + Send + Sized> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process(&mut self, _context: &mut AudioContext, output: &mut AudioBuffer<SampleType>) {
        for channel in output.channels_mut() {
            for sample in channel {
                *sample = SampleType::zero();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::AudioProcessor;

    use super::*;

    #[test]
    fn test_noop_processor() {
        let mut output = NoopAudioProcessor::<f32>::default();
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);

        let buffer = [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0]];
        let mut buffer = AudioBuffer::from(buffer.iter().cloned());
        output.process(&mut ctx, &mut buffer);
        assert_eq!(
            buffer.channels(),
            &vec![vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0],]
        );
    }

    #[test]
    fn test_silence_processor() {
        let mut output = SilenceAudioProcessor::<f32>::default();
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);
        let mut buffer = AudioBuffer::from([[1.0, 1.0, 1.0], [2.0, 2.0, 2.0]].iter().cloned());

        output.process(&mut ctx, &mut buffer);
        assert_eq!(
            buffer.channels(),
            &vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0],]
        );
    }
}
