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

//! Envelope follower implementation
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/audio-envelope.png)
//!
//! ## Usage
//! ```
//! use std::time::Duration;
//! use audio_garbage_collector::Shared;
//! use audio_processor_analysis::envelope_follower_processor::{EnvelopeFollowerHandle, EnvelopeFollowerProcessor};
//! use audio_processor_traits::{AudioProcessorSettings, SimpleAudioProcessor};
//!
//! let mut envelope_follower = EnvelopeFollowerProcessor::default();
//! let handle: Shared<EnvelopeFollowerHandle> = envelope_follower.handle().clone();
//! handle.set_attack(Duration::from_secs_f32(0.4));
//!
//! envelope_follower.s_prepare(AudioProcessorSettings::default());
//! envelope_follower.s_process(0.0);
//! ```

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AtomicF32, AudioContext, AudioProcessorSettings};
use std::time::Duration;

fn calculate_multiplier(sample_rate: f32, duration_ms: f32) -> f32 {
    let attack_secs = duration_ms * 0.001;
    let attack_samples = sample_rate * attack_secs;
    (-1.0 / attack_samples).exp2()
}

/// Handle for [`EnvelopeFollowerProcessor`] use this to interact with the processor parameters from
/// any thread.
pub struct EnvelopeFollowerHandle {
    envelope_state: AtomicF32,
    attack_multiplier: AtomicF32,
    release_multiplier: AtomicF32,
    attack_duration_ms: AtomicF32,
    release_duration_ms: AtomicF32,
    sample_rate: AtomicF32,
}

impl EnvelopeFollowerHandle {
    /// Get the current envelope value
    pub fn state(&self) -> f32 {
        self.envelope_state.get()
    }

    /// Set the attack as a `Duration`
    pub fn set_attack(&self, duration: Duration) {
        let duration_ms = duration.as_millis() as f32;
        self.attack_duration_ms.set(duration_ms);
        self.attack_multiplier
            .set(calculate_multiplier(self.sample_rate.get(), duration_ms));
    }

    /// Set the release as a `Duration`
    pub fn set_release(&self, duration: Duration) {
        let duration_ms = duration.as_millis() as f32;
        self.release_duration_ms.set(duration_ms);
        self.release_multiplier
            .set(calculate_multiplier(self.sample_rate.get(), duration_ms));
    }
}

/// An implementation of an envelope follower.
///
/// Implements [`audio_processor_traits::SimpleAudioProcessor`]. Can either use it for per-sample
/// processing or wrap this with [`audio_processor_traits::simple_processor::BufferProcessor`].
///
/// # Example
/// ```rust
/// use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessor;
/// use audio_processor_traits::SimpleAudioProcessor;
///
/// let mut  envelope_follower = EnvelopeFollowerProcessor::default();
/// let _handle = envelope_follower.handle(); // can send to another thread
///
/// // Envelope follower implements `SimpleAudioProcessor`
/// envelope_follower.s_prepare(Default::default());
/// envelope_follower.s_process(1.0);
/// ```
pub struct EnvelopeFollowerProcessor {
    handle: Shared<EnvelopeFollowerHandle>,
}

impl Default for EnvelopeFollowerProcessor {
    fn default() -> Self {
        Self::new(Duration::from_millis(10), Duration::from_millis(10))
    }
}

impl EnvelopeFollowerProcessor {
    /// Create a new `EnvelopeFollowerProcessor` with this attack and release times.
    pub fn new(attack_duration: Duration, release_duration: Duration) -> Self {
        let sample_rate = AudioProcessorSettings::default().sample_rate;
        EnvelopeFollowerProcessor {
            handle: make_shared(EnvelopeFollowerHandle {
                envelope_state: 0.0.into(),
                attack_multiplier: calculate_multiplier(
                    sample_rate,
                    attack_duration.as_millis() as f32,
                )
                .into(),
                release_multiplier: calculate_multiplier(
                    sample_rate,
                    release_duration.as_millis() as f32,
                )
                .into(),
                attack_duration_ms: (attack_duration.as_millis() as f32).into(),
                release_duration_ms: (release_duration.as_millis() as f32).into(),
                sample_rate: sample_rate.into(),
            }),
        }
    }

    /// Get a reference to the `basedrop::Shared` state handle of this processor
    pub fn handle(&self) -> &Shared<EnvelopeFollowerHandle> {
        &self.handle
    }
}

impl MonoAudioProcessor for EnvelopeFollowerProcessor {
    type SampleType = f32;

    fn m_prepare(&mut self, _context: &mut AudioContext, settings: AudioProcessorSettings) {
        let sample_rate = settings.sample_rate;
        self.handle.sample_rate.set(sample_rate);
        self.handle.attack_multiplier.set(calculate_multiplier(
            sample_rate,
            self.handle.attack_duration_ms.get(),
        ));
        self.handle.release_multiplier.set(calculate_multiplier(
            sample_rate,
            self.handle.release_duration_ms.get(),
        ));
    }

    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        sample: Self::SampleType,
    ) -> Self::SampleType {
        let value = sample.abs();

        let handle = &self.handle;
        let envelope = handle.envelope_state.get();
        let attack = handle.attack_multiplier.get();
        let release = handle.release_multiplier.get();

        if value > envelope {
            handle
                .envelope_state
                .set((1.0 - attack) * value + attack * envelope);
        } else {
            handle
                .envelope_state
                .set((1.0 - release) * value + release * envelope);
        }

        sample
    }
}

#[cfg(test)]
mod test {
    use audio_processor_file::AudioFileProcessor;
    use audio_processor_testing_helpers::charts::draw_vec_chart;
    use audio_processor_testing_helpers::relative_path;
    use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
    use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

    use super::*;

    #[test]
    fn test_draw_envelope() {
        let output_path = relative_path!("src/envelope_follower_processor");
        let input_file_path = relative_path!("../../../../input-files/C3-loop.mp3");

        let settings = AudioProcessorSettings::default();
        let mut context = AudioContext::from(settings);
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        input.prepare(&mut context, settings);

        let mut envelope_follower = EnvelopeFollowerProcessor::default();
        envelope_follower.m_prepare(&mut context, settings);

        let mut buffer = VecAudioBuffer::new();
        buffer.resize(1, settings.block_size(), 0.0);
        let num_chunks = (input.num_samples() / 8) / settings.block_size();

        let mut envelope_readings = vec![];
        for _chunk in 0..num_chunks {
            for sample in buffer.slice_mut() {
                *sample = 0.0;
            }

            input.process(&mut context, &mut buffer);
            for frame in buffer.frames_mut() {
                let sample = frame[0];
                envelope_follower.m_process(&mut context, sample);
                envelope_readings.push(envelope_follower.handle.envelope_state.get());
            }
        }

        draw_vec_chart(&output_path, "Envelope", envelope_readings);
    }
}
