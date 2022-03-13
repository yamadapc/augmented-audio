use std::f32::consts::PI;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_analysis::fft_processor::{FftDirection, FftProcessor, FftProcessorOptions};
use audio_processor_analysis::window_functions::{
    blackman_harris, hann, make_hann_vec, make_window_vec, WindowFunctionType,
};
use audio_processor_traits::num::Complex;
use audio_processor_traits::{
    simple_processor, AudioBuffer, AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor,
    Zero,
};

fn make_vec(size: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(size);
    v.resize(size, 0.0);
    v
}

struct PhaseCorrectionParams<'a> {
    /// The hop size of the FFT
    step_len: usize,
    fft_frequency_domain: &'a mut [Complex<f32>],
    pitch_shift_ratio: f32,
}

struct NormalPhaseVocoder {
    last_output_phase: Vec<f32>,
    last_input_phase: Vec<f32>,
}

impl NormalPhaseVocoder {
    fn new(fft_size: usize) -> Self {
        Self {
            last_output_phase: make_vec(fft_size),
            last_input_phase: make_vec(fft_size),
        }
    }

    fn update_phases(&mut self, params: PhaseCorrectionParams) {
        let PhaseCorrectionParams {
            step_len,
            fft_frequency_domain,
            pitch_shift_ratio,
        } = params;

        let step_len = step_len as f32;
        let fft_size = fft_frequency_domain.len() as f32;
        for (bin, value) in fft_frequency_domain.iter_mut().enumerate() {
            if bin <= (fft_size as usize) / 2 {
                let value: &mut Complex<f32> = value;
                let (magnitude, phase) = value.to_polar();

                let bin_frequency = 2.0 * PI * bin as f32 / fft_size;
                let expected_bin_phase = bin_frequency * step_len;
                let phase_delta = phase - self.last_input_phase[bin];
                let bin_deviation = phase_delta - expected_bin_phase;
                let bin_frequency = expected_bin_phase + princ_arg(bin_deviation);

                let last_output_phase = self.last_output_phase[bin];
                let new_phase =
                    princ_arg(last_output_phase + bin_frequency * pitch_shift_ratio * step_len);

                *value = Complex::from_polar(magnitude, new_phase);

                self.last_output_phase[bin] = new_phase;
                self.last_input_phase[bin] = phase;
            } else {
                *value = Complex::new(0.0, 0.0);
            }
        }
    }
}

struct PhaseLockedVocoder {
    last_output_phase: Vec<f32>,
    last_input_phase: Vec<f32>,
    scratch: Vec<Complex<f32>>,
}

impl PhaseLockedVocoder {
    fn new(fft_size: usize) -> Self {
        Self {
            last_output_phase: make_vec(fft_size),
            last_input_phase: make_vec(fft_size),
            scratch: make_vec(fft_size).iter().map(|_| Complex::zero()).collect(),
        }
    }

    fn update_phases(&mut self, params: PhaseCorrectionParams) {
        let PhaseCorrectionParams {
            step_len,
            fft_frequency_domain,
            pitch_shift_ratio,
        } = params;

        let step_len = step_len as f32;
        let fft_size = fft_frequency_domain.len() as f32;
        for bin in 0..fft_frequency_domain.len() {
            if bin <= (fft_size as usize) / 2 {
                let value = fft_frequency_domain[bin];
                let (magnitude, partial_phase) = value.to_polar();
                let phase = if bin == 0 {
                    partial_phase
                } else {
                    let v1 = value;
                    let v2 = fft_frequency_domain[bin - 1];
                    let v3 = fft_frequency_domain[bin + 1];
                    let (_, phase) = (v1 + v2 + v3).to_polar();
                    phase
                };

                let bin_frequency = 2.0 * PI * bin as f32 / fft_size;
                let expected_bin_phase = bin_frequency * step_len;
                let phase_delta = phase - self.last_input_phase[bin];
                let bin_deviation = phase_delta - expected_bin_phase;
                let bin_frequency = expected_bin_phase + princ_arg(bin_deviation);

                let last_output_phase = self.last_output_phase[bin];
                let new_phase =
                    princ_arg(last_output_phase + bin_frequency * pitch_shift_ratio * step_len);

                self.last_input_phase[bin] = phase;
                self.last_output_phase[bin] = new_phase;
                self.scratch[bin] = Complex::from_polar(magnitude, new_phase);
            } else {
                fft_frequency_domain[bin] = Complex::new(0.0, 0.0);
            }
        }

        for bin in 0..self.scratch.len() {
            fft_frequency_domain[bin] = self.scratch[bin];
        }
    }
}

enum PhaseProcessingStrategy {
    Normal(NormalPhaseVocoder),
    PhaseLocking(PhaseLockedVocoder),
}

struct PitchShifterProcessorHandle {
    shift_steps: f32,
}

struct PitchShifterProcessor {
    pitch_shift_ratio: f32,
    resample_buffer: Vec<f32>,
    resample_buffer_size: usize,
    output_buffer: Vec<f32>,
    output_read_cursor: usize,
    output_write_cursor: usize,
    phase_processing_strategy: PhaseProcessingStrategy,
    fft_processor: FftProcessor,
    inverse_fft_processor: FftProcessor,
    handle: Shared<PitchShifterProcessorHandle>,
}

impl Default for PitchShifterProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl PitchShifterProcessor {
    fn new() -> Self {
        let fft_size = 8192;
        let fft_processor = FftProcessor::new(FftProcessorOptions {
            size: fft_size,
            overlap_ratio: 0.875,
            window_function: WindowFunctionType::Hann,
            ..Default::default()
        });
        let step_len = fft_processor.step_len() as f32;
        let pitch_shift_ratio = 2.0;
        let resample_buffer_size = fft_size as f32 / pitch_shift_ratio;
        let resample_buffer_size = ((resample_buffer_size * step_len).round() / step_len) as usize;

        Self {
            pitch_shift_ratio,
            resample_buffer: make_vec(fft_size),
            resample_buffer_size,
            output_buffer: make_vec(fft_size),
            output_read_cursor: 0,
            output_write_cursor: 0,
            fft_processor,
            inverse_fft_processor: FftProcessor::new(FftProcessorOptions {
                size: fft_size,
                direction: FftDirection::Inverse,
                ..Default::default()
            }),
            phase_processing_strategy: PhaseProcessingStrategy::PhaseLocking(
                PhaseLockedVocoder::new(fft_size),
            ),
            handle: make_shared(PitchShifterProcessorHandle { shift_steps: 12.0 }),
        }
    }

    fn on_fft_frame(&mut self) {
        self.update_phases();
        self.inverse_fft_processor
            .process_fft_buffer(self.fft_processor.buffer_mut());

        self.resample_fft();
        // Read resampled output into output buffer, apply Hann window here
        let fft_size = self.fft_processor.buffer().len();
        for i in 0..fft_size {
            let resample_idx = i % self.resample_buffer_size;
            let s = self.resample_buffer[resample_idx];
            let output_idx = (self.output_write_cursor + i) % self.output_buffer.len();
            assert!(!s.is_nan());
            self.output_buffer[output_idx] += s * hann(i as f32, fft_size as f32);
        }

        self.output_write_cursor =
            (self.output_write_cursor + self.fft_processor.step_len()) % self.output_buffer.len();
    }

    fn update_phases(&mut self) {
        let params = PhaseCorrectionParams {
            step_len: self.fft_processor.step_len(),
            fft_frequency_domain: self.fft_processor.buffer_mut(),
            pitch_shift_ratio: self.pitch_shift_ratio,
        };
        match &mut self.phase_processing_strategy {
            PhaseProcessingStrategy::Normal(normal) => normal.update_phases(params),
            PhaseProcessingStrategy::PhaseLocking(strategy) => strategy.update_phases(params),
        }
    }

    fn resample_fft(&mut self) {
        let fft_time_domain = self.fft_processor.buffer();

        let multiplier = 0.03 / 200.0;
        let ratio = fft_time_domain.len() as f32 / self.resample_buffer_size as f32;
        for i in 0..self.resample_buffer_size {
            let fft_index = i as f32 * ratio;
            let fft_index_floor = fft_index.floor();
            let delta = fft_index - fft_index_floor;
            let sample1 = fft_time_domain[fft_index_floor as usize].re;
            let sample2 = fft_time_domain
                .get(((fft_index_floor + 1.0) as usize))
                .map(|c| c.re)
                .unwrap_or(0.0);
            let sample = sample1 + delta * (sample2 - sample1);
            assert!(!sample.is_nan());
            assert!(!(sample * multiplier).is_nan());
            self.resample_buffer[i] = sample * multiplier;
        }
    }
}

fn princ_arg(phase: f32) -> f32 {
    if phase >= 0.0 {
        (phase + PI) % (2.0 * PI) - PI
    } else {
        (phase + PI) % (-2.0 * PI) + PI
    }
}

impl AudioProcessor for PitchShifterProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.fft_processor.s_prepare(settings);
        self.inverse_fft_processor.s_prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            let output = self.output_buffer[self.output_read_cursor];
            self.output_buffer[self.output_read_cursor] = 0.0;
            self.output_read_cursor = (self.output_read_cursor + 1) % self.output_buffer.len();

            self.fft_processor.s_process_frame(frame);
            if self.fft_processor.has_changed() {
                self.on_fft_frame();
            }

            for channel in frame.iter_mut() {
                *channel = output;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::process::Output;

    use audio_processor_testing_helpers::{relative_path, rms_level};

    use audio_processor_file::{AudioFileProcessor, OutputAudioFileProcessor, OutputFileSettings};
    use audio_processor_traits::{
        AudioBuffer, AudioProcessorSettings, OwnedAudioBuffer, VecAudioBuffer,
    };

    use super::*;

    /// Read an input file for testing
    fn read_input_file(input_file_path: &str) -> impl AudioBuffer<SampleType = f32> {
        let settings = AudioProcessorSettings::default();
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            input_file_path,
        )
        .unwrap();
        input.prepare(settings);
        let input_buffer = input.buffer();
        let mut buffer = VecAudioBuffer::new();

        // We read at most 10s of audio & mono it.
        let max_len = (settings.sample_rate() * 10.0) as usize;
        buffer.resize(1, input_buffer[0].len().min(max_len), 0.0);
        let channel = &input_buffer[0];
        for (sample_index, sample) in channel.iter().enumerate().take(max_len) {
            buffer.set(0, sample_index, *sample + buffer.get(0, sample_index));
        }
        buffer
    }

    #[test]
    fn test_pitch_shift_12steps() {
        let input_path = relative_path!("../../../../input-files/bass.mp3");
        // let input_path = relative_path!("../../../../input-files/1sec-sine.mp3");
        // let input_path = relative_path!("../../../confirmation.mp3");
        let transients_file_path = format!("{}.transients.wav", input_path);
        let mut input = read_input_file(&input_path);
        let input_rms = rms_level(input.slice());

        let mut pitch_shifter = PitchShifterProcessor::default();
        pitch_shifter.prepare(AudioProcessorSettings::default());
        pitch_shifter.process(&mut input);
        let output_rms = rms_level(input.slice());
        let diff = (input_rms - output_rms).abs();
        println!("diff={} input={} output={}", diff, input_rms, output_rms);
        assert!(diff.abs() < 0.1);

        let output_path = relative_path!("./output_test.wav");
        let mut output_file_processor =
            OutputAudioFileProcessor::from_path(AudioProcessorSettings::default(), &output_path);
        output_file_processor.prepare(AudioProcessorSettings::default());
        let mut samples: Vec<f32> = input
            .slice()
            .iter()
            .cloned()
            .flat_map(|sample| [sample, sample])
            .collect();
        output_file_processor.process(&mut samples);
    }
}
