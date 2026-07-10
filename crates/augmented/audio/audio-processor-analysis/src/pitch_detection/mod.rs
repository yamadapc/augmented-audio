//! This module implements multiple pitch-detection strategies.
//!
//! References:
//!
//! * YIN, a fundamental frequency estimator for speech and music, Alain de Cheveigne and Hideki Kawahara
//! * Probabilistic Modelling of Note Events in the Transcription of Monophonic Melodies, Matti Ryynanen
//! * HMM Decoding: Viterbi Algorithm, Shallow Processing Techniques for NLP Ling570
//! * Parabolic Interpolation, http://fourier.eng.hmc.edu/

use audio_processor_traits::{AudioProcessorSettings, SimpleAudioProcessor};

use crate::fft_processor::FftProcessor;

mod max_value_estimator;
mod quadratic_estimator;
mod utils;
mod yin_estimator;

enum EstimationStrategy {
    MaxValue,
    Quadratic,
    YIN,
}

/// Real-time pitch-detection processor. Pitch will be detected with latency from the FFT size.
///
/// Multiple pitch-detection strategies are implemented defaults to the best strategy.
pub struct PitchDetectorProcessor {
    fft: FftProcessor,
    strategy: EstimationStrategy,
    sample_rate: f32,
    estimate: f32,
}

impl From<EstimationStrategy> for PitchDetectorProcessor {
    fn from(strategy: EstimationStrategy) -> Self {
        let mut processor = Self::default();
        processor.strategy = strategy;
        processor
    }
}

impl Default for PitchDetectorProcessor {
    fn default() -> Self {
        PitchDetectorProcessor {
            fft: FftProcessor::default(),
            strategy: EstimationStrategy::MaxValue,
            sample_rate: 0.0,
            estimate: 0.0,
        }
    }
}

impl PitchDetectorProcessor {
    fn on_fft(&mut self) {
        let fft_buffer = self.fft.buffer();
        let fft_real_buffer = fft_buffer.iter().map(|f| f.re).take(fft_buffer.len() / 2);
        let estimate = match self.strategy {
            EstimationStrategy::MaxValue => {
                max_value_estimator::estimate_frequency(self.sample_rate, fft_real_buffer)
            }
            EstimationStrategy::Quadratic => {
                quadratic_estimator::estimate_frequency(self.sample_rate, &fft_buffer)
            }
            EstimationStrategy::YIN => todo!(),
        };
        if let Some(estimate) = estimate {
            self.estimate = estimate;
        }
    }
}

impl SimpleAudioProcessor for PitchDetectorProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        self.fft.s_prepare(settings);
        self.sample_rate = settings.sample_rate();
    }

    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        self.fft.s_process(sample);
        if self.fft.has_changed() {
            self.on_fft();
        }
        sample
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::charts::draw_vec_chart;
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{
        AudioBuffer, AudioProcessorSettings, OwnedAudioBuffer, SimpleAudioProcessor, VecAudioBuffer,
    };

    use crate::pitch_detection::{EstimationStrategy, PitchDetectorProcessor};

    #[test]
    fn test_max_estimator() {
        let strategy = EstimationStrategy::MaxValue;
        let strategy_name = "MaxValueEstimator";
        draw_estimation_strategy(strategy, strategy_name);
    }

    #[test]
    fn test_quadratic_estimator() {
        let strategy = EstimationStrategy::Quadratic;
        let strategy_name = "QuadraticEstimator";
        draw_estimation_strategy(strategy, strategy_name);
    }

    fn draw_estimation_strategy(strategy: EstimationStrategy, strategy_name: &str) {
        wisual_logger::init_from_env();
        let input_file_path = relative_path!("../../../../input-files/C3-loop.mp3");

        let settings = AudioProcessorSettings::default();
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        input.prepare(settings);

        let mut pitch_detector = PitchDetectorProcessor::from(strategy);
        pitch_detector.s_prepare(settings);

        let mut results = vec![];
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(1, settings.block_size(), 0.0);
        let num_chunks = (input.num_samples() / 8) / settings.block_size();
        // results.push(0.0);
        for _chunk in 0..num_chunks {
            for sample in buffer.slice_mut() {
                *sample = 0.0;
            }

            input.process(&mut buffer);
            for frame in buffer.frames_mut() {
                let sample = frame[0];
                pitch_detector.s_process(sample);
                results.push(pitch_detector.estimate);
            }
        }
        // results.push(0.0);

        let results: Vec<f32> = results.iter().skip(10000).cloned().collect();
        let output_path = relative_path!("src/pitch_detection/mod.rs");
        draw_vec_chart(&output_path, strategy_name, results);
    }
}
