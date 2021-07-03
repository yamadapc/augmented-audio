use basedrop::Owned;
use vst::api::MidiEvent;

use crate::audio_io::midi::MidiMessageWrapper;
use std::cmp::min;

/// This is a super unsafe converter from MIDI events as received into the VST api. It's unsafe
/// because it must do manual memory allocation & management.
///
/// Pre-allocates buffers up-front. Capacity is set to 10 MIDI messages. More than 10 midi messages
/// being passed into it will result in dropped messages.
///
/// The collecting phase of the audio-thread should collect at most a limit of messages.
///
/// Threshold can be changed in the future to include more.
pub struct MidiConverter {
    events: Box<vst::api::Events>,
    #[allow(dead_code)]
    events_lst: Vec<Box<vst::api::Event>>,
}

impl MidiConverter {
    pub fn new() -> Self {
        unsafe {
            let (event_ptr_size, event_ptr_align) = (
                std::mem::size_of::<*mut vst::api::Event>(),
                std::mem::align_of::<*mut vst::api::Event>(),
            );
            let events_layout = std::alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<*mut vst::api::Events>() + event_ptr_size * 98,
                std::mem::align_of::<*mut vst::api::Events>(),
            );
            let events_ptr = std::alloc::alloc(events_layout) as *mut vst::api::Events;
            let event_ptrs = std::slice::from_raw_parts_mut(
                &mut (*events_ptr).events[0] as *mut *mut _ as *mut *mut _,
                100 as usize,
            );
            let mut events_lst = Vec::with_capacity(100);
            for i in 0..100 {
                let event_layout =
                    std::alloc::Layout::from_size_align_unchecked(event_ptr_size, event_ptr_align);
                let event_ptr = std::alloc::alloc(event_layout) as *mut vst::api::Event;
                event_ptrs[i] = event_ptr;
                events_lst.push(Box::from_raw(event_ptrs[i] as *mut vst::api::Event));
            }
            Self {
                events: Box::from_raw(events_ptr),
                events_lst,
            }
        }
    }

    pub fn events(&self) -> &vst::api::Events {
        &self.events
    }

    pub fn accept(&mut self, midi_message_buffer: &[Owned<MidiMessageWrapper>]) {
        self.events.num_events = min(100, midi_message_buffer.len() as i32);

        for (i, message) in midi_message_buffer.into_iter().enumerate() {
            if i >= 100 {
                log::error!("Message was dropped");
                return;
            }

            unsafe {
                let event = vst::api::MidiEvent {
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
                let mut event: vst::api::Event = std::mem::transmute(event);
                event.event_type = vst::api::EventType::Midi;

                let ptr = std::slice::from_raw_parts_mut(
                    &mut self.events.events[0] as *mut *mut _ as *mut *mut _,
                    self.events.num_events as usize,
                );
                let in_place_ptr = ptr[i];
                *in_place_ptr = event;
            }
        }
    }
}
