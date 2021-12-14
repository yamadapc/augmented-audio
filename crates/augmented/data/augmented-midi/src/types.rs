use std::borrow::Borrow;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDIMessageNote {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub enum MIDIMessage<Buffer: Borrow<[u8]>> {
    // 0x9
    NoteOn(MIDIMessageNote),
    // 0x8
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
    // ChannelModeMessage {
    //     channel: u8,
    //     controller_number: u8,
    //     value: u8,
    // },
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
    pub fn note_on(channel: u8, note: u8, velocity: u8) -> Self {
        MIDIMessage::NoteOn(MIDIMessageNote {
            channel,
            note,
            velocity,
        })
    }

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
}

pub type Input<'a> = &'a [u8];

#[derive(Debug, PartialEq)]
pub enum MIDIFileFormat {
    // 0
    Single,
    // 1
    Simultaneous,
    // 2
    Sequential,
    Unknown,
}

#[derive(Debug)]
pub enum MIDIFileDivision {
    // 0
    TicksPerQuarterNote { ticks_per_quarter_note: u16 },
    // 1
    SMPTE { format: u8, ticks_per_frame: u8 },
}

#[derive(Debug)]
pub struct MIDIFileHeader {
    pub format: MIDIFileFormat,
    pub num_tracks: u16,
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
    SysEx(MIDISysExEvent<Buffer>),
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
