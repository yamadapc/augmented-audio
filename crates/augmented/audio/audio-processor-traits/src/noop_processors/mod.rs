use std::marker::PhantomData;

use num::Float;

use crate::{AudioBuffer, AudioContext, AudioProcessor, SimpleAudioProcessor};

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

impl<SampleType: Send + Copy> SimpleAudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;
    fn s_process_frame(&mut self, _context: &mut AudioContext, _frame: &mut [SampleType]) {}
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

impl<SampleType: Float + Send> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _context: &mut AudioContext,
        output: &mut BufferType,
    ) {
        for sample in output.slice_mut() {
            *sample = SampleType::zero();
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
        let settings = ctx.settings().clone();
        output.prepare(&mut ctx, settings);

        let mut output_buffer = VecAudioBuffer::new_with(
            vec![
                1.0, 2.0, // 1
                1.0, 2.0, // 2
                1.0, 2.0, // 3
            ],
            2,
            3,
        );
        output.process(&mut ctx, &mut output_buffer);
        assert_eq!(
            output_buffer.slice(),
            [
                1.0, 2.0, // 1
                1.0, 2.0, // 2
                1.0, 2.0, // 3
            ]
        );
    }

    #[test]
    fn test_silence_processor() {
        let mut output = SilenceAudioProcessor::<f32>::default();
        let mut ctx = AudioContext::default();
        let settings = ctx.settings().clone();
        output.prepare(&mut ctx, settings);
        let mut output_buffer = VecAudioBuffer::new_with(
            vec![
                1.0, 2.0, // 1
                1.0, 2.0, // 2
                1.0, 2.0, // 3
            ],
            2,
            3,
        );
        output.process(&mut ctx, &mut output_buffer);
        assert_eq!(
            output_buffer.slice(),
            [
                0.0, 0.0, // 1
                0.0, 0.0, // 2
                0.0, 0.0, // 3
            ]
        );
    }
}
