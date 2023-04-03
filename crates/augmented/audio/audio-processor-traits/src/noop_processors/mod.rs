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
    fn process(&mut self, _context: &mut AudioContext, _frame: &mut AudioBuffer<Self::SampleType>) {
    }
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

    fn process(&mut self, _context: &mut AudioContext, output: &mut AudioBuffer<SampleType>) {
        for channel in output.channels_mut() {
            for sample in channel {
                *sample = SampleType::zero();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{AudioProcessor, BufferProcessor};

    use super::*;

    #[test]
    fn test_noop_processor() {
        let mut output = BufferProcessor(NoopAudioProcessor::<f32>::default());
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);

        let mut right_channel = [1.0, 1.0, 1.0];
        let mut left_channel = [2.0, 2.0, 2.0];
        let buffer = [right_channel, left_channel];
        let mut buffer = AudioBuffer::from(buffer.iter().cloned());
        output.process(&mut ctx, &mut buffer);
        assert_eq!(
            buffer.channels(),
            &vec![vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0],]
        );
    }

    #[test]
    fn test_silence_processor() {
        let mut output = SilenceAudioProcessor::<f32>::default();
        let mut ctx = AudioContext::default();
        output.prepare(&mut ctx);
        let mut right_channel = [1.0, 1.0, 1.0];
        let mut left_channel = [2.0, 2.0, 2.0];
        let buffer = [right_channel, left_channel];
        let mut buffer = AudioBuffer::from(buffer.iter().cloned());

        output.process(&mut ctx, &mut buffer);
        assert_eq!(
            buffer.channels(),
            &vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0],]
        );
    }
}
