/// Represents an "Event" type for audio processors. Due to how events are forwarded to processors,
/// the list of events received might contain non-MIDI events.
pub trait MidiMessageLike {
    fn is_midi(&self) -> bool;
    fn bytes(&self) -> Option<&[u8]>;
}

/// A MIDI event processor
pub trait MidiEventHandler {
    /// MIDI messages. May contain invalid events (of a different type) which should be skipped.
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]);
}

pub struct NoopMidiEventHandler {}

impl Default for NoopMidiEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopMidiEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl MidiEventHandler for NoopMidiEventHandler {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

/// `rust-vst` compatibility for the MidiMessageLike trait
#[cfg(feature = "vst_support")]
pub mod vst {
    use ::vst::api::{Event, EventType, MidiEvent};

    use super::*;

    /// Cast the VST `Events` struct onto a `MidiMessageLike` slice you can pass into processors
    pub fn midi_slice_from_events(events: &::vst::api::Events) -> &[*mut Event] {
        unsafe {
            std::slice::from_raw_parts(
                &events.events[0] as *const *mut _,
                events.num_events as usize,
            )
        }
    }

    impl MidiMessageLike for *mut Event {
        fn is_midi(&self) -> bool {
            unsafe { matches!((**self).event_type, EventType::Midi) }
        }

        fn bytes(&self) -> Option<&[u8]> {
            unsafe {
                if matches!((**self).event_type, EventType::Midi) {
                    let midi_event = *self as *const MidiEvent;
                    Some(&(*midi_event).midi_data)
                } else {
                    None
                }
            }
        }
    }

    impl MidiMessageLike for *const Event {
        fn is_midi(&self) -> bool {
            unsafe { matches!((**self).event_type, EventType::Midi) }
        }

        fn bytes(&self) -> Option<&[u8]> {
            unsafe {
                match (**self).event_type {
                    EventType::Midi => {
                        let midi_event = *self as *const MidiEvent;
                        Some(&(*midi_event).midi_data)
                    }
                    _ => None,
                }
            }
        }
    }
}
