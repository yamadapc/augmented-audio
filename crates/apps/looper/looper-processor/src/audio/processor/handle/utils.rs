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
use audio_processor_traits::{AtomicF32, AudioBuffer};

use super::scratch_pad;

pub struct CopyLoopClipParams<'a> {
    pub scratch_pad: &'a scratch_pad::ScratchPad,
    pub start_cursor: usize,
    pub length: usize,
}

pub fn copy_looped_clip(params: CopyLoopClipParams, result_buffer: &mut AudioBuffer<AtomicF32>) {
    let buffer = params.scratch_pad.buffer();

    result_buffer.resize_with(buffer.num_channels(), params.length, || AtomicF32::new(0.0));

    for channel in 0..buffer.num_channels() {
        for i in 0..params.length {
            let index = (i + params.start_cursor) % buffer.num_samples();
            let sample = buffer.get(channel, index).get();
            result_buffer.get(channel, i).set(sample);
        }
    }
}

pub fn empty_buffer(channels: usize, samples: usize) -> AudioBuffer<AtomicF32> {
    let mut b = AudioBuffer::empty();
    b.resize_with(channels, samples, || AtomicF32::new(0.0));
    b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buffer = empty_buffer(2, 10);
        assert_eq!(buffer.num_channels(), 2);
        assert_eq!(buffer.num_samples(), 10);
        for sample in buffer.channel(0) {
            assert_eq!(sample.get(), 0.0)
        }
    }
}
