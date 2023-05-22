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

//! FFT processor implementation with windowing & overlap, wraps `rustfft`.
//!
//! `rustfft` audio-processor, forwards or backwards, real-time safe, FFT.
//!
//! Applies a Hann window by default. Several window functions are exported by [`audio_processor_analysis::window_functions`].
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/window_functions/windows--HannWindow.png)
//!
//! Then performs FFT with N bins.
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/fft_processor.png--FFT_sine_440Hz.png)
//!
//! Overlap is configurable
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/screen.png)

use std::sync::Arc;

use rustfft::num_complex::Complex;
pub use rustfft::FftDirection;
use rustfft::{Fft, FftNum, FftPlanner};

use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioContext, Float};

use crate::window_functions::{make_window_vec, WindowFunctionType};

pub struct FftProcessorOptions {
    pub size: usize,
    pub direction: FftDirection,
    pub overlap_ratio: f32,
    pub window_function: WindowFunctionType,
}

impl Default for FftProcessorOptions {
    fn default() -> Self {
        Self {
            size: 8192,
            direction: FftDirection::Forward,
            overlap_ratio: 0.0,
            window_function: WindowFunctionType::Hann,
        }
    }
}

/// Default f32 FFT processor
pub type FftProcessor = FftProcessorImpl<f32>;

/// An FFT processor with overlap and windowing.
///
/// This processor will collect samples onto a circular buffer and perform FFTs whenever hop size is
/// reached.
pub struct FftProcessorImpl<ST> {
    input_buffer: Vec<ST>,
    fft_buffer: Vec<Complex<ST>>,
    scratch: Vec<Complex<ST>>,
    cursor: usize,
    window: Vec<ST>,
    step_len: usize,
    size: usize,
    fft: Arc<dyn Fft<ST>>,
    has_changed: bool,
}

impl<ST: FftNum + std::iter::Sum + Float> Default for FftProcessorImpl<ST> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<ST: FftNum + std::iter::Sum + Float> FftProcessorImpl<ST> {
    /// Constructs a new `FftProcessor`
    ///
    /// * size: Size of the FFT
    /// * direction: Direction of the FFT
    /// * overlap_ratio: 0.0 will do no overlap, 0.5 will do half a window of overlap and 0.75 will
    ///   do 3/4 window overlap
    /// * window_function: The window function to use
    pub fn new(options: FftProcessorOptions) -> Self {
        let FftProcessorOptions {
            size,
            direction,
            overlap_ratio,
            window_function,
        } = options;
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(size, direction);

        let mut input_buffer = Vec::with_capacity(size);
        input_buffer.resize(size, ST::zero());
        let mut fft_buffer = Vec::with_capacity(size);
        fft_buffer.resize(size, ST::zero().into());

        let scratch_size = fft.get_inplace_scratch_len();
        let mut scratch = Vec::with_capacity(scratch_size);
        scratch.resize(scratch_size, ST::zero().into());

        let window = make_window_vec(size, window_function);
        let step_len = Self::calculate_hop_size(size, overlap_ratio);

        Self {
            input_buffer,
            fft_buffer,
            window,
            scratch,
            size,
            step_len,
            cursor: 0,
            fft,
            has_changed: false,
        }
    }

    fn calculate_hop_size(size: usize, overlap_ratio: f32) -> usize {
        (size as f32 * (1.0 - overlap_ratio)) as usize
    }

    /// The number of frequency bins this FFT processor operates with
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get a reference to the FFT bins buffer
    pub fn buffer(&self) -> &Vec<Complex<ST>> {
        &self.fft_buffer
    }

    /// Get a reference to the rustfft instance
    pub fn fft(&self) -> &Arc<dyn Fft<ST>> {
        &self.fft
    }

    /// Get a mutable reference to the FFT bins buffer
    pub fn buffer_mut(&mut self) -> &mut Vec<Complex<ST>> {
        &mut self.fft_buffer
    }

    /// Get a mutable reference to the scratch buffer
    pub fn scratch_mut(&mut self) -> &mut Vec<Complex<ST>> {
        &mut self.scratch
    }

    /// Get the hop size of this processor. This is the number of samples between each FFT.
    pub fn step_len(&self) -> usize {
        self.step_len
    }

    /// Manually process an external FFT buffer in-place.
    pub fn process_fft_buffer(&mut self, samples: &mut [Complex<ST>]) {
        self.fft.process_with_scratch(samples, &mut self.scratch);
    }

    /// Returns true if an FFT has just been performed on the last call to `s_process`
    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    /// Returns the sum of the power of the current input buffer window.
    pub fn input_buffer_sum(&self) -> ST {
        self.input_buffer.iter().map(|f| f.abs()).sum()
    }

    /// Manually perform an FFT; offset the input buffer by a certain index.
    #[inline]
    pub fn perform_fft(&mut self, start_idx: usize) {
        for i in 0..self.size {
            let index = (start_idx + i) % self.size;
            let sample = self.input_buffer[index];

            let magnitude = sample * self.window[i];
            assert!(!magnitude.is_nan());
            let complex = Complex::new(magnitude, ST::zero());
            assert!(!complex.re.is_nan());
            assert!(!complex.im.is_nan());

            self.fft_buffer[i] = complex;
        }

        self.fft
            .process_with_scratch(&mut self.fft_buffer, &mut self.scratch);
    }
}

impl<ST: FftNum + Float + std::iter::Sum> MonoAudioProcessor for FftProcessorImpl<ST> {
    type SampleType = ST;

    #[inline]
    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        sample: Self::SampleType,
    ) -> Self::SampleType {
        self.has_changed = false;
        self.input_buffer[self.cursor] = sample;

        if self.cursor % self.step_len == 0 {
            // Offset FFT so it's reading from the input buffer at the start of this window
            let start_idx = (self.cursor as i32 - self.size as i32) as usize % self.size;
            self.perform_fft(start_idx);
            self.has_changed = true;
        }

        self.cursor = (self.cursor + 1) % self.size;

        sample
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::{
        charts::draw_vec_chart, oscillator_buffer, relative_path, sine_generator,
    };

    use audio_processor_traits::simple_processor::process_buffer;
    use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};

    use super::*;

    #[test]
    fn test_hop_size_is_correct() {
        let hop_size = FftProcessor::calculate_hop_size(2048, 0.75);
        assert_eq!(hop_size, 512);
        let hop_size = FftProcessor::calculate_hop_size(2048, 0.875);
        assert_eq!(hop_size, 256);
    }

    #[test]
    fn test_draw_fft() {
        println!("Generating signal");
        let signal = oscillator_buffer(44100.0, 440.0, Duration::from_millis(1000), sine_generator);
        let mut context = AudioContext::from(AudioProcessorSettings::new(44100.0, 1, 1, 512));
        let mut signal = AudioBuffer::from_interleaved(1, &signal);

        println!("Processing");
        let mut fft_processor = FftProcessor::default();
        process_buffer(&mut context, &mut fft_processor, &mut signal);

        println!("Drawing chart");
        let mut output: Vec<f32> = fft_processor
            .buffer()
            .iter()
            .map(|c| 20.0 * (c.norm() / 10.0).log10())
            .collect();
        output.reverse();
        let output: Vec<f32> = output.iter().take(1000).copied().collect();

        draw_vec_chart(
            &relative_path!("src/fft_processor.png"),
            "FFT_sine_440Hz",
            output,
        );
    }

    #[test]
    fn test_usize_cast() {
        let i = -1;
        let i = i as usize % 2;
        assert_eq!(i, 1)
    }
}
