use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use augmented_audio_volume::{Amplitude, Decibels};
use handle::CompressorHandle;

mod handle {
    use audio_processor_traits::AtomicF32;

    pub fn calculate_multiplier(sample_rate: f32, duration_ms: f32) -> f32 {
        let attack_secs = duration_ms * 0.001;
        let attack_samples = sample_rate * attack_secs;
        (-1.0 / attack_samples).exp2()
    }

    pub struct CompressorHandle {
        make_up_gain_db: AtomicF32,
        knee_width_db: AtomicF32,
        threshold_db: AtomicF32,
        ratio: AtomicF32,
        attack_ms: AtomicF32,
        release_ms: AtomicF32,
        sample_rate: AtomicF32,
    }

    impl Default for CompressorHandle {
        fn default() -> Self {
            Self {
                make_up_gain_db: AtomicF32::new(0.0),
                knee_width_db: AtomicF32::new(1.0),
                threshold_db: AtomicF32::new(-10.0),
                ratio: AtomicF32::new(2.0),
                attack_ms: AtomicF32::new(3.0),
                release_ms: AtomicF32::new(10.0),
                sample_rate: AtomicF32::new(44100.0),
            }
        }
    }

    impl CompressorHandle {
        pub fn attack_mult(&self) -> f32 {
            calculate_multiplier(self.sample_rate.get(), self.attack_ms.get())
        }

        pub fn release_mult(&self) -> f32 {
            calculate_multiplier(self.sample_rate.get(), self.release_ms.get())
        }

        pub fn set_attack_ms(&self, value: f32) {
            self.attack_ms.set(value);
        }

        pub fn set_release_ms(&self, value: f32) {
            self.release_ms.set(value);
        }

        pub fn set_sample_rate(&self, sample_rate: f32) {
            self.sample_rate.set(sample_rate);
        }

        pub fn set_threshold(&self, threshold: f32) {
            self.threshold_db.set(threshold)
        }

        pub fn ratio(&self) -> f32 {
            self.ratio.get()
        }

        pub fn threshold(&self) -> f32 {
            self.threshold_db.get()
        }

        pub fn knee_width(&self) -> f32 {
            self.knee_width_db.get()
        }
    }
}

pub struct CompressorProcessor {
    peak_detector_state: PeakDetector,
    handle: Shared<CompressorHandle>,
}

impl CompressorProcessor {
    pub fn new() -> Self {
        Self {
            peak_detector_state: PeakDetector::default(),
            handle: make_shared(CompressorHandle::default()),
        }
    }
}

impl AudioProcessor for CompressorProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.handle.set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            self.peak_detector_state.accept_frame(
                self.handle.attack_mult(),
                self.handle.release_mult(),
                frame,
            );

            self.apply_gain(frame);
        }
    }
}

impl CompressorProcessor {
    fn apply_gain(&self, frame: &mut [f32]) {
        let gain = self.compute_gain();
        for sample in frame {
            *sample = *sample * gain;
        }
    }

    fn compute_gain(&self) -> f32 {
        let power = self.peak_detector_state.power;
        let level = self.peak_detector_state.value;
        let ratio = self.handle.ratio();
        let threshold = Amplitude::from_db(self.handle.threshold(), 1.0).as_amplitude();
        let width = Amplitude::from_db(self.handle.knee_width(), 1.0).as_amplitude();

        let delta = level - threshold;
        let output = if (2.0 * delta) < -width {
            power
        } else if (2.0 * delta.abs()) <= width {
            power + (1.0 / ratio - 1.0) * (delta + width / 2.0).powf(2.0) / 2.0 * width
        } else {
            threshold + (power - threshold) / ratio
        };

        (output / power).max(0.0)
    }
}

struct PeakDetector {
    value: f32,
    power: f32,
}

impl Default for PeakDetector {
    fn default() -> Self {
        Self {
            value: 0.0.into(),
            power: 0.0.into(),
        }
    }
}

impl PeakDetector {
    fn accept_frame(&mut self, attack_mult: f32, release_mult: f32, frame: &[f32]) {
        let frame_len = frame.len() as f32;
        let new: f32 = frame.into_iter().map(|f| f.abs()).sum::<f32>() / frame_len;
        let old: f32 = self.value;
        self.power = new;
        self.value = release_mult * old + (1.0 - attack_mult) * (new - old).max(0.0);
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::charts::{
        draw_multi_vec_charts, draw_vec_chart, BLUE, RED, YELLOW,
    };
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{audio_buffer, OwnedAudioBuffer, VecAudioBuffer};

    use super::*;

    #[test]
    fn test_peak_detector() {
        let mut peak = PeakDetector::default();
        peak.accept_frame(0.01, 0.02, &[1.0, 1.0]);
        assert!(peak.value > 0.0);
    }

    #[test]
    fn test_create_compressor() {
        let _ = CompressorProcessor::new();
    }

    #[test]
    fn test_peak_detector_output() {
        let output_path = relative_path!("src/peak-detector");
        let settings = AudioProcessorSettings::default();
        let mut input = setup_input_processor(settings);
        let mut processor = PeakDetector::default();
        let attack_multi = handle::calculate_multiplier(settings.sample_rate(), 10.0);
        let release_mult = handle::calculate_multiplier(settings.sample_rate(), 30.0);

        let mut input_vec = vec![];
        let mut output_vec = vec![];
        let mut power_vec = vec![];
        {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(2, settings.block_size(), 0.0);
            let num_chunks = (input.num_samples() / 8) / settings.block_size();
            for _chunk in 0..num_chunks {
                audio_buffer::clear(&mut buffer);
                input.process(&mut buffer);
                for frame in buffer.frames() {
                    input_vec.push(average(frame));
                    processor.accept_frame(attack_multi, release_mult, frame);
                    output_vec.push(processor.value * 2.0);
                    power_vec.push(processor.power);
                }
            }
        }

        draw_multi_vec_charts(
            &output_path,
            "Peak Detector",
            vec![(RED, input_vec), (YELLOW, power_vec), (BLUE, output_vec)],
        );
    }

    #[test]
    fn test_compress_synth_loop() {
        let output_path = relative_path!("src/compressor");
        let settings = AudioProcessorSettings::default();
        let mut input = setup_input_processor(settings);
        let mut processor = CompressorProcessor::new();
        processor.prepare(settings);
        processor.handle.set_threshold(-20.0);
        processor.handle.set_attack_ms(2.0);

        let mut input_vec = vec![];
        let mut output_vec = vec![];
        {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, settings.block_size(), 0.0);
            let num_chunks = (input.num_samples() / 8) / settings.block_size();
            for _chunk in 0..num_chunks {
                audio_buffer::clear(&mut buffer);
                input.process(&mut buffer);
                for frame in buffer.frames() {
                    input_vec.push(average(frame))
                }

                processor.process(&mut buffer);
                for frame in buffer.frames() {
                    output_vec.push(average(frame))
                }
            }
        }

        draw_vec_chart(&output_path, "Input", input_vec);
        draw_vec_chart(&output_path, "Output", output_vec);
    }

    fn average(frame: &[f32]) -> f32 {
        let num_samples = frame.len() as f32;
        frame.into_iter().map(|f| *f).sum::<f32>() / num_samples
    }

    fn setup_input_processor(settings: AudioProcessorSettings) -> AudioFileProcessor {
        let input_file_path = relative_path!("../../../../input-files/C3-loop.mp3");
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        input.prepare(settings);
        input
    }
}
