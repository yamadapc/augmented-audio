use crate::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over.
pub trait SimpleAudioProcessor {
    type SampleType;

    /// Prepare for playback based on current audio settings
    fn s_prepare(&mut self, _settings: AudioProcessorSettings) {}

    /// Process a single sample. If the input is mult-channel, will run for each channel by default.
    ///
    /// If the processor is stereo, implement s_process_channel instead.
    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType;

    /// Process a single sample given its channel
    ///
    /// By default calls s_process.
    fn s_process_channel(&mut self, _channel: usize, sample: Self::SampleType) -> Self::SampleType {
        self.s_process(sample)
    }
}

impl<Processor> AudioProcessor for Processor
where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
{
    type SampleType = <Processor as SimpleAudioProcessor>::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.s_prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            for (channel, sample) in frame.iter_mut().enumerate() {
                *sample = self.s_process_channel(channel, *sample);
            }
        }
    }
}
