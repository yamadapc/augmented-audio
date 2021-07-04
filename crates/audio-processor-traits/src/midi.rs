pub trait MidiMessageLike {
    fn is_midi(&self) -> bool;
    fn bytes(&self) -> Option<&[u8]>;
}

pub trait MidiEventHandler {
    /// MIDI messages. May contain invalid events (of a different type) which should be skipped.
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]);
}

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
