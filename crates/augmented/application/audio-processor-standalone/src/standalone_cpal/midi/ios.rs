use basedrop::Handle;

use crate::standalone_processor::StandaloneProcessor;

use super::MidiContext;

pub type MidiReference = ();

pub fn initialize_midi_host(
    _app: &mut impl StandaloneProcessor,
    _handle: Option<&Handle>,
) -> (MidiReference, Option<MidiContext>) {
    ((), None)
}

pub fn flush_midi_events(
    _midi_context: Option<&mut MidiContext>,
    _processor: &impl StandaloneProcessor,
) {
}
