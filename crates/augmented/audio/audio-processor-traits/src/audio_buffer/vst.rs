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

use num::Float;

use super::{AudioBuffer, OwnedAudioBuffer, VecAudioBuffer};

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
        self.buffer.resize(2, num_samples, SampleType::zero());
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
