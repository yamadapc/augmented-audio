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

use crate::{AudioBuffer, InterleavedAudioBuffer, OwnedAudioBuffer};

/// An owned version of the interleaved buffer implementation. Can be converted onto an
/// `InterleavedAudioBuffer`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VecAudioBuffer<SampleType> {
    buffer: Vec<SampleType>,
    num_channels: usize,
    num_samples: usize,
}

impl<SampleType: Clone> VecAudioBuffer<SampleType> {
    pub fn new_with(buffer: Vec<SampleType>, num_channels: usize, num_samples: usize) -> Self {
        Self {
            buffer,
            num_samples,
            num_channels,
        }
    }

    pub fn empty_with(num_channels: usize, num_samples: usize, value: SampleType) -> Self {
        let mut result = Self::new();
        result.resize(num_channels, num_samples, value);
        result
    }
}

impl<SampleType: Clone> From<Vec<SampleType>> for VecAudioBuffer<SampleType> {
    fn from(simple_vec: Vec<SampleType>) -> Self {
        let num_samples = simple_vec.len();
        VecAudioBuffer::new_with(simple_vec, 1, num_samples)
    }
}

impl<SampleType> AudioBuffer for VecAudioBuffer<SampleType> {
    type SampleType = SampleType;

    #[inline]
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    #[inline]
    fn num_samples(&self) -> usize {
        self.num_samples
    }

    #[inline]
    fn slice(&self) -> &[Self::SampleType] {
        &self.buffer
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::SampleType] {
        &mut self.buffer
    }

    #[inline]
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
        &self.buffer[sample * self.num_channels + channel]
    }

    #[inline]
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        &mut self.buffer[sample * self.num_channels + channel]
    }

    #[inline]
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.buffer[sample * self.num_channels + channel] = value;
    }

    #[inline]
    unsafe fn get_unchecked(&self, channel: usize, sample: usize) -> &Self::SampleType {
        self.buffer
            .get_unchecked(sample * self.num_channels + channel)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.buffer
            .get_unchecked_mut(sample * self.num_channels + channel)
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        let sample = self
            .buffer
            .get_unchecked_mut(sample * self.num_channels + channel);
        *sample = value;
    }
}

impl<SampleType: Clone> OwnedAudioBuffer for VecAudioBuffer<SampleType> {
    #[inline]
    fn new() -> Self {
        VecAudioBuffer {
            num_channels: 0,
            num_samples: 0,
            buffer: Vec::new(),
        }
    }

    #[inline]
    fn resize(&mut self, num_channels: usize, num_samples: usize, sample: Self::SampleType) {
        self.num_samples = num_samples;
        self.num_channels = num_channels;
        self.buffer.resize(num_channels * num_samples, sample);
    }
}

impl<SampleType: Clone> VecAudioBuffer<SampleType> {
    pub fn new_with_size(num_channels: usize, num_samples: usize, sample: SampleType) -> Self {
        let mut buffer = Vec::with_capacity(num_samples * num_channels);
        buffer.resize(num_channels * num_samples, sample);
        VecAudioBuffer {
            num_channels,
            num_samples,
            buffer,
        }
    }

    /// Get an `InterleavedAudioBuffer` reference type out this `VecAudioBuffer`.
    pub fn interleaved(&mut self) -> InterleavedAudioBuffer<SampleType> {
        InterleavedAudioBuffer::new(self.num_channels, &mut self.buffer)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[test]
    fn test_vec_is_send() {
        let mut vec = VecAudioBuffer::new();
        vec.resize(2, 1000, 0.0);
        let handle = thread::spawn(move || std::mem::forget(vec));
        handle.join().unwrap();
    }

    #[test]
    fn test_vec_is_sync() {
        let mut vec = VecAudioBuffer::new();
        vec.resize(2, 1000, 0.0);
        let vec = Arc::new(vec);
        let vec_2 = vec.clone();
        let handle = thread::spawn(move || std::mem::forget(vec));
        std::mem::forget(vec_2);
        handle.join().unwrap();
    }
}
