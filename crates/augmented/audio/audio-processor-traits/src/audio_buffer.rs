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
use std::slice::{Chunks, ChunksMut};

use num::Float;

/// Represents an audio buffer. This decouples audio processing code from a certain representation
/// of multi-channel sample buffers.
///
/// This crate provides implementations of this trait for CPal style buffers, which use interleaved
/// internal representation.
///
/// When processing samples, it'll be more efficient to use `.slice` and `.slice_mut` than `.get` /
/// `.set` methods. For the VST buffer, these methods will not work.
///
/// It's recommended to convert the buffer into interleaved layout before processing as that will be
/// around as expensive as the overhead of `get`/`set` methods on a single loop through samples.
///
/// (due to bounds checking and other compiler optimisations that fail with them)
pub trait AudioBuffer {
    /// The type of samples within this buffer.
    type SampleType;

    /// The number of channels in this buffer
    fn num_channels(&self) -> usize;

    /// The number of samples in this buffer
    fn num_samples(&self) -> usize;

    /// Get a slice to the internal data. Will not work with VST adapter
    ///
    /// This is the faster way to process
    fn slice(&self) -> &[Self::SampleType];

    /// Get a mutable slice to the internal data. Will not work with VST adapter
    ///
    /// This is the faster way to process
    fn slice_mut(&mut self) -> &mut [Self::SampleType];

    /// Shortcut for `.slice().chunks(num_channels)`
    fn frames(&self) -> Chunks<'_, Self::SampleType> {
        self.slice().chunks(self.num_channels())
    }

    /// Shortcut for `.slice_mut().chunks_mut(num_channels)`
    ///
    /// This is a frame representing a sample in time, for all
    /// channels.
    fn frames_mut(&mut self) -> ChunksMut<'_, Self::SampleType> {
        let channels = self.num_channels();
        self.slice_mut().chunks_mut(channels)
    }

    /// Get a ref to an INPUT sample in this buffer.
    ///
    /// Calling this on a loop will be ~20x slower than reading from `slice`.
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;

    /// Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    ///
    /// Calling this on a loop will be ~20x slower than reading from `slice`.
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;

    /// Set an OUTPUT sample in this buffer
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);

    /// Unsafe, no bounds check - Get a ref to an INPUT sample in this buffer
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    unsafe fn get_unchecked(&self, channel: usize, sample: usize) -> &Self::SampleType {
        self.get(channel, sample)
    }

    /// Unsafe, no bounds check - Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.get_mut(channel, sample)
    }

    /// Unsafe, no bounds check - Set an OUTPUT sample in this buffer
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.set(channel, sample, value)
    }
}

/// Set all samples of an AudioBuffer to a constant
pub fn set_all<Buffer, SampleType>(buf: &mut Buffer, value: SampleType)
where
    Buffer: AudioBuffer<SampleType = SampleType>,
    SampleType: Clone,
{
    for sample in buf.slice_mut() {
        *sample = value.clone();
    }
}

/// Set all samples of an AudioBuffer to Zero::zero
pub fn clear<Buffer, SampleType>(buf: &mut Buffer)
where
    Buffer: AudioBuffer<SampleType = SampleType>,
    SampleType: num::Zero,
{
    for sample in buf.slice_mut() {
        *sample = SampleType::zero();
    }
}

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

/// A trait for buffer types that own the data they hold & can be constructed / resized.
pub trait OwnedAudioBuffer: AudioBuffer {
    /// Create an empty buffer of this type
    fn new() -> Self;
    /// Resize the buffer to fit `num_channels` and `num_samples`
    fn resize(&mut self, num_channels: usize, num_samples: usize, sample: Self::SampleType);
}

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

impl<SampleType> VecAudioBuffer<SampleType> {
    /// Get an `InterleavedAudioBuffer` reference type out this `VecAudioBuffer`.
    pub fn interleaved(&mut self) -> InterleavedAudioBuffer<SampleType> {
        InterleavedAudioBuffer::new(self.num_channels, &mut self.buffer)
    }
}

pub mod vst {
    use super::{AudioBuffer, Float, OwnedAudioBuffer, VecAudioBuffer};

    pub struct VSTBufferHandler<SampleType> {
        buffer: VecAudioBuffer<SampleType>,
    }

    impl<SampleType: Float> Default for VSTBufferHandler<SampleType> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<SampleType: Float> VSTBufferHandler<SampleType> {
        pub fn new() -> Self {
            Self {
                buffer: VecAudioBuffer::new(),
            }
        }

        pub fn set_block_size(&mut self, block_size: usize) {
            self.buffer.resize(2, block_size, SampleType::zero());
        }

        pub fn with_buffer<F>(&mut self, buffer: &mut vst::buffer::AudioBuffer<SampleType>, f: F)
        where
            F: FnOnce(&mut VecAudioBuffer<SampleType>),
        {
            let num_samples = buffer.samples();
            let (inputs, mut outputs) = buffer.split();
            self.buffer
                .resize(2, num_samples as usize, SampleType::zero());
            {
                let buffer_slice = self.buffer.slice_mut();
                for (channel, input) in inputs.into_iter().take(2).enumerate() {
                    for (index, sample) in input.iter().enumerate() {
                        buffer_slice[index * 2 + channel] = *sample;
                    }
                }
            }

            f(&mut self.buffer);

            {
                let buffer_slice = self.buffer.slice();
                for (channel, output) in outputs.into_iter().take(2).enumerate() {
                    for (index, sample) in output.iter_mut().enumerate() {
                        *sample = buffer_slice[index * 2 + channel];
                    }
                }
            }
        }
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
        let handle = thread::spawn(move || println!("HELLO VEC {:?}", vec));
        handle.join().unwrap();
    }

    #[test]
    fn test_vec_is_sync() {
        let mut vec = VecAudioBuffer::new();
        vec.resize(2, 1000, 0.0);
        let vec = Arc::new(vec);
        let vec_2 = vec.clone();
        let handle = thread::spawn(move || println!("HELLO VEC {:?}", vec));
        println!("HELLO VEC {:?}", vec_2);
        handle.join().unwrap();
    }
}
