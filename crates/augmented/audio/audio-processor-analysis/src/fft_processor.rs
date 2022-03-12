use std::f32::consts::PI;
use std::sync::Arc;

use rustfft::num_complex::Complex;
pub use rustfft::FftDirection;
use rustfft::{Fft, FftPlanner};

use audio_processor_traits::simple_processor::SimpleAudioProcessor;

use crate::window_functions::{make_hann_vec, make_window_vec, WindowFunctionType};

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

pub struct FftProcessor {
    input_buffer: Vec<f32>,
    fft_buffer: Vec<Complex<f32>>,
    scratch: Vec<Complex<f32>>,
    cursor: usize,
    window: Vec<f32>,
    step_len: usize,
    size: usize,
    fft: Arc<dyn Fft<f32>>,
    has_changed: bool,
}

impl Default for FftProcessor {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl FftProcessor {
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
        input_buffer.resize(size, 0.0.into());
        let mut fft_buffer = Vec::with_capacity(size);
        fft_buffer.resize(size, 0.0.into());

        let scratch_size = fft.get_inplace_scratch_len();
        let mut scratch = Vec::with_capacity(scratch_size);
        scratch.resize(scratch_size, 0.0.into());

        let window = make_window_vec(size, window_function);
        let step_len = (size as f32 * (1.0 - overlap_ratio)) as usize;

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

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn buffer(&self) -> &Vec<Complex<f32>> {
        &self.fft_buffer
    }

    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    fn perform_fft(&mut self) {
        let start_idx = ((self.cursor as i32) - 1 - self.size as i32) as usize % self.size;
        for i in 0..self.size {
            let index = (start_idx + i) % self.size;
            let sample = self.input_buffer[index];

            let magnitude = sample * self.window[i];
            assert!(!magnitude.is_nan());
            let complex = Complex::from_polar(magnitude, 0.0);
            assert!(!complex.re.is_nan());
            assert!(!complex.im.is_nan());

            self.fft_buffer[i] = complex;
        }

        self.fft
            .process_with_scratch(&mut self.fft_buffer, &mut self.scratch);
    }
}

impl SimpleAudioProcessor for FftProcessor {
    type SampleType = f32;

    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        self.has_changed = false;
        self.input_buffer[self.cursor] = sample;
        self.cursor += 1;

        if self.cursor == self.step_len {
            self.perform_fft();
            self.cursor = 0;
            self.has_changed = true;
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
    use audio_processor_traits::simple_processor::process_buffer;

    use super::*;

    #[test]
    fn test_draw_fft() {
        println!("Generating signal");
        let signal = oscillator_buffer(44100.0, 440.0, Duration::from_millis(1000), sine_generator);
        let signal_len = signal.len();
        let mut signal = VecAudioBuffer::new_with(signal, 1, signal_len);

        println!("Processing");
        let mut fft_processor = FftProcessor::default();
        process_buffer(&mut fft_processor, &mut signal);

        println!("Drawing chart");
        let mut output: Vec<f32> = fft_processor
            .buffer()
            .iter()
            .map(|c| 20.0 * (c.norm() / 10.0).log10())
            .collect();
        output.reverse();
        let output: Vec<f32> = output.iter().take(1000).copied().collect();

        draw_vec_chart(
            &*relative_path!("src/fft_processor.png"),
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
