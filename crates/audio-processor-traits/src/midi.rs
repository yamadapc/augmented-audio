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

/// `rust-vst` compatibility for the MidiMessageLike trait
#[cfg(feature = "vst_support")]
mod vst {
    use super::*;

    use ::vst::api::{Event, EventType, MidiEvent};

    impl MidiMessageLike for *mut Event {
        fn is_midi(&self) -> bool {
            unsafe {
                match (**self).event_type {
                    EventType::Midi => true,
                    _ => false,
                }
            }
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

    impl MidiMessageLike for *const Event {
        fn is_midi(&self) -> bool {
            unsafe {
                match (**self).event_type {
                    EventType::Midi => true,
                    _ => false,
                }
            }
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
