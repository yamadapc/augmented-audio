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

/// Set all samples of an AudioBuffer to a constant
pub fn set_all<SampleType>(buf: &mut AudioBuffer<SampleType>, value: SampleType)
where
    SampleType: Clone,
{
    for sample in buf.slice_mut() {
        *sample = value.clone();
    }
}

/// Set all samples of an AudioBuffer to Zero::zero
pub fn clear<SampleType>(buf: &mut AudioBuffer<SampleType>)
where
    SampleType: num::Zero + Copy,
{
    for sample in buf.slice_mut() {
        *sample = SampleType::zero();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_all() {
        let mut buffer = AudioBuffer::from_interleaved(1, &[1.0, 2.0, 3.0, 4.0]);
        set_all(&mut buffer, 4.0);
        assert_eq!(buffer.channel(0), &[4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_clear() {
        let mut buffer = AudioBuffer::from_interleaved(1, &[1.0, 2.0, 3.0, 4.0]);
        clear(&mut buffer);
        assert_eq!(buffer.channel(0), &[0.0, 0.0, 0.0, 0.0]);
    }
}
