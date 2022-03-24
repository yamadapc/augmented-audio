use crate::constants::MIDI_BUFFER_CAPACITY;
use crate::host::{MidiMessageEntry, MidiMessageQueue};

/// Audio-thread side of MIDI handling.
///
/// Pops MIDI events from the the MIDI queue & collects them on a pre-allocated fixed capacity
/// vector.
pub struct MidiAudioThreadHandler {
    buffer: Vec<MidiMessageEntry>,
    capacity: usize,
}

impl Default for MidiAudioThreadHandler {
    fn default() -> Self {
        Self::new(MIDI_BUFFER_CAPACITY)
    }
}

impl MidiAudioThreadHandler {
    pub fn new(capacity: usize) -> Self {
        MidiAudioThreadHandler {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Get a reference to the message buffer
    pub fn buffer(&self) -> &Vec<MidiMessageEntry> {
        &self.buffer
    }

    /// Push messages onto the buffer
    ///
    /// This is real-time safe as long as `MidiAudioThreadHandler::clear` is called on every tick.
    pub fn collect_midi_messages(&mut self, midi_message_queue: &MidiMessageQueue) -> usize {
        let mut midi_message_count = 0;
        for _i in 0..self.capacity {
            if let Some(midi_message) = midi_message_queue.pop() {
                self.buffer.push(midi_message);
                midi_message_count += 1;
            } else {
                return midi_message_count;
            }
        }
        midi_message_count
    }

    /// Clear the messages buffer. Must be called after `collect_midi_messages` on every tick.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;
    use basedrop::{Collector, Owned};

    use audio_processor_traits::MidiMessageLike;

    use crate::host::MidiMessageWrapper;

    use super::*;

    #[test]
    fn test_create_handler_and_collect_empty_messages() {
        let mut collector = Collector::new();
        let handle = collector.handle();
        let queue = MidiMessageQueue::new(&handle, atomic_queue::Queue::new(MIDI_BUFFER_CAPACITY));

        let mut midi_audio_thread_handler = MidiAudioThreadHandler::default();
        let num_messages = midi_audio_thread_handler.collect_midi_messages(&queue);
        assert_eq!(num_messages, 0);
        let buffer = midi_audio_thread_handler.buffer();
        assert_eq!(buffer.len(), 0);

        collector.collect();
    }

    #[test]
    fn test_create_handler_and_collect_some_messages() {
        let mut collector = Collector::new();
        let handle = collector.handle();
        let queue = MidiMessageQueue::new(&handle, atomic_queue::Queue::new(MIDI_BUFFER_CAPACITY));
        queue.push(MidiMessageEntry(Owned::new(
            &handle,
            MidiMessageWrapper {
                message_data: [128, 0, 12],
                timestamp: 0,
            },
        )));
        queue.push(MidiMessageEntry(Owned::new(
            &handle,
            MidiMessageWrapper {
                message_data: [129, 0, 12],
                timestamp: 0,
            },
        )));
        queue.push(MidiMessageEntry(Owned::new(
            &handle,
            MidiMessageWrapper {
                message_data: [130, 0, 12],
                timestamp: 0,
            },
        )));

        let mut midi_audio_thread_handler = MidiAudioThreadHandler::default();

        let num_messages =
            assert_no_alloc(|| midi_audio_thread_handler.collect_midi_messages(&queue));

        assert_eq!(num_messages, 3);
        let buffer = midi_audio_thread_handler.buffer();
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer[0].is_midi(), true);
        assert_eq!(buffer[0].message_data, [128, 0, 12]);
        assert_eq!(buffer[1].message_data, [129, 0, 12]);
        assert_eq!(buffer[2].message_data, [130, 0, 12]);

        collector.collect();
    }

    #[test]
    fn test_create_handler_and_clear() {
        let mut collector = Collector::new();
        let handle = collector.handle();
        let queue = MidiMessageQueue::new(&handle, atomic_queue::Queue::new(MIDI_BUFFER_CAPACITY));
        queue.push(MidiMessageEntry(Owned::new(
            &handle,
            MidiMessageWrapper {
                message_data: [128, 0, 12],
                timestamp: 0,
            },
        )));

        let mut midi_audio_thread_handler = MidiAudioThreadHandler::default();
        let num_messages =
            assert_no_alloc(|| midi_audio_thread_handler.collect_midi_messages(&queue));
        assert_eq!(num_messages, 1);
        let buffer = midi_audio_thread_handler.buffer();
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.capacity(), MIDI_BUFFER_CAPACITY);
        assert_eq!(queue.is_empty(), true);
        midi_audio_thread_handler.clear();
        let buffer = midi_audio_thread_handler.buffer();
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), MIDI_BUFFER_CAPACITY);

        collector.collect();
    }
}
