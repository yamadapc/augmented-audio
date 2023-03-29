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

/// `rust-vst` compatibility for the MidiMessageLike trait
#[cfg(feature = "vst")]
pub mod vst;

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

/// An instance of MidiEventHandler that doesn't do anything with its events.
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

#[cfg(test)]
mod test {
    use super::*;

    struct Message {}
    impl MidiMessageLike for Message {
        fn is_midi(&self) -> bool {
            false
        }

        fn bytes(&self) -> Option<&[u8]> {
            None
        }
    }

    #[test]
    fn test_noop_midi_event_handler() {
        let mut handler = super::NoopMidiEventHandler::new();
        handler.process_midi_events::<Message>(&[]);
    }
}
