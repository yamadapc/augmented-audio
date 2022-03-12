use audio_processor_traits::parameters::AudioProcessorHandleRef;
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

    fn options(&self) -> &StandaloneOptions;

    fn handle(&self) -> Option<AudioProcessorHandleRef> {
        None
    }
}

pub struct StandaloneOptions {
    pub accepts_input: bool,
    pub handle: Option<AudioProcessorHandleRef>,
}

impl Default for StandaloneOptions {
    fn default() -> Self {
        StandaloneOptions {
            accepts_input: true,
            handle: None,
        }
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
    options: StandaloneOptions,
}

impl<P> StandaloneAudioOnlyProcessor<P> {
    pub fn new(processor: P, options: StandaloneOptions) -> Self {
        StandaloneAudioOnlyProcessor { processor, options }
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

    fn options(&self) -> &StandaloneOptions {
        &self.options
    }

    fn handle(&self) -> Option<AudioProcessorHandleRef> {
        self.options.handle.clone()
    }
}

/// A standalone processor impl that will process audio and MIDI
pub struct StandaloneProcessorImpl<P> {
    processor: P,
    options: StandaloneOptions,
}

impl<P> StandaloneProcessorImpl<P> {
    pub fn new(processor: P) -> Self {
        StandaloneProcessorImpl {
            processor,
            options: StandaloneOptions::default(),
        }
    }

    pub fn new_with(processor: P, options: StandaloneOptions) -> Self {
        StandaloneProcessorImpl { processor, options }
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

    fn options(&self) -> &StandaloneOptions {
        &self.options
    }

    fn handle(&self) -> Option<AudioProcessorHandleRef> {
        self.options.handle.clone()
    }
}

#[cfg(test)]
mod test {
    use audio_processor_standalone_midi::host::MidiMessageEntry;
    use audio_processor_traits::simple_processor::SimpleAudioProcessor;
    use audio_processor_traits::{BufferProcessor, NoopAudioProcessor};

    use super::*;

    #[test]
    fn test_midi_event_handler() {
        let mut handler = NoMidiEventHandler {};
        let midi_messages: Vec<MidiMessageEntry> = vec![];
        handler.process_midi_events(&midi_messages);
    }

    #[test]
    fn test_create_standalone_audio_processor() {
        let processor = BufferProcessor(NoopAudioProcessor::new());
        let mut standalone_audio_processor =
            StandaloneAudioOnlyProcessor::new(processor, Default::default());
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
            fn process_midi_events<Message: MidiMessageLike>(
                &mut self,
                _midi_messages: &[Message],
            ) {
            }
        }

        let processor = MockProcessor {};
        let mut standalone_audio_processor =
            StandaloneProcessorImpl::new(BufferProcessor(processor));
        assert!(standalone_audio_processor.supports_midi());
        assert!(standalone_audio_processor.midi().is_some());
        let _processor = standalone_audio_processor.processor();
    }
}
