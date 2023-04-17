// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::sync::{Arc, Mutex};

use basedrop::Handle;

pub use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
use audio_processor_standalone_midi::host::MidiError;
pub use audio_processor_standalone_midi::host::MidiHost;
pub use audio_processor_standalone_midi::host::MidiMessageQueue;
use audio_processor_traits::MidiEventHandler;

use crate::StandaloneProcessor;

use super::MidiContext;

pub type MidiReference = Arc<Mutex<Option<Result<MidiHost, MidiError>>>>;

pub fn initialize_midi_host(
    app: &mut impl StandaloneProcessor,
    handle: Option<&Handle>,
) -> (MidiReference, Option<MidiContext>) {
    log::info!("Initializing MIDI Host in the background");

    let midi_host = app.midi().and(handle).map(MidiHost::default_with_handle);
    let midi_context = midi_host.as_ref().map(|midi_host| {
        let midi_message_queue = midi_host.messages().clone();
        let midi_audio_thread_handler = MidiAudioThreadHandler::default();
        MidiContext {
            midi_audio_thread_handler,
            midi_message_queue,
        }
    });

    let midi_ref = Arc::new(Mutex::new(None));

    if let Some(mut midi_host) = midi_host {
        let midi_ref = midi_ref.clone();
        std::thread::Builder::new()
            .name(String::from("MIDI Host start thread"))
            .spawn(move || match midi_host.start_midi() {
                Ok(_) => {
                    let mut midi_ref = midi_ref.lock().unwrap();
                    *midi_ref = Some(Ok(midi_host));
                    log::info!("MIDI Host ready");
                }
                Err(err) => {
                    log::error!("MIDI Host start error {}", err);
                    let mut midi_ref = midi_ref.lock().unwrap();
                    *midi_ref = Some(Err(err));
                }
            })
            .unwrap();
    }

    (midi_ref, midi_context)
}

pub fn flush_midi_events(
    midi_context: Option<&mut MidiContext>,
    processor: &mut impl StandaloneProcessor,
) {
    if let Some(MidiContext {
        midi_audio_thread_handler,
        midi_message_queue,
    }) = midi_context
    {
        if let Some(midi_handler) = processor.midi() {
            midi_audio_thread_handler.collect_midi_messages(midi_message_queue);
            midi_handler.process_midi_events(midi_audio_thread_handler.buffer());
            midi_audio_thread_handler.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use basedrop::Owned;

    use atomic_queue::Queue;
    use audio_garbage_collector::make_shared;
    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};
    use audio_processor_traits::{MidiMessageLike, NoopAudioProcessor};

    use crate::standalone_processor::StandaloneOptions;

    use super::*;

    struct MockMidiEventHandler {
        messages: Vec<MidiMessageWrapper>,
    }

    impl MockMidiEventHandler {
        fn new() -> Self {
            Self { messages: vec![] }
        }

        fn messages(&self) -> &Vec<MidiMessageWrapper> {
            &self.messages
        }
    }

    impl MidiEventHandler for MockMidiEventHandler {
        fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
            let mut messages: Vec<MidiMessageWrapper> = midi_messages
                .iter()
                .filter_map(|msg| msg.bytes())
                .map(|bytes| MidiMessageWrapper {
                    timestamp: 0,
                    message_data: [bytes[0], bytes[1], bytes[2]],
                })
                .collect();
            self.messages.append(&mut messages);
        }
    }

    struct MockStandaloneProcessor {
        processor: NoopAudioProcessor<f32>,
        midi: MockMidiEventHandler,
        options: StandaloneOptions,
    }

    impl MockStandaloneProcessor {
        fn new() -> Self {
            Self {
                processor: NoopAudioProcessor::new(),
                midi: MockMidiEventHandler::new(),
                options: StandaloneOptions::default(),
            }
        }
    }

    impl StandaloneProcessor for MockStandaloneProcessor {
        type Processor = NoopAudioProcessor<f32>;
        type Midi = MockMidiEventHandler;

        fn processor(&mut self) -> &mut Self::Processor {
            &mut self.processor
        }

        fn midi(&mut self) -> Option<&mut Self::Midi> {
            Some(&mut self.midi)
        }

        fn supports_midi(&mut self) -> bool {
            true
        }

        fn options(&self) -> &StandaloneOptions {
            &self.options
        }
    }

    #[test]
    fn test_flush_midi_events_without_midi_context_does_nothing() {
        let midi_context = None;
        let mut processor = MockStandaloneProcessor::new();
        flush_midi_events(midi_context, &mut processor);
    }

    #[test]
    fn test_flush_midi_calls_into_midi_processing() {
        let midi_audio_thread_handler = MidiAudioThreadHandler::default();
        let midi_message_queue = make_shared(Queue::new(10));
        let mut processor = MockStandaloneProcessor::new();

        midi_message_queue.push(MidiMessageEntry(Owned::new(
            audio_garbage_collector::handle(),
            MidiMessageWrapper {
                timestamp: 0,
                message_data: [0, 10, 20],
            },
        )));

        let mut midi_context = MidiContext {
            midi_message_queue,
            midi_audio_thread_handler,
        };
        flush_midi_events(Some(&mut midi_context), &mut processor);
        assert_eq!(midi_context.midi_message_queue.len(), 0);
        assert_eq!(processor.midi.messages().len(), 1);
        assert_eq!(processor.midi.messages()[0].message_data, [0, 10, 20]);
    }
}
