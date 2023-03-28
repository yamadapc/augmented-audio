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

use crate::AudioBuffer;

/// An AudioBuffer that stores samples as interleaved frames, used for [`cpal`] compatibility.
///
/// Example layout:
///
/// [
///   0, 0, // <- left_sample, right_sample,
///   ...,
/// ]
pub struct InterleavedAudioBuffer<'a, SampleType> {
    num_channels: usize,
    inner: &'a mut [SampleType],
}

impl<'a, SampleType> InterleavedAudioBuffer<'a, SampleType> {
    pub fn new(num_channels: usize, inner: &'a mut [SampleType]) -> Self {
        Self {
            num_channels,
            inner,
        }
    }
}

impl<'a, SampleType> AudioBuffer for InterleavedAudioBuffer<'a, SampleType> {
    type SampleType = SampleType;

    #[inline]
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    #[inline]
    fn num_samples(&self) -> usize {
        self.inner.len() / self.num_channels
    }

    #[inline]
    fn slice(&self) -> &[Self::SampleType] {
        self.inner
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::SampleType] {
        self.inner
    }

    #[inline]
    fn get(&self, channel: usize, sample: usize) -> &SampleType {
        &self.inner[sample * self.num_channels + channel]
    }

    #[inline]
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut SampleType {
        &mut self.inner[sample * self.num_channels + channel]
    }

    #[inline]
    fn set(&mut self, channel: usize, sample: usize, value: SampleType) {
        let sample_ref = self.get_mut(channel, sample);
        *sample_ref = value;
    }
}
