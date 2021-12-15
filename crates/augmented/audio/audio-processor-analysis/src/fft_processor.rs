use std::f32::consts::PI;
use std::sync::Arc;

use rustfft::num_complex::Complex;
use rustfft::{Fft, FftDirection, FftPlanner};

use audio_processor_traits::simple_processor::SimpleAudioProcessor;

fn hann_window(size: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(size);
    let fsize = size as f32;
    for i in 0..size {
        let fi = i as f32;
        let value = 0.5 * (1.0 - (2.0 * PI * (fi / fsize)).cos());
        result.push(value);
    }
    result
}

pub struct FftProcessor {
    buffer: Vec<Complex<f32>>,
    scratch: Vec<Complex<f32>>,
    cursor: usize,
    window: Vec<f32>,
    size: usize,
    fft: Arc<dyn Fft<f32>>,
}

impl Default for FftProcessor {
    fn default() -> Self {
        Self::new(8192, FftDirection::Forward)
    }
}

impl FftProcessor {
    pub fn new(size: usize, direction: FftDirection) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(size, direction);

        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0.0.into());

        let scratch_size = fft.get_inplace_scratch_len();
        let mut scratch = Vec::with_capacity(scratch_size);
        scratch.resize(scratch_size, 0.0.into());

        let window = hann_window(size);

        Self {
            buffer,
            window,
            scratch,
            size,
            cursor: 0,
            fft,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn buffer(&self) -> &Vec<Complex<f32>> {
        &self.buffer
    }

    fn perform_fft(&mut self) {
        self.fft
            .process_with_scratch(&mut self.buffer, &mut self.scratch);
    }
}

impl SimpleAudioProcessor for FftProcessor {
    type SampleType = f32;

    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        self.buffer[self.cursor] = Complex::from_polar(sample * self.window[self.cursor], 0.0);
        self.cursor += 1;

        if self.cursor == self.buffer.len() {
            self.perform_fft();
            self.cursor = 0;
        }

        sample
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::{
        charts::draw_vec_chart, oscillator_buffer, relative_path, sine_generator,
    };

    use audio_processor_traits::audio_buffer::VecAudioBuffer;
    use audio_processor_traits::AudioProcessor;

    use super::*;

    #[test]
    fn test_draw_hann_window() {
        let window = hann_window(2048);
        draw_vec_chart(
            &*relative_path!("src/fft_processor.png"),
            "HannWindow",
            window,
        );
    }

    #[test]
    fn test_draw_fft() {
        println!("Generating signal");
        let signal = oscillator_buffer(44100.0, 440.0, Duration::from_millis(1000), sine_generator);
        let signal_len = signal.len();
        let mut signal = VecAudioBuffer::new_with(signal, 1, signal_len);

        println!("Processing");
        let mut fft_processor = FftProcessor::default();
        fft_processor.process(&mut signal);

        println!("Drawing chart");
        let mut output: Vec<f32> = fft_processor
            .buffer
            .iter()
            .map(|c| 20.0 * (c.norm() / 10.0).log10())
            .collect();
        output.reverse();
        let output: Vec<f32> = output.iter().take(1000).copied().collect();

        draw_vec_chart(
            &*relative_path!("src/fft_processor.png"),
            "FFT_SquareWave_440Hz",
            output,
        );
    }
}
