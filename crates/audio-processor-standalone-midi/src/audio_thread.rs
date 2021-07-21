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
