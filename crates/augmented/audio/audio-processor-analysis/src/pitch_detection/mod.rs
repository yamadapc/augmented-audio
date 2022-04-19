//! This module implements multiple pitch-detection strategies.
//!
//! References:
//!
//! * YIN, a fundamental frequency estimator for speech and music, Alain de Cheveigne and Hideki Kawahara
//! * Probabilistic Modelling of Note Events in the Transcription of Monophonic Melodies, Matti Ryynanen
//! * HMM Decoding: Viterbi Algorithm, Shallow Processing Techniques for NLP Ling570
//! * Parabolic Interpolation, http://fourier.eng.hmc.edu/

use std::cmp::Ordering;

use audio_processor_traits::{AudioProcessorSettings, SimpleAudioProcessor};

use crate::fft_processor::FftProcessor;

/// Given a bin, estimate its frequency
fn estimate_frequency_from_location(sample_rate: f32, location: usize, bin_count: usize) -> f32 {
    let ratio: f32 = location as f32 / bin_count as f32;
    sample_rate * ratio
}

enum EstimationStrategy {
    MaxValue,
}

/// Implements pitch-detection by finding the maximum bin in a FFT buffer.
///
/// This is not accurate and susceptible to jumping around harmonics.
fn max_value_estimate_frequency(
    sample_rate: f32,
    fft_real_buffer: impl ExactSizeIterator<Item = f32>,
) -> Option<f32> {
    let bin_count = fft_real_buffer.len();
    let max_bin = maximum_index(fft_real_buffer)?;

    Some(estimate_frequency_from_location(
        sample_rate,
        max_bin,
        bin_count,
    ))
}

fn maximum_index(iterator: impl ExactSizeIterator<Item = f32>) -> Option<usize> {
    iterator
        .enumerate()
        .max_by(|(_, f1): &(usize, f32), (_, f2): &(usize, f32)| {
            f1.abs().partial_cmp(&f2.abs()).unwrap_or(Ordering::Equal)
        })
        .map(|(i, _f)| i)
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
        // log::info!("{:?}", fft_buffer);
        let fft_real_buffer = fft_buffer.iter().map(|f| f.re).take(fft_buffer.len() / 2);
        let estimate = match self.strategy {
            EstimationStrategy::MaxValue => {
                max_value_estimate_frequency(self.sample_rate, fft_real_buffer)
            }
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
    use audio_processor_testing_helpers::{assert_f_eq, relative_path};

    use crate::pitch_detection::{
        max_value_estimate_frequency, maximum_index, PitchDetectorProcessor,
    };
    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{
        AudioBuffer, AudioProcessorSettings, OwnedAudioBuffer, SimpleAudioProcessor, VecAudioBuffer,
    };

    #[test]
    fn test_max_estimator() {
        wisual_logger::init_from_env();
        let input_file_path = relative_path!("../../../confirmation.mp3");
        // let input_file_path = relative_path!("../../../../input-files/C3-loop.mp3");

        let settings = AudioProcessorSettings::default();
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        input.prepare(settings);

        let mut pitch_detector = PitchDetectorProcessor::default();
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
        draw_vec_chart(&output_path, "MaxValueEstimator", results);
    }

    #[test]
    fn test_maximum_estimate_location() {
        let iterator: Vec<f32> = vec![10.0, 30.0, 20.0, 5.0];
        let freq = max_value_estimate_frequency(1000.0, iterator.iter().cloned()).unwrap();
        // Maximum bin 1, sample rate is 1000Hz, num bins is 4
        // Estimated freq should be 250Hz
        assert_f_eq!(freq, 250.0);
    }

    #[test]
    fn test_maximum_index_when_exists() {
        let iterator: Vec<f32> = vec![10.0, 30.0, 20.0, 5.0];
        let index = maximum_index(iterator.iter().cloned()).unwrap();
        assert_eq!(index, 1);
    }

    #[test]
    fn test_maximum_index_when_does_not_exist() {
        let iterator: Vec<f32> = vec![];
        let index = maximum_index(iterator.iter().cloned());
        assert_eq!(index, None);
    }
}
