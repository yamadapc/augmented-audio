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

//! Implements a compressor [`audio_processor_traits::AudioProcessor`].
//!
//! # Background
//! * [Digital Dynamic Range Compressor Design — A Tutorial and Analysis](https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf)

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor, AudioProcessorSettings};
use augmented_audio_volume::db_to_amplitude;
use handle::CompressorHandle;

type FloatT = augmented_audio_volume::Float;

mod handle {
    #[cfg(not(feature = "f64"))]
    use audio_processor_traits::AtomicF32 as AtomicFloat;

    #[cfg(feature = "f64")]
    use audio_processor_traits::AtomicF64 as AtomicFloat;

    use super::FloatT;

    pub fn calculate_multiplier(sample_rate: FloatT, duration_ms: FloatT) -> FloatT {
        let attack_secs = duration_ms * 0.001;
        let attack_samples = sample_rate * attack_secs;
        FloatT::exp2(-1.0 / attack_samples)
    }

    pub struct CompressorHandle {
        make_up_gain_db: AtomicFloat,
        knee_width_db: AtomicFloat,
        threshold_db: AtomicFloat,
        ratio: AtomicFloat,
        attack_ms: AtomicFloat,
        release_ms: AtomicFloat,
        sample_rate: AtomicFloat,
    }

    impl Default for CompressorHandle {
        fn default() -> Self {
            Self {
                make_up_gain_db: AtomicFloat::new(0.0),
                knee_width_db: AtomicFloat::new(0.1),
                threshold_db: AtomicFloat::new(-10.0),
                ratio: AtomicFloat::new(2.0),
                attack_ms: AtomicFloat::new(3.0),
                release_ms: AtomicFloat::new(10.0),
                sample_rate: AtomicFloat::new(44100.0),
            }
        }
    }

    impl CompressorHandle {
        pub fn attack_mult(&self) -> FloatT {
            calculate_multiplier(self.sample_rate.get(), self.attack_ms.get())
        }

        pub fn release_mult(&self) -> FloatT {
            calculate_multiplier(self.sample_rate.get(), self.release_ms.get())
        }

        pub fn set_attack_ms(&self, value: FloatT) {
            self.attack_ms.set(value);
        }

        pub fn set_make_up_gain(&self, value: FloatT) {
            self.make_up_gain_db.set(value);
        }

        pub fn set_release_ms(&self, value: FloatT) {
            self.release_ms.set(value);
        }

        pub fn set_sample_rate(&self, sample_rate: FloatT) {
            self.sample_rate.set(sample_rate);
        }

        pub fn set_threshold(&self, threshold: FloatT) {
            self.threshold_db.set(threshold)
        }

        pub fn set_knee_width(&self, width: FloatT) {
            self.knee_width_db.set(width)
        }

        pub fn set_ratio(&self, ratio: FloatT) {
            self.ratio.set(ratio)
        }

        pub fn ratio(&self) -> FloatT {
            self.ratio.get()
        }

        pub fn make_up_gain(&self) -> FloatT {
            self.make_up_gain_db.get()
        }

        pub fn threshold(&self) -> FloatT {
            self.threshold_db.get()
        }

        pub fn knee_width(&self) -> FloatT {
            self.knee_width_db.get()
        }
    }
}

pub struct CompressorProcessor {
    peak_detector_state: PeakDetector,
    handle: Shared<CompressorHandle>,
}

impl Default for CompressorProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressorProcessor {
    pub fn new() -> Self {
        Self {
            peak_detector_state: PeakDetector::default(),
            handle: make_shared(CompressorHandle::default()),
        }
    }

    pub fn handle(&self) -> &Shared<CompressorHandle> {
        &self.handle
    }
}

impl AudioProcessor for CompressorProcessor {
    type SampleType = FloatT;

    fn prepare(&mut self, _context: &mut AudioContext, settings: AudioProcessorSettings) {
        self.handle.set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _context: &mut AudioContext,
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
    fn apply_gain(&mut self, frame: &mut [FloatT]) {
        let gain = self.compute_gain();
        for sample in frame {
            *sample *= gain;
        }
    }

    fn compute_gain(&self) -> FloatT {
        let level = self.peak_detector_state.value;
        let ratio = self.handle.ratio();
        let make_up_gain = db_to_amplitude(self.handle.make_up_gain(), 1.0);
        let threshold = db_to_amplitude(self.handle.threshold(), 1.0);
        let width = db_to_amplitude(self.handle.knee_width(), 1.0);

        let delta = level - threshold;
        let output = if (2.0 * delta) < -width {
            1.0
        } else if (2.0 * delta.abs()) <= width {
            1.0 + (1.0 / ratio - 1.0) * (delta + width / 2.0).powf(2.0) / 2.0 * width
        } else {
            1.0 + delta * (1.0 / ratio - 1.0)
        };

        make_up_gain + output
    }
}

struct PeakDetector {
    value: FloatT,
}

impl Default for PeakDetector {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl PeakDetector {
    fn accept_frame(&mut self, attack_mult: FloatT, release_mult: FloatT, frame: &[FloatT]) {
        let frame_len = frame.len() as FloatT;
        let new: FloatT = frame.iter().map(|f| FloatT::abs(*f)).sum::<FloatT>() / frame_len;
        let curr_slope = if self.value > new {
            release_mult
        } else {
            attack_mult
        };
        self.value = (self.value * curr_slope) + ((1.0 - curr_slope) * new);
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::charts::{
        draw_multi_vec_charts, draw_vec_chart, BLUE, RED,
    };
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{
        audio_buffer, InterleavedAudioBuffer, OwnedAudioBuffer, VecAudioBuffer,
    };
    use augmented_audio_volume::amplitude_to_db;

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
    fn test_knee_widths() {
        let amp = db_to_amplitude(0.1, 1.0);
        assert!(amp > 0.0);
        assert!(amp < 2.0);
    }

    #[test]
    fn test_peak_detector_output() {
        let output_path = relative_path!("src/peak-detector");
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::default();
        let mut input = setup_input_processor(settings);
        let mut processor = PeakDetector::default();
        let attack_multi = handle::calculate_multiplier(settings.sample_rate(), 1.0);
        let release_mult = handle::calculate_multiplier(settings.sample_rate(), 5.0);

        let mut input_vec = vec![];
        let mut output_vec = vec![];
        {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(2, settings.block_size(), 0.0);
            let num_chunks = (input.num_samples() / 8) / settings.block_size();
            for _chunk in 0..num_chunks {
                audio_buffer::clear(&mut buffer);
                input.process(&mut context, &mut buffer);
                for frame in buffer.frames() {
                    input_vec.push(average(frame));
                    processor.accept_frame(attack_multi, release_mult, frame);
                    output_vec.push(processor.value * 2.0);
                }
            }
        }

        draw_multi_vec_charts(
            &output_path,
            "Peak Detector",
            vec![(RED, input_vec), (BLUE, output_vec)],
        );
    }

    #[test]
    fn test_compress_synth_loop() {
        let output_path = relative_path!("src/compressor");
        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        let mut input = setup_input_processor(settings);
        let mut processor = CompressorProcessor::new();
        processor.prepare(&mut context, settings);
        processor.handle.set_ratio(30.0);
        processor.handle.set_threshold(-10.0);
        processor.handle.set_attack_ms(1.0);
        processor.handle.set_release_ms(5.0);
        processor.handle.set_knee_width(-1.0);
        processor
            .handle
            .set_make_up_gain(amplitude_to_db(0.25, 1.0));

        let mut input_vec = vec![];
        let mut output_vec = vec![];
        let mut gain_vec = vec![];

        {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, settings.block_size(), 0.0);
            let num_chunks = (input.num_samples() / 8) / settings.block_size();
            for _chunk in 0..num_chunks {
                audio_buffer::clear(&mut buffer);
                input.process(&mut context, &mut buffer);
                for frame in buffer.frames() {
                    input_vec.push(average(frame))
                }

                for sample in buffer.slice_mut() {
                    let mut buf = [*sample];
                    let mut one_sample = InterleavedAudioBuffer::new(1, &mut buf);
                    processor.process(&mut context, &mut one_sample);
                    *sample = buf[0];
                    output_vec.push(*sample);
                    gain_vec.push(processor.compute_gain());
                }
            }
        }

        draw_vec_chart(&output_path, "Input", input_vec);
        draw_vec_chart(&output_path, "Gain", gain_vec);
        draw_vec_chart(&output_path, "Output", output_vec);
    }

    fn average(frame: &[FloatT]) -> FloatT {
        let num_samples = frame.len() as FloatT;
        frame.iter().copied().sum::<FloatT>() / num_samples
    }

    fn setup_input_processor(settings: AudioProcessorSettings) -> AudioFileProcessor {
        let input_file_path = relative_path!("../../../../input-files/C3-loop.mp3");
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        let mut context = AudioContext::from(settings);
        input.prepare(&mut context, settings);
        input
    }
}
