use std::marker::PhantomData;

use num::Float;

use crate::{AudioBuffer, AudioContext, AudioProcessor};

/// An audio-processor which doesn't do any work.
pub struct NoopAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> Default for NoopAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType> NoopAudioProcessor<SampleType> {
    pub fn new() -> Self {
        NoopAudioProcessor(PhantomData::default())
    }
}

impl<SampleType: Send + Copy> AudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;
    fn process(&mut self, _context: &mut AudioContext, _frame: &mut [&mut [SampleType]]) {}
}

/// An audio-processor which mutes all channels.
pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> SilenceAudioProcessor<SampleType> {
    pub fn new() -> Self {
        SilenceAudioProcessor(PhantomData)
    }
}

impl<SampleType> Default for SilenceAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType: Float + Send + Sized> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process(&mut self, _context: &mut AudioContext, output: &mut [&mut [Self::SampleType]]) {
        for channel in output {
            for sample in &mut **channel {
                *sample = SampleType::zero();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{AudioProcessor, BufferProcessor, VecAudioBuffer};

    use super::*;

    #[test]
    fn test_noop_processor() {
        let mut output = BufferProcessor(NoopAudioProcessor::<f32>::default());
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);

        let mut right_channel = [1.0, 1.0, 1.0];
        let mut left_channel = [2.0, 2.0, 2.0];
        let mut output_buffer = [right_channel.as_mut(), left_channel.as_mut()];
        output.process(&mut ctx, output_buffer.as_mut());
        assert_eq!(output_buffer, [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0],]);
    }

    #[test]
    fn test_silence_processor() {
        let mut output = SilenceAudioProcessor::<f32>::default();
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);
        let mut right_channel = [1.0, 1.0, 1.0];
        let mut left_channel = [2.0, 2.0, 2.0];
        let mut output_buffer = [right_channel.as_mut(), left_channel.as_mut()];

        output.process(&mut ctx, output_buffer.as_mut());
        assert_eq!(output_buffer, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0],]);
    }
}
