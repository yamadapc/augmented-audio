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

use rubato::Resampler;

use super::BLOCK_SIZE;

pub type Decoder = rubato::FftFixedIn<f32>;
pub type DecoderError = rubato::ResampleError;
pub type DecoderCreateError = rubato::ResamplerConstructionError;

pub fn make_decoder(
    input_rate: u32,
    output_rate: u32,
    channels: usize,
) -> Result<Decoder, DecoderCreateError> {
    Decoder::new(
        input_rate as usize,
        output_rate as usize,
        BLOCK_SIZE,
        2,
        channels,
    )
}

pub fn process<T: AsRef<[f32]>>(
    decoder: &mut Decoder,
    block: &[T],
) -> Result<Vec<Vec<f32>>, DecoderError> {
    decoder.process(block, None)
}
