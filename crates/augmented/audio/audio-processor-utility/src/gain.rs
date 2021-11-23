use std::ops::Mul;

use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::Float;

/// An `AudioProcessor` which applies gain to an input signal
pub struct GainProcessor<SampleType> {
    gain: SampleType,
}

impl<SampleType: Float> Default for GainProcessor<SampleType> {
    /// Construct a `GainProcessor` with 1.0 gain
    fn default() -> Self {
        Self {
            gain: SampleType::from(1.0).unwrap(),
        }
    }
}

impl<SampleType: Float> GainProcessor<SampleType> {
    /// Construct a `GainProcessor` with gain
    pub fn new(gain: SampleType) -> Self {
        Self { gain }
    }

    /// Change the gain
    pub fn set_gain(&mut self, gain: SampleType) {
        self.gain = gain;
    }

    /// Get the gain
    pub fn gain(&self) -> SampleType {
        self.gain
    }
}

impl<SampleType> SimpleAudioProcessor for GainProcessor<SampleType>
where
    SampleType: Float + Send + Sync + Mul<Output = SampleType>,
{
    type SampleType = SampleType;

    fn s_process(&mut self, sample: SampleType) -> SampleType {
        self.gain * sample
    }
}

#[cfg(test)]
mod test {
    use audio_processor_traits::AudioProcessor;
    use audio_processor_traits::InterleavedAudioBuffer;

    use super::*;

    #[test]
    fn test_gain_does_its_thing() {
        let mut gain = GainProcessor::new(0.8);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(1, &mut samples);

        gain.process(&mut input);

        for sample in samples {
            assert_eq!(sample, 0.8);
        }
    }

    #[test]
    fn test_gain_can_be_changed() {
        let mut gain = GainProcessor::default();
        gain.set_gain(0.8);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(1, &mut samples);

        gain.process(&mut input);

        for sample in samples {
            assert_eq!(sample, 0.8);
        }
    }
}
