use std::cmp::{max, min};

use vst::api::{Event, Events, MidiEvent};

use crate::constants::MIDI_BUFFER_CAPACITY;
use crate::host::MidiMessageEntry;

/// This is an unsafe converter from MIDI events received from `midir` into the `rust-vst` VST api.
///
/// It's unsafe because it must do manual memory allocation & management to interface with the VST
/// C-style API.
///
/// Pre-allocates buffers of MIDI events up-front. Capacity is set to 100 MIDI messages by default.
///
/// More than 100 midi messages being passed into it in a single buffer tick will result in dropped
/// messages.
///
/// The collecting phase of the audio-thread should collect at most a limit of messages.
///
/// Threshold can be changed in the future to include more.
pub struct MidiVSTConverter {
    events: Box<vst::api::Events>,
    /// Events list here for freeing manually allocated memory.
    #[allow(dead_code, clippy::vec_box)]
    events_lst: Vec<Box<Event>>,
    capacity: usize,
}

impl Default for MidiVSTConverter {
    fn default() -> Self {
        Self::new(MIDI_BUFFER_CAPACITY)
    }
}

impl MidiVSTConverter {
    /// Create a new MidiVSTConverter with capacity.
    ///
    /// Will pre-allocate buffers.
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let events_ptr = MidiVSTConverter::allocate_events(capacity);
            let event_ptrs = std::slice::from_raw_parts_mut(
                &mut (*events_ptr).events[0] as *mut *mut _ as *mut *mut _,
                capacity,
            );
            let mut events_lst = Vec::with_capacity(capacity);
            for event_ptr_cell in event_ptrs.iter_mut().take(capacity) {
                let event_ptr = MidiVSTConverter::allocate_event();
                *event_ptr_cell = event_ptr;
                events_lst.push(Box::from_raw(*event_ptr_cell));
            }

            Self {
                events: Box::from_raw(events_ptr),
                events_lst,
                capacity,
            }
        }
    }

    /// Pushes MIDI messages onto a pre-allocated `Events` struct. Returns a reference to it.
    ///
    /// This should be real-time safe.
    ///
    /// The `vst::api::Events` returned may be passed into a VST plugin instance.
    pub fn accept(&mut self, midi_message_buffer: &[MidiMessageEntry]) -> &vst::api::Events {
        self.events.num_events = min(self.capacity as i32, midi_message_buffer.len() as i32);

        for (i, message) in midi_message_buffer.iter().enumerate() {
            if i >= self.capacity {
                log::trace!("MIDI Message was dropped");
                break;
            }

            unsafe {
                log::trace!("Forwarding message {:?}", message.message_data);
                let event = MidiEvent {
                    event_type: vst::api::EventType::Midi,
                    byte_size: std::mem::size_of::<MidiEvent>() as i32,
                    delta_frames: 0,
                    flags: 0,
                    note_length: 0,
                    note_offset: 0,
                    midi_data: message.message_data,
                    _midi_reserved: 0,
                    detune: 0,
                    note_off_velocity: 0,
                    _reserved1: 0,
                    _reserved2: 0,
                };

                let ptr = std::slice::from_raw_parts_mut(
                    &mut self.events.events[0] as *mut *mut Event,
                    self.events.num_events as usize,
                );
                let ptr_event_ref: *mut Event = ptr[i];
                let midi_event: *mut MidiEvent = std::mem::transmute(ptr_event_ref);
                *midi_event = event;
            }
        }

        &self.events
    }

    /// Get a reference to the events
    pub fn events(&self) -> &vst::api::Events {
        &self.events
    }

    /// Allocates a simple event. We're only using MIDI events for now, but should create enough
    /// space for any message type.
    unsafe fn allocate_event() -> *mut Event {
        let (event_size, event_align) = (
            max(
                std::mem::size_of::<vst::api::SysExEvent>(),
                max(
                    std::mem::size_of::<Event>(),
                    std::mem::size_of::<MidiEvent>(),
                ),
            ),
            std::mem::align_of::<Event>(),
        );
        let event_layout = std::alloc::Layout::from_size_align_unchecked(event_size, event_align);

        std::alloc::alloc(event_layout) as *mut Event
    }

    /// Allocates the `Events` struct. This is a C struct with a trailing array of events.
    /// The Rust declaration sizes this array as 2 elements ; here we append space for another
    /// capacity elements after it.
    unsafe fn allocate_events(capacity: usize) -> *mut Events {
        let event_ptr_size = std::mem::size_of::<*mut Event>();
        let events_layout = std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<vst::api::Events>() + event_ptr_size * capacity,
            std::mem::align_of::<vst::api::Events>(),
        );

        std::alloc::alloc(events_layout) as *mut vst::api::Events
    }
}

#[cfg(test)]
mod test {
    use basedrop::Owned;

    use audio_processor_traits::MidiMessageLike;

    use crate::host::MidiMessageWrapper;
    use assert_no_alloc::assert_no_alloc;

    use super::*;

    #[test]
    fn test_create_converter() {
        let _midi_vst_converter = MidiVSTConverter::new(10);
    }

    #[test]
    fn test_allocate_event() {
        // Just leak the event ptr
        let _event_ptr = unsafe { MidiVSTConverter::allocate_event() };
    }

    #[test]
    fn test_accept_events() {
        let mut converter = MidiVSTConverter::new(10);
        let buffer = [
            MidiMessageEntry(Owned::new(
                audio_garbage_collector::handle(),
                MidiMessageWrapper {
                    timestamp: 0,
                    message_data: [10, 20, 30],
                },
            )),
            MidiMessageEntry(Owned::new(
                audio_garbage_collector::handle(),
                MidiMessageWrapper {
                    timestamp: 10,
                    message_data: [30, 40, 50],
                },
            )),
        ];

        let events = assert_no_alloc(|| converter.accept(&buffer));
        assert_eq!(events.num_events, 2);

        let event = events.events[0];
        assert_eq!(event.is_midi(), true);
        let msg = [10_u8, 20, 30];
        assert_eq!(event.bytes(), Some(&msg as &[u8]));
        let event = events.events[1];
        assert_eq!(event.is_midi(), true);
        let msg = [30_u8, 40, 50];
        assert_eq!(event.bytes(), Some(&msg as &[u8]));
    }
}
