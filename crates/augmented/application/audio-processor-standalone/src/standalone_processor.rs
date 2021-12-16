use audio_processor_traits::{AudioProcessor, MidiEventHandler, MidiMessageLike};

/// Abstract standalone processor with runtime optional MIDI handling.
pub trait StandaloneProcessor: Send + 'static {
    type Processor: AudioProcessor<SampleType = f32>;
    type Midi: MidiEventHandler;

    fn processor(&mut self) -> &mut Self::Processor;

    fn midi(&mut self) -> Option<&mut Self::Midi> {
        None
    }

    fn supports_midi(&mut self) -> bool {
        self.midi().is_some()
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

#[cfg(test)]
mod test {
    use super::*;

    use audio_processor_standalone_midi::host::MidiMessageEntry;
    use audio_processor_traits::simple_processor::SimpleAudioProcessor;
    use audio_processor_traits::NoopAudioProcessor;

    #[test]
    fn test_midi_event_handler() {
        let mut handler = NoMidiEventHandler {};
        let midi_messages: Vec<MidiMessageEntry> = vec![];
        handler.process_midi_events(&midi_messages);
    }

    #[test]
    fn test_create_standalone_audio_processor() {
        let processor = NoopAudioProcessor::new();
        let mut standalone_audio_processor = StandaloneAudioOnlyProcessor::new(processor);
        assert!(!standalone_audio_processor.supports_midi());
        assert!(standalone_audio_processor.midi().is_none());
        let _processor = standalone_audio_processor.processor();
    }

    #[test]
    fn test_create_standalone_audio_midi_processor() {
        struct MockProcessor {}
        impl SimpleAudioProcessor for MockProcessor {
            type SampleType = f32;
        }
        impl MidiEventHandler for MockProcessor {
            fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
            }
        }

        let processor = MockProcessor {};
        let mut standalone_audio_processor = StandaloneProcessorImpl::new(processor);
        assert!(standalone_audio_processor.supports_midi());
        assert!(standalone_audio_processor.midi().is_some());
        let _processor = standalone_audio_processor.processor();
    }
}
