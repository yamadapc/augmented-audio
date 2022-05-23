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
use crate::{parse_midi_event, serialize_message, ParserState};
use std::borrow::Borrow;

/// Type of node-on or note-off contents
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDIMessageNote {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}

/// Represents a MIDI message
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub enum MIDIMessage<Buffer: Borrow<[u8]>> {
    NoteOn(MIDIMessageNote),
    NoteOff(MIDIMessageNote),
    PolyphonicKeyPressure {
        channel: u8,
        note: u8,
        pressure: u8,
    },
    ControlChange {
        channel: u8,
        controller_number: u8,
        value: u8,
    },
    ProgramChange {
        channel: u8,
        program_number: u8,
    },
    ChannelPressure {
        channel: u8,
        pressure: u8,
    },
    PitchWheelChange {
        channel: u8,
        value: u16,
    },
    SysExMessage(MIDISysExEvent<Buffer>),
    SongPositionPointer {
        beats: u16,
    },
    SongSelect {
        song: u8,
    },
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
    Other {
        status: u8,
    },
}

impl<Buffer: Borrow<[u8]>> MIDIMessage<Buffer> {
    /// Helper to construct `MIDIMessage::NoteOn`
    pub fn note_on(channel: u8, note: u8, velocity: u8) -> Self {
        MIDIMessage::NoteOn(MIDIMessageNote {
            channel,
            note,
            velocity,
        })
    }

    /// Helper to construct `MIDIMessage::NoteOff`
    pub fn note_off(channel: u8, note: u8, velocity: u8) -> Self {
        MIDIMessage::NoteOff(MIDIMessageNote {
            channel,
            note,
            velocity,
        })
    }

    pub fn control_change(channel: u8, controller_number: u8, value: u8) -> Self {
        MIDIMessage::ControlChange {
            channel,
            controller_number,
            value,
        }
    }

    /// This returns the size in bytes of this message when serialised into MIDI.
    pub fn size_hint(&self) -> usize {
        match self {
            MIDIMessage::NoteOn(_) => 3,
            MIDIMessage::NoteOff(_) => 3,
            MIDIMessage::PolyphonicKeyPressure { .. } => 3,
            MIDIMessage::ControlChange { .. } => 3,
            MIDIMessage::ProgramChange { .. } => 2,
            MIDIMessage::ChannelPressure { .. } => 2,
            MIDIMessage::PitchWheelChange { .. } => 3,
            MIDIMessage::SysExMessage(inner) => 2 + inner.message.borrow().len(),
            MIDIMessage::SongPositionPointer { .. } => 3,
            MIDIMessage::SongSelect { .. } => 2,
            MIDIMessage::TuneRequest => 1,
            MIDIMessage::TimingClock => 1,
            MIDIMessage::Start => 1,
            MIDIMessage::Continue => 1,
            MIDIMessage::Stop => 1,
            MIDIMessage::ActiveSensing => 1,
            MIDIMessage::Reset => 1,
            MIDIMessage::Other { .. } => 1,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for MIDIMessage<&'a [u8]> {
    type Error = nom::Err<nom::error::Error<&'a [u8]>>;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let mut state = ParserState::default();
        let (_, message) = parse_midi_event(value, &mut state)?;
        Ok(message)
    }
}

impl<B: Borrow<[u8]>> From<MIDIMessage<B>> for Vec<u8> {
    fn from(msg: MIDIMessage<B>) -> Vec<u8> {
        let mut output = vec![];
        let _ = serialize_message(msg, &mut output);
        output
    }
}

pub type Input<'a> = &'a [u8];

/// Describes how the file is organized
#[derive(Debug, PartialEq)]
pub enum MIDIFileFormat {
    /// The file contains a single multi-channel track
    ///
    /// Represented by 0b0_000_0000_0000_0000
    Single,
    /// The file contains one or more simultaneous tracks (or MIDI outputs) of a
    /// sequence
    ///
    /// Represented by 0b1_000_0000_0000_0000
    Simultaneous,
    /// The file contains one or more sequentially independent single-track patterns
    ///
    /// Represented by 0b2_000_0000_0000_0000
    Sequential,
    /// An unknown file format was found. Parse will continue if this is found in the header chunk,
    /// it will possibly fail on other sections of the file.
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum MIDIFileDivision {
    /// The `ticks_per_quarter_note` field contains how many [`MIDITrackEvent::delta_time`] ticks
    /// make up a quarter-note.
    ///
    /// Represented by 0b0_000_0000_0000_0000
    TicksPerQuarterNote { ticks_per_quarter_note: u16 },
    /// Indicates [`MIDITrackEvent::delta_time`] are time based SMPTE offsets.
    ///
    /// Represented by 0b1_000_0000_0000_0000
    SMPTE { format: u8, ticks_per_frame: u8 },
}

/// The header chunk's contents
#[derive(Debug, PartialEq)]
pub struct MIDIFileHeader {
    /// How the file is organized
    pub format: MIDIFileFormat,
    /// The number of tracks in the file
    pub num_tracks: u16,
    /// Specifies the meaning of the delta times in events
    pub division: MIDIFileDivision,
}

#[derive(Debug)]
pub enum MIDIFileChunk<StringRepr: Borrow<str>, Buffer: Borrow<[u8]>> {
    Header(MIDIFileHeader),
    Track { events: Vec<MIDITrackEvent<Buffer>> },
    Unknown { name: StringRepr, body: Buffer },
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub enum MIDITrackInner<Buffer: Borrow<[u8]>> {
    Message(MIDIMessage<Buffer>),
    Meta(MIDIMetaEvent<Buffer>),
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDITrackEvent<Buffer: Borrow<[u8]>> {
    pub delta_time: u32,
    pub inner: MIDITrackInner<Buffer>,
}

impl<Buffer: Borrow<[u8]>> MIDITrackEvent<Buffer> {
    pub fn new(delta_time: u32, event: MIDITrackInner<Buffer>) -> Self {
        MIDITrackEvent {
            delta_time,
            inner: event,
        }
    }

    pub fn delta_time(&self) -> u32 {
        self.delta_time
    }

    pub fn message(&self) -> Option<&MIDIMessage<Buffer>> {
        match &self.inner {
            MIDITrackInner::Message(message) => Some(message),
            _ => None,
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDIMetaEvent<Buffer: Borrow<[u8]>> {
    pub meta_type: u8,
    pub length: u32,
    pub bytes: Buffer,
}

impl<Buffer: Borrow<[u8]>> MIDIMetaEvent<Buffer> {
    pub fn new(meta_type: u8, length: u32, bytes: Buffer) -> Self {
        MIDIMetaEvent {
            meta_type,
            length,
            bytes,
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDISysExEvent<Buffer: Borrow<[u8]>> {
    pub message: Buffer,
}

impl<Buffer: Borrow<[u8]>> MIDISysExEvent<Buffer> {
    pub fn new(message: Buffer) -> Self {
        MIDISysExEvent { message }
    }
}

#[derive(Debug)]
pub struct MIDIFile<StringRepr: Borrow<str>, Buffer: Borrow<[u8]>> {
    pub chunks: Vec<MIDIFileChunk<StringRepr, Buffer>>,
}

impl<StringRepr: Borrow<str>, Buffer: Borrow<[u8]>> MIDIFile<StringRepr, Buffer> {
    pub fn new(chunks: Vec<MIDIFileChunk<StringRepr, Buffer>>) -> Self {
        Self { chunks }
    }

    pub fn chunks(&self) -> &Vec<MIDIFileChunk<StringRepr, Buffer>> {
        &self.chunks
    }

    pub fn header(&self) -> Option<&MIDIFileHeader> {
        self.chunks.iter().find_map(|chunk| match chunk {
            MIDIFileChunk::Header(header) => Some(header),
            _ => None,
        })
    }

    pub fn track_chunks(&self) -> impl Iterator<Item = &MIDITrackEvent<Buffer>> {
        self.chunks
            .iter()
            .filter_map(|chunk| match chunk {
                MIDIFileChunk::Track { events } => Some(events),
                _ => None,
            })
            .flatten()
    }

    pub fn ticks_per_quarter_note(&self) -> u16 {
        if let Some(MIDIFileHeader {
            division:
                MIDIFileDivision::TicksPerQuarterNote {
                    ticks_per_quarter_note,
                },
            ..
        }) = self.header()
        {
            *ticks_per_quarter_note
        } else {
            0
        }
    }
}
