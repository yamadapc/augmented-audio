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

#[cfg(feature = "samplerate")]
pub use samplerate_impl::*;

#[cfg(all(feature = "rubato", not(feature = "samplerate")))]
pub use rubato_impl::*;

#[cfg(feature = "samplerate")]
mod samplerate_impl {
    pub type Decoder = samplerate::Samplerate;
    pub type DecoderError = samplerate::Error;

    pub fn make_decoder(
        input_rate: u32,
        output_rate: u32,
        channels: usize,
    ) -> Result<Decoder, DecoderError> {
        samplerate::Samplerate::new(
            samplerate::ConverterType::SincBestQuality,
            input_rate,
            output_rate,
            channels,
        )
    }

    pub fn process<T: AsRef<[f32]>>(
        decoder: &mut Decoder,
        block: &[T],
    ) -> Result<Vec<Vec<f32>>, DecoderError> {
        let num_channels = block.len();
        let num_samples = block[0].as_ref().len();

        let mut interleaved_buffer = vec![];
        interleaved_buffer.resize(num_channels * num_samples, 0.0);
        for sample in 0..num_samples {
            #[allow(clippy::needless_range_loop)]
            for channel in 0..num_channels {
                let index = sample * num_channels + channel;
                interleaved_buffer[index] = block[channel].as_ref()[sample];
            }
        }

        let result = decoder.process(&interleaved_buffer)?;

        let mut deinterleaved_buffer = vec![];
        deinterleaved_buffer.resize(num_channels, vec![]);
        for sample in 0..result.len() {
            let channel = sample % num_channels;
            deinterleaved_buffer[channel].push(result[sample]);
        }
        Ok(deinterleaved_buffer)
    }
}

pub const BLOCK_SIZE: usize = 1024;

#[cfg(feature = "rubato")]
#[allow(unused)]
pub mod rubato_impl {
    use crate::file_io::sample_rate_converter::BLOCK_SIZE;
    use rubato::Resampler;

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
}
