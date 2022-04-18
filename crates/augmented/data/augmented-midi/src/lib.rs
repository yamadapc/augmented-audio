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
//! Implements a MIDI (file) parser/serializer using `nom` and `cookie-factory` combinators.
//!
//! Thanks to the combinators this library requires no allocation for
//! serialization/de-serialization into/from the MIDI types provided.
//!
//! _(This is part of [augmented-audio](https://github.com/yamadapc/augmented-audio/))_
//!
//! # Specification
//! Based on MIDI 1.0 specification. [MIDI 1.0](https://www.midi.org/specifications/midi1-specifications/m1-v4-2-1-midi-1-0-detailed-specification-96-1-4.)
//!
//! # License notes
//! There's a Bach MIDI file used from `piano-midi.de` linked here. This file is licensed as described
//! in <http://www.piano-midi.de/copy.htm>. Name: Bernd Krueger
//! The distribution or public playback of the files is only allowed under identical license conditions.
//! The scores are open source.
//!
//! # Parsing a single message
//! We can parse a single MIDI message as follows.
//!
//! ```rust
//! use augmented_midi::{MIDIMessage, MIDIMessageNote, MIDIParseResult, parse_midi_event, ParserState};
//!
//! // Initialize parser state. This is here to support rolling status on MIDI files
//! let mut state = ParserState::default();
//!
//! // We'll parse this &[u8] buffer. This could be a vec
//! let input_buffer = [0x9_8, 0x3C, 0x44];
//!
//! // We parse a message borrowing from the input buffer. We could use `MIDIMessage<Vec<u8>>` to
//! // allocate owned messages.
//! //
//! // This is only relevant for variable size messages like SysEx.
//! //
//! // Parsing is otherwise only using the stack.
//! let parse_result: MIDIParseResult<MIDIMessage<&[u8]>> =
//!     parse_midi_event(&input_buffer, &mut state);
//! let (_remaining_input, midi_message) = parse_result.unwrap();
//!
//! assert_eq!(midi_message, MIDIMessage::NoteOn(MIDIMessageNote { channel: 8, note: 60, velocity: 68 }));
//! ```
//!
//! # Serializing messages
//!
//! ```
//! use augmented_midi::{serialize_message, MIDIMessage};
//!
//! let mut  writer = [0_u8;3]; // This could be a vec.
//! let message: MIDIMessage<Vec<u8>> = MIDIMessage::control_change(0, 55, 127); // CC#55 127 - channel 0
//! let _ = serialize_message(message, &mut writer[..]).unwrap();
//!
//! assert_eq!(writer, [0xB0, 0x37, 0x7F]);
//! ```
pub use cookie_factory;
pub use nom;
pub use parser::*;
pub use serializer::serialize_message;
pub use types::*;

mod parser;
mod serializer;
mod types;
