// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

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
