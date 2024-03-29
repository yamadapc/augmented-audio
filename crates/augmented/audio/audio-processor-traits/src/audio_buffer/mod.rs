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

use std::ops::{AddAssign, Div};

use num::Zero;

// pub use audio_buffer_trait::*;
// pub use interleaved_buffers::*;
// pub use owned_audio_buffer_trait::*;
pub use util::*;

// mod audio_buffer_trait;
// mod interleaved_buffers;
// mod owned_audio_buffer_trait;
mod util;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct AudioBuffer<SampleType> {
    channels: Vec<Vec<SampleType>>,
}

impl<SampleType: Clone + Default> AudioBuffer<SampleType> {
    pub fn from_interleaved(channels: usize, samples: &[SampleType]) -> Self {
        let mut buffers = vec![vec![SampleType::default(); samples.len() / channels]; channels];
        copy_from_interleaved(samples, buffers.as_mut_slice());

        Self::new(buffers)
    }

    pub fn copy_from_interleaved(&mut self, source: &[SampleType]) {
        copy_from_interleaved(source, &mut self.channels);
    }

    pub fn copy_into_interleaved(&self, target: &mut [SampleType]) {
        copy_into_interleaved(&self.channels, target);
    }
}

impl<SampleType: Copy + num::Zero> AudioBuffer<SampleType> {
    pub fn copy_from(&mut self, source: &Self) {
        for (target, source) in self.channels.iter_mut().zip(source.channels.iter()) {
            target.copy_from_slice(source);
        }
    }
}

impl<SampleType: Clone> AudioBuffer<SampleType> {
    pub fn resize_with(
        &mut self,
        channels: usize,
        samples: usize,
        mut value: impl FnMut() -> SampleType,
    ) {
        self.channels
            .resize_with(channels, || vec![value(); samples]);
    }
}

impl<SampleType: num::Zero + Clone> AudioBuffer<SampleType> {
    pub fn resize(&mut self, channels: usize, samples: usize) {
        if self.num_channels() != channels {
            self.channels
                .resize_with(channels, || vec![SampleType::zero(); samples]);
        }

        if self.num_samples() != samples {
            for channel in &mut self.channels {
                channel.resize(samples, SampleType::zero());
            }
        }
    }
}

impl<SampleType> AudioBuffer<SampleType> {
    pub fn new(channels: Vec<Vec<SampleType>>) -> Self {
        Self { channels }
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn channel(&self, channel: usize) -> &[SampleType] {
        &self.channels[channel]
    }

    pub fn channel_mut(&mut self, channel: usize) -> &mut [SampleType] {
        &mut self.channels[channel]
    }

    pub fn channels(&self) -> &Vec<Vec<SampleType>> {
        &self.channels
    }

    pub fn channels_mut(&mut self) -> &mut Vec<Vec<SampleType>> {
        &mut self.channels
    }

    pub fn slice_mut(&mut self) -> impl Iterator<Item = &mut SampleType> {
        self.channels.iter_mut().flat_map(|c| c.iter_mut())
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }

    pub fn num_samples(&self) -> usize {
        if self.channels.is_empty() {
            0
        } else {
            self.channels[0].len()
        }
    }

    pub fn get(&self, channel: usize, sample: usize) -> &SampleType {
        &self.channels[channel][sample]
    }

    pub fn get_mut(&mut self, channel: usize, sample: usize) -> &mut SampleType {
        &mut self.channels[channel][sample]
    }

    pub fn set(&mut self, channel: usize, sample: usize, value: SampleType) {
        self.channels[channel][sample] = value;
    }

    pub fn is_empty(&self) -> bool {
        self.channels.is_empty() || self.channels[0].is_empty()
    }
}

impl<SampleType: Copy + Zero + Div<Output = SampleType> + AddAssign + From<f32>>
    AudioBuffer<SampleType>
{
    pub fn get_mono(&self, sample: usize) -> SampleType {
        let mut sum = SampleType::zero();
        for channel in &self.channels {
            sum += channel[sample];
        }
        sum / SampleType::from(self.channels.len() as f32)
    }
}

impl<SampleType: Clone + AddAssign> AudioBuffer<SampleType> {
    pub fn add(&mut self, other: &Self) {
        for (channel, other_channel) in self.channels.iter_mut().zip(other.channels.iter()) {
            for (sample, other_sample) in channel.iter_mut().zip(other_channel.iter()) {
                *sample += other_sample.clone();
            }
        }
    }
}

impl<SampleType, I, J> From<I> for AudioBuffer<SampleType>
where
    I: Iterator<Item = J>,
    Vec<SampleType>: From<J>,
{
    fn from(value: I) -> Self {
        AudioBuffer::new(value.map(Vec::from).collect())
    }
}

#[cfg(feature = "vst")]
pub mod vst;

/// Copy from an interleaved buffer into a target non-interleaved buffer.
pub fn copy_from_interleaved<SampleType: Clone>(
    source: &[SampleType],
    target: &mut [Vec<SampleType>],
) {
    if target.is_empty() {
        return;
    }

    let num_channels = target.len();
    for (sample_idx, frame) in source.chunks(num_channels).enumerate() {
        for (channel_idx, sample) in frame.iter().enumerate() {
            target[channel_idx][sample_idx] = sample.clone();
        }
    }
}

pub fn copy_into_interleaved<SampleType: Clone>(
    source: &[Vec<SampleType>],
    target: &mut [SampleType],
) {
    if source.is_empty() {
        return;
    }

    let num_channels = source.len();
    for (sample_idx, frame) in target.chunks_mut(num_channels).enumerate() {
        for (channel_idx, sample) in frame.iter_mut().enumerate() {
            *sample = source[channel_idx][sample_idx].clone();
        }
    }
}

pub fn to_interleaved<SampleType: Copy + num::Zero>(
    source: &[SampleType],
    num_channels: usize,
) -> Vec<Vec<SampleType>> {
    let mut result = vec![vec![SampleType::zero(); source.len() / num_channels]; num_channels];
    copy_from_interleaved(source, &mut result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_interleaved() {
        let source = vec![1.0, 2.0, 3.0, 4.0];
        let target = AudioBuffer::from_interleaved(2, &source);
        assert_eq!(target.channels, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn test_audio_buffer_copy_from_interleaved() {
        let source = vec![1.0, 2.0, 3.0, 4.0];
        let mut target = AudioBuffer::empty();
        target.resize(2, 2);
        target.copy_from_interleaved(&source);
    }

    #[test]
    fn test_audio_buffer_copy_into_interleaved() {
        let source = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
        let buffer = AudioBuffer::new(source);

        let mut target = vec![0.0; 4];
        buffer.copy_into_interleaved(&mut target);
        assert_eq!(target, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_copy_from_interleaved_empty() {
        let mut target: Vec<Vec<f32>> = vec![vec![0.0; 0]; 0];
        let source = vec![];
        copy_from_interleaved(&source, &mut target);
        assert_eq!(target, Vec::<Vec<f32>>::new());
    }

    #[test]
    fn test_copy_from_interleaved() {
        let mut target: Vec<Vec<f32>> = vec![vec![0.0; 2]; 2];
        let source = vec![1.0, 2.0, 3.0, 4.0];
        copy_from_interleaved(&source, &mut target);
        assert_eq!(target, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn test_to_interleaved() {
        let source = vec![1.0, 2.0, 3.0, 4.0];
        let result = to_interleaved(&source, 2);
        assert_eq!(result, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn test_copy_into_interleaved() {
        let source = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
        let mut target = vec![0.0, 0.0, 0.0, 0.0];
        copy_into_interleaved(&source, &mut target);
        assert_eq!(target, vec![1.0, 2.0, 3.0, 4.0]);
    }
}
