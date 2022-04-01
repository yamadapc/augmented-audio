use crate::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};

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

/// Wrapper over `SimpleAudioProcessor` to provide an `AudioProcessor` impl.
#[derive(Clone, Default, Debug)]
pub struct BufferProcessor<Processor>(pub Processor);

/// Process a buffer of samples with a `SimpleAudioProcessor`
#[inline]
pub fn process_buffer<Processor, BufferType>(processor: &mut Processor, data: &mut BufferType)
where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
    BufferType: AudioBuffer<SampleType = Processor::SampleType>,
{
    for frame in data.frames_mut() {
        processor.s_process_frame(frame);
    }
}

impl<Processor> AudioProcessor for BufferProcessor<Processor>
where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
{
    type SampleType = <Processor as SimpleAudioProcessor>::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.0.s_prepare(settings);
    }

    #[inline]
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        process_buffer(&mut self.0, data)
    }
}

impl<Processor> MidiEventHandler for BufferProcessor<Processor>
where
    Processor: MidiEventHandler,
{
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        self.0.process_midi_events(midi_messages)
    }
}
