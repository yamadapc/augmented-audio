use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use basedrop::Shared;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::Duration;

use audio_processor_traits::MidiMessageLike;
use augmented_atomics::AtomicOption;
use augmented_midi::{parse_midi_event, MIDIMessage, ParserState};

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MidiStoreValue {
    pub channel: u8,
    pub controller_number: u8,
    pub value: u8,
}

pub struct MidiStoreHandle {
    cc_store: Vec<Vec<AtomicOption<AtomicU8>>>,
    events: Shared<Queue<MidiStoreValue>>,
}

impl Default for MidiStoreHandle {
    fn default() -> Self {
        Self::new(make_shared(Queue::new(100)))
    }
}

impl MidiStoreHandle {
    pub fn new(events: Shared<Queue<MidiStoreValue>>) -> Self {
        let cc_store = [[0u16; 256]; 256]
            .iter()
            .map(|_| {
                [0u16; 256]
                    .iter()
                    .map(|_| AtomicOption::empty())
                    .collect_vec()
            })
            .collect_vec();

        Self { cc_store, events }
    }

    pub fn queue(&self) -> &Shared<Queue<MidiStoreValue>> {
        &self.events
    }

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
                self.events.push(MidiStoreValue {
                    channel,
                    controller_number,
                    value,
                });
            }
        }
    }
}

#[repr(C)]
pub enum MidiEvent {
    Value(MidiStoreValue),
}

pub struct MidiStoreActor {
    events_queue: Shared<Queue<MidiStoreValue>>,
    latest_events: VecDeque<MidiStoreValue>,
    current_cc_values: HashMap<u8, u8>,
    is_running: Shared<AtomicBool>,
    callback: Box<dyn Fn(MidiEvent) + Send>,
}

impl MidiStoreActor {
    pub fn new(
        events_queue: Shared<Queue<MidiStoreValue>>,
        is_running: Shared<AtomicBool>,
        callback: Box<dyn Fn(MidiEvent) + Send>,
    ) -> Self {
        Self {
            events_queue,
            latest_events: VecDeque::new(),
            current_cc_values: HashMap::new(),
            is_running,
            callback,
        }
    }

    pub fn run(&mut self) {
        while self.is_running.load(Ordering::Relaxed) {
            if let Some(event) = self.events_queue.pop() {
                self.current_cc_values
                    .insert(event.controller_number, event.value);
                self.latest_events.push_front(event.clone());
                self.latest_events.truncate(100);
                (self.callback)(MidiEvent::Value(event.clone()));
            }

            std::thread::sleep(Duration::from_millis(50))
        }
    }
}

pub trait MidiStoreActorDelegate {
    fn on_event(&self, event: MidiStoreValue);
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;
    use basedrop::Owned;
    use itertools::Itertools;

    use audio_garbage_collector::{handle, make_shared};
    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};

    use super::*;

    #[test]
    fn test_create_store() {
        let _store = MidiStoreHandle::new(make_shared(Queue::new(100)));
    }

    #[test]
    fn test_process_event() {
        let queue = make_shared(Queue::new(100));
        let store = MidiStoreHandle::new(queue.clone());
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

        let event = queue.pop().unwrap();
        assert_eq!(
            event,
            MidiStoreValue {
                channel: 0,
                controller_number: 55,
                value: 12
            }
        );
    }

    #[test]
    fn test_actor_state() {
        let queue = make_shared(Queue::new(100));
        let store = MidiStoreHandle::new(queue.clone());

        let actor_is_running = make_shared(AtomicBool::new(true));
        let mut actor = MidiStoreActor::new(queue, actor_is_running.clone(), Box::new(|_| {}));

        let handle = std::thread::spawn(move || actor.run());
        let message = MidiMessageEntry(Owned::new(
            audio_garbage_collector::handle(),
            MidiMessageWrapper {
                message_data: [0b1011_0000, 55, 12],
                timestamp: 0,
            },
        ));
        assert_no_alloc(|| {
            store.process_event(&message);
        });
        assert_no_alloc(|| {
            store.process_event(&message);
        });
        assert_no_alloc(|| {
            store.process_event(&message);
        });

        actor_is_running.store(false, Ordering::Relaxed);
        handle.join().unwrap();
    }
}
