use itertools::Itertools;
use std::sync::atomic::AtomicU8;

use audio_processor_traits::MidiMessageLike;
use augmented_atomics::AtomicOption;
use augmented_midi::{parse_midi_event, MIDIMessage, ParserState};

#[derive(PartialEq, Eq, Debug)]
pub struct MidiStoreValue {
    pub channel: u8,
    pub controller_number: u8,
    pub value: u8,
}

pub struct MidiStoreHandle {
    cc_store: Vec<Vec<AtomicOption<AtomicU8>>>,
}

impl Default for MidiStoreHandle {
    fn default() -> Self {
        let cc_store = [[0u16; 256]; 256]
            .iter()
            .map(|_| {
                [0u16; 256]
                    .iter()
                    .map(|_| AtomicOption::empty())
                    .collect_vec()
            })
            .collect_vec();
        Self { cc_store }
    }
}

impl MidiStoreHandle {
    pub fn values(&self) -> impl Iterator<Item = MidiStoreValue> + '_ {
        self.cc_store
            .iter()
            .enumerate()
            .flat_map(|(channel, channel_values)| {
                channel_values
                    .iter()
                    .enumerate()
                    .filter_map(|(controller_number, value)| {
                        value.inner().map(|v| (controller_number as u8, v))
                    })
                    .map(move |(controller_number, value)| MidiStoreValue {
                        channel: channel as u8,
                        controller_number,
                        value,
                    })
            })
    }

    pub fn process_midi_events<Message: MidiMessageLike>(&self, midi_messages: &[Message]) {
        for message in midi_messages {
            self.process_event(message);
        }
    }

    fn process_event<Message: MidiMessageLike>(&self, midi_message: &Message) {
        let event = midi_message
            .bytes()
            .map(|bytes| parse_midi_event::<&[u8]>(bytes, &mut ParserState::default()).ok())
            .flatten()
            .map(|(_, event)| event);

        if let Some(event) = event {
            if let MIDIMessage::ControlChange {
                channel,
                controller_number,
                value,
            } = event
            {
                self.cc_store[channel as usize][controller_number as usize].set(Some(value));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;
    use basedrop::Owned;
    use itertools::Itertools;

    use audio_garbage_collector::handle;
    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};

    use super::*;

    #[test]
    fn test_create_store() {
        let _store = MidiStoreHandle::default();
    }

    #[test]
    fn test_process_event() {
        let store = MidiStoreHandle::default();
        let message = MidiMessageEntry(Owned::new(
            handle(),
            MidiMessageWrapper {
                message_data: [0b1011_0000, 55, 12],
                timestamp: 0,
            },
        ));

        assert_no_alloc(|| {
            store.process_event(&message);
        });

        let values = store.values().collect_vec();
        assert_eq!(values.len(), 1);
        assert_eq!(
            values[0],
            MidiStoreValue {
                channel: 0,
                controller_number: 55,
                value: 12
            }
        );
    }
}
