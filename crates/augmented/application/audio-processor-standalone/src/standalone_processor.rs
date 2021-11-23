use audio_processor_traits::{AudioProcessor, MidiEventHandler, MidiMessageLike};

/// Abstract standalone processor with runtime optional MIDI handling.
pub trait StandaloneProcessor: Send + 'static {
    type Processor: AudioProcessor<SampleType = f32>;
    type Midi: MidiEventHandler;

    fn processor(&mut self) -> &mut Self::Processor;
    fn midi(&mut self) -> Option<&mut Self::Midi> {
        None
    }
}

/// Noop MIDI event handler.
pub struct NoMidiEventHandler {}
impl MidiEventHandler for NoMidiEventHandler {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

/// A standalone processor impl that will only process audio
pub struct StandaloneAudioOnlyProcessor<P> {
    processor: P,
}

impl<P> StandaloneAudioOnlyProcessor<P> {
    pub fn new(processor: P) -> Self {
        StandaloneAudioOnlyProcessor { processor }
    }
}

impl<P> StandaloneProcessor for StandaloneAudioOnlyProcessor<P>
where
    P: AudioProcessor<SampleType = f32> + Send + 'static,
{
    type Processor = P;
    type Midi = NoMidiEventHandler;

    fn processor(&mut self) -> &mut Self::Processor {
        &mut self.processor
    }

    fn midi(&mut self) -> Option<&mut Self::Midi> {
        None
    }
}

/// A standalone processor impl that will process audio and MIDI
pub struct StandaloneProcessorImpl<P> {
    processor: P,
}

impl<P> StandaloneProcessorImpl<P> {
    pub fn new(processor: P) -> Self {
        StandaloneProcessorImpl { processor }
    }
}

impl<P> StandaloneProcessor for StandaloneProcessorImpl<P>
where
    P: AudioProcessor<SampleType = f32> + Send + 'static,
    P: MidiEventHandler,
{
    type Processor = P;
    type Midi = P;

    fn processor(&mut self) -> &mut Self::Processor {
        &mut self.processor
    }

    fn midi(&mut self) -> Option<&mut Self::Midi> {
        Some(&mut self.processor)
    }
}
