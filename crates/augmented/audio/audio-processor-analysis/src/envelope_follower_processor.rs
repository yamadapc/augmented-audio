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
//! use audio_processor_analysis::envelope_follower_processor::{EnvelopeFollowerHandleImpl, EnvelopeFollowerProcessorImpl};
//! use audio_processor_traits::{AudioContext, AudioProcessorSettings, simple_processor::MonoAudioProcessor};
//!
//! let mut envelope_follower = EnvelopeFollowerProcessorImpl::default();
//! let handle: Shared<EnvelopeFollowerHandleImpl> = envelope_follower.handle().clone();
//! handle.set_attack(Duration::from_secs_f32(0.4));
//!
//! let mut context = AudioContext::from(AudioProcessorSettings::default());
//! envelope_follower.m_prepare(&mut context);
//! envelope_follower.m_process(&mut context, 0.0);
//! ```

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::atomic_float::{AtomicFloatRepresentable, AtomicValue};
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioContext, AudioProcessorSettings, Float};
use std::time::Duration;

fn calculate_multiplier<F: Float>(sample_rate: F, duration_ms: F) -> F {
    let attack_secs = duration_ms * F::from(0.001).unwrap();
    let attack_samples = sample_rate * attack_secs;
    (F::from(-1.0).unwrap() / attack_samples).exp2()
}

/// Envelope follower processor (f32)
pub type EnvelopeFollowerProcessor = EnvelopeFollowerProcessorImpl<f32>;

/// Envelope follower handle (f32)
pub type EnvelopeFollowerHandle = EnvelopeFollowerHandleImpl<f32>;

/// Handle for [`EnvelopeFollowerProcessorImpl`] use this to interact with the processor parameters from
/// any thread.
pub struct EnvelopeFollowerHandleImpl<ST: AtomicFloatRepresentable> {
    envelope_state: ST::AtomicType,
    attack_multiplier: ST::AtomicType,
    release_multiplier: ST::AtomicType,
    attack_duration_ms: ST::AtomicType,
    release_duration_ms: ST::AtomicType,
    sample_rate: ST::AtomicType,
    _marker: std::marker::PhantomData<ST>,
}

impl<ST: Float + AtomicFloatRepresentable> EnvelopeFollowerHandleImpl<ST> {
    /// Get the current envelope value
    pub fn state(&self) -> ST {
        ST::from(self.envelope_state.get()).unwrap()
    }

    /// Set the attack as a `Duration`
    pub fn set_attack(&self, duration: Duration) {
        let duration_ms = ST::from(duration.as_millis()).unwrap();
        self.attack_duration_ms.set(duration_ms);
        self.attack_multiplier
            .set(calculate_multiplier(self.sample_rate.get(), duration_ms));
    }

    /// Set the release as a `Duration`
    pub fn set_release(&self, duration: Duration) {
        let duration_ms = ST::from(duration.as_millis()).unwrap();
        self.release_duration_ms.set(duration_ms);
        self.release_multiplier
            .set(calculate_multiplier(self.sample_rate.get(), duration_ms));
    }
}

/// An implementation of an envelope follower.
///
/// Implements [`audio_processor_traits::simple_processor::MonoAudioProcessor`]. Can either use it for per-sample
/// processing or wrap this with [`audio_processor_traits::simple_processor::BufferProcessor`].
///
/// # Example
/// ```rust
/// use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessorImpl;
/// use audio_processor_traits::{AudioContext, AudioProcessorSettings, simple_processor::MonoAudioProcessor};
///
/// let mut  envelope_follower = EnvelopeFollowerProcessorImpl::default();
/// let _handle = envelope_follower.handle(); // can send to another thread
///
/// // Envelope follower implements `MonoAudioProcessor
/// let mut context = AudioContext::from(AudioProcessorSettings::default());
/// envelope_follower.m_prepare(&mut context);
/// envelope_follower.m_process(&mut context, 1.0);
/// ```
pub struct EnvelopeFollowerProcessorImpl<ST: AtomicFloatRepresentable> {
    handle: Shared<EnvelopeFollowerHandleImpl<ST>>,
}

impl<ST> Default for EnvelopeFollowerProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + Send + 'static,
    ST::AtomicType: Send + 'static,
{
    fn default() -> Self {
        Self::new(Duration::from_millis(10), Duration::from_millis(10))
    }
}

impl<ST> EnvelopeFollowerProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + Send + 'static,
    ST::AtomicType: Send + 'static,
{
    /// Create a new `EnvelopeFollowerProcessorImpl` with this attack and release times.
    pub fn new(attack_duration: Duration, release_duration: Duration) -> Self {
        let sample_rate = AudioProcessorSettings::default().sample_rate as f64;
        EnvelopeFollowerProcessorImpl {
            handle: make_shared(EnvelopeFollowerHandleImpl {
                envelope_state: ST::AtomicType::from(ST::zero()),
                attack_multiplier: ST::AtomicType::from(
                    ST::from(calculate_multiplier(
                        sample_rate,
                        attack_duration.as_millis() as f64,
                    ))
                    .unwrap(),
                ),
                release_multiplier: ST::AtomicType::from(
                    ST::from(calculate_multiplier(
                        sample_rate,
                        release_duration.as_millis() as f64,
                    ))
                    .unwrap(),
                ),
                attack_duration_ms: (ST::AtomicType::from(
                    ST::from(attack_duration.as_millis() as f64).unwrap(),
                )),
                release_duration_ms: ST::AtomicType::from(
                    ST::from(release_duration.as_millis() as f64).unwrap(),
                ),
                sample_rate: ST::AtomicType::from(ST::from(sample_rate).unwrap()),
                _marker: Default::default(),
            }),
        }
    }

    /// Get a reference to the `basedrop::Shared` state handle of this processor
    pub fn handle(&self) -> &Shared<EnvelopeFollowerHandleImpl<ST>> {
        &self.handle
    }
}

impl<ST: AtomicFloatRepresentable + Copy + Float> MonoAudioProcessor
    for EnvelopeFollowerProcessorImpl<ST>
{
    type SampleType = ST;

    fn m_prepare(&mut self, context: &mut AudioContext) {
        let sample_rate = ST::from(context.settings.sample_rate as f64).unwrap();
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
        let envelope = ST::from(handle.envelope_state.get()).unwrap();
        let attack = ST::from(handle.attack_multiplier.get()).unwrap();
        let release = ST::from(handle.release_multiplier.get()).unwrap();

        let one = ST::from(1.0).unwrap();
        if value > envelope {
            handle
                .envelope_state
                .set((one - attack) * value + attack * envelope);
        } else {
            handle
                .envelope_state
                .set((one - release) * value + release * envelope);
        }

        sample
    }
}

#[cfg(test)]
mod test {
    use audio_processor_file::AudioFileProcessor;
    use audio_processor_testing_helpers::charts::draw_vec_chart;
    use audio_processor_testing_helpers::relative_path;
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
        input.prepare(&mut context);

        let mut envelope_follower = EnvelopeFollowerProcessorImpl::default();
        envelope_follower.m_prepare(&mut context);

        let mut buffer = AudioBuffer::empty();
        buffer.resize(1, settings.block_size());
        let num_chunks = (input.num_samples() / 8) / settings.block_size();

        let mut envelope_readings = vec![];
        for _chunk in 0..num_chunks {
            for sample in buffer.slice_mut() {
                *sample = 0.0;
            }

            input.process(&mut context, &mut buffer);
            for sample_num in 0..buffer.num_samples() {
                let sample = *buffer.get(0, sample_num);
                envelope_follower.m_process(&mut context, sample);
                envelope_readings.push(envelope_follower.handle.envelope_state.get());
            }
        }

        draw_vec_chart(&output_path, "Envelope", envelope_readings);
    }
}
