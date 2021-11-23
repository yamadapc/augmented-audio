use crate::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over.
pub trait SimpleAudioProcessor {
    type SampleType: Copy;

    /// Prepare for playback based on current audio settings
    fn s_prepare(&mut self, _settings: AudioProcessorSettings) {}

    /// Process a single sample. If the input is mult-channel, will run for each channel by default.
    /// If the processor is multi-channel, implement s_process_frame instead.
    ///
    /// `s_process_frame` is what should be called by consumers & its not required to implement a
    /// sound `s_process` method.
    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        sample
    }

    /// Process a multi-channel frame.
    ///
    /// By default calls s_process.
    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        for sample in frame {
            *sample = self.s_process(*sample);
        }
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
            self.s_process_frame(frame);
        }
    }
}
