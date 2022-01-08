use basedrop::Handle;

pub use audio_processor_standalone_midi::{
    audio_thread::MidiAudioThreadHandler,
    host::{MidiHost, MidiMessageQueue},
};
use audio_processor_traits::MidiEventHandler;

use crate::StandaloneProcessor;

use super::MidiContext;

pub fn initialize_midi_host(
    app: &mut impl StandaloneProcessor,
    handle: Option<&Handle>,
) -> (Option<MidiHost>, Option<MidiContext>) {
    let midi_host = app.midi().and(handle).map(|handle| {
        // MIDI set-up
        let mut midi_host = MidiHost::default_with_handle(handle);
        midi_host.start_midi().expect("Failed to start MIDI host");
        midi_host
    });
    let midi_context = midi_host.as_ref().map(|midi_host| {
        let midi_message_queue = midi_host.messages().clone();
        let midi_audio_thread_handler = MidiAudioThreadHandler::default();
        MidiContext {
            midi_audio_thread_handler,
            midi_message_queue,
        }
    });
    (midi_host, midi_context)
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
