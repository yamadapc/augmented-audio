use nom::bytes::complete::tag;
use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::take_till,
    error::FromExternalError,
    error::{Error, ErrorKind},
    multi::many0,
    multi::many1,
    number::complete::*,
    Err, IResult,
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub enum MIDIMessage<'a> {
    // 0x9
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    // 0x8
    NoteOff {
        channel: u8,
        note: u8,
        velocity: u8,
    },
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
    ChannelModeMessage {
        channel: u8,
        controller_number: u8,
        value: u8,
    },
    SysExMessage(MIDISysExEvent<'a>),
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

type Input<'a> = &'a [u8];
type Result<'a, Output> = IResult<Input<'a>, Output>;

#[derive(Debug)]
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
pub enum MIDIFileChunk<'a> {
    Header {
        format: MIDIFileFormat,
        num_tracks: u16,
        division: MIDIFileDivision,
    },
    Track {
        events: Vec<MIDITrackEvent<'a>>,
    },
    Unknown {
        name: &'a str,
        body: &'a [u8],
    },
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
enum MIDITrackInner<'a> {
    Message(MIDIMessage<'a>),
    Meta(MIDIMetaEvent<'a>),
    SysEx(MIDISysExEvent<'a>),
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDITrackEvent<'a> {
    delta_time: u32,
    event: MIDITrackInner<'a>,
}

pub fn parse_header_body(input: Input) -> Result<MIDIFileChunk> {
    let (input, format) = be_u16(input)?;
    let format = match format {
        0 => Ok(MIDIFileFormat::Single),
        1 => Ok(MIDIFileFormat::Simultaneous),
        2 => Ok(MIDIFileFormat::Sequential),
        _ => Ok(MIDIFileFormat::Unknown),
    }?;
    let (input, num_tracks) = be_u16(input)?;
    let (input, division_word) = be_u16(input)?;

    let division_type = division_word >> 15;
    let (input, division) = match division_type {
        0 => {
            let ticks_per_quarter_note = (division_word << 1) >> 1;
            Ok((
                input,
                MIDIFileDivision::TicksPerQuarterNote {
                    ticks_per_quarter_note,
                },
            ))
        }
        1 => {
            let format = ((division_word << 1) >> 8) as u8;
            let ticks_per_frame = ((division_word << 8) >> 8) as u8;
            Ok((
                input,
                MIDIFileDivision::SMPTE {
                    format,
                    ticks_per_frame,
                },
            ))
        }
        _ => Err(Err::Error(Error::new(input, ErrorKind::Fail))),
    }?;

    Ok((
        input,
        MIDIFileChunk::Header {
            format,
            num_tracks,
            division,
        },
    ))
}

// https://en.wikipedia.org/wiki/Variable-length_quantity
pub fn parse_variable_length_num(input: Input) -> Result<u32> {
    use nom::bytes::complete::*;

    let mut result: u32 = 0;

    let (input, parts) = take_till(|b| b & 0b10000000 == 0)(input)?;
    let (input, extra_part) = take(1u8)(input)?;

    let mut i = parts.len() + 1;
    for part in parts.into_iter().chain(extra_part.into_iter()) {
        i -= 1;
        let part = (part << 1) >> 1;
        let part32 = part as u32;
        result += part32 << (i * 7);
    }

    Ok((input, result))
}

pub fn parse_midi_event<'a>(
    input: Input<'a>,
    state: &mut ParserState,
) -> Result<'a, MIDIMessage<'a>> {
    let (tmp_input, tmp_status) = be_u8(input)?;
    let (input, status) = if tmp_status >= 0x7F {
        state.last_status = Some(tmp_status);
        Ok((tmp_input, tmp_status))
    } else if let Some(status) = state.last_status {
        Ok((input, status))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    }?;

    let status_start = status & 0b1111_0000;
    let (input, message) = if status_start == 0b1000_0000 {
        let channel = parse_channel(status);
        let (input, note) = be_u8(input)?;
        let (input, velocity) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::NoteOff {
                channel,
                note,
                velocity,
            },
        ))
    } else if status_start == 0b1001_0000 {
        let channel = parse_channel(status);
        let (input, note) = be_u8(input)?;
        let (input, velocity) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::NoteOn {
                channel,
                note,
                velocity,
            },
        ))
    } else if status_start == 0b1010_0000 {
        let channel = parse_channel(status);
        let (input, note) = be_u8(input)?;
        let (input, pressure) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::PolyphonicKeyPressure {
                channel,
                note,
                pressure,
            },
        ))
    } else if status_start == 0b1011_0000 {
        let channel = parse_channel(status);
        let (input, controller_number) = be_u8(input)?;
        let (input, value) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::ControlChange {
                channel,
                controller_number,
                value,
            },
        ))
    } else if status_start == 0b1100_0000 {
        let channel = parse_channel(status);
        let (input, program_number) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::ProgramChange {
                channel,
                program_number,
            },
        ))
    } else if status_start == 0b1101_0000 {
        let channel = parse_channel(status);
        let (input, pressure) = be_u8(input)?;
        Ok((input, MIDIMessage::ChannelPressure { channel, pressure }))
    } else if status_start == 0b1110_0000 {
        let channel = parse_channel(status);
        let (input, value) = parse_14bit_midi_number(input)?;
        Ok((input, MIDIMessage::PitchWheelChange { channel, value }))
    } else if status_start == 0b1011_0000 {
        let channel = parse_channel(status);
        let (input, controller_number) = be_u8(input)?;
        let (input, value) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::ChannelModeMessage {
                channel,
                controller_number,
                value,
            },
        ))
    } else if status == 0b1111_0000 {
        let (input, sysex_message) = take_till(|b| b == 0b11110111)(input)?;
        let (input, extra) = take(1u8)(input)?;
        assert!(extra.len() == 0 && extra[0] == 0b11110111);
        Ok((
            input,
            MIDIMessage::SysExMessage(MIDISysExEvent {
                message: sysex_message,
            }),
        ))
    } else if status == 0b1111_0010 {
        let (input, value) = parse_14bit_midi_number(input)?;
        Ok((input, MIDIMessage::SongPositionPointer { beats: value }))
    } else if status == 0b1111_0011 {
        let (input, song) = be_u8(input)?;
        Ok((input, MIDIMessage::SongSelect { song }))
    } else if status == 0b1111_1000 {
        Ok((input, MIDIMessage::TimingClock))
    } else if status == 0b1111_1010 {
        Ok((input, MIDIMessage::Start))
    } else if status == 0b1111_1011 {
        Ok((input, MIDIMessage::Continue))
    } else if status == 0b1111_1100 {
        Ok((input, MIDIMessage::Stop))
    } else if status == 0b1111_1110 {
        Ok((input, MIDIMessage::ActiveSensing))
    } else if status == 0b1111_1111 {
        Ok((input, MIDIMessage::Reset))
    } else {
        Ok((input, MIDIMessage::Other { status }))
    }?;

    Ok((input, message))
}

fn parse_channel(status: u8) -> u8 {
    status & 0b0000_1111
}

fn parse_14bit_midi_number(input: Input) -> Result<u16> {
    let (input, value1) = be_u8(input)?;
    let (input, value2) = be_u8(input)?;
    let value1 = ((value1 & !0b1000_0000) as u16) << 7;
    let value2 = (value2 & !0b1000_0000) as u16;
    let value = value1 + value2;
    Ok((input, value))
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDIMetaEvent<'a> {
    meta_type: u8,
    length: u32,
    bytes: &'a [u8],
}

pub fn parse_meta_event(input: Input) -> Result<MIDIMetaEvent> {
    let (input, _) = tag([0xFF])(input)?;
    let (input, meta_type) = be_u8(input)?;
    let (input, length) = parse_variable_length_num(input)?;
    let (input, bytes) = take(length)(input)?;

    Ok((
        input,
        MIDIMetaEvent {
            meta_type,
            length,
            bytes,
        },
    ))
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct MIDISysExEvent<'a> {
    message: &'a [u8],
}

pub fn parse_sysex_event(input: Input) -> Result<MIDISysExEvent> {
    let (input, _) = alt((tag([0xF7]), tag([0xF0])))(input)?;
    let (input, bytes) = take_till(|b| b == 0xF7)(input)?;
    let (input, _) = take(1u8)(input)?;
    Ok((input, MIDISysExEvent { message: bytes }))
}

pub fn parse_track_event<'a>(
    input: Input<'a>,
    state: &mut ParserState,
) -> Result<'a, MIDITrackEvent<'a>> {
    let (input, delta_time) = parse_variable_length_num(input)?;
    let (input, event) = alt((
        |input| parse_meta_event(input).map(|(input, event)| (input, MIDITrackInner::Meta(event))),
        |input| {
            parse_sysex_event(input).map(|(input, event)| (input, MIDITrackInner::SysEx(event)))
        },
        |input| {
            parse_midi_event(input, state)
                .map(|(input, event)| (input, MIDITrackInner::Message(event)))
        },
    ))(input)?;

    match event {
        MIDITrackInner::Meta(_) => {
            state.last_status = None;
        }
        MIDITrackInner::SysEx(_) => {
            state.last_status = None;
        }
        _ => {}
    }

    Ok((input, MIDITrackEvent { delta_time, event }))
}

#[derive(Default)]
pub struct ParserState {
    last_status: Option<u8>,
}

pub fn parse_chunk(input: Input) -> Result<MIDIFileChunk> {
    let (input, chunk_name) = take(4u32)(input)?;
    let chunk_name: &str = std::str::from_utf8(chunk_name)
        .map_err(|err| Err::Failure(Error::from_external_error(input, ErrorKind::Fail, err)))?;

    let (input, chunk_length) = parse_chunk_length(input)?;
    let (input, chunk_body) = take(chunk_length)(input)?;

    let (_, chunk) = match chunk_name {
        "MThd" => {
            assert_eq!(chunk_length, 6);
            parse_header_body(chunk_body)
        }
        "MTrk" => {
            let mut state = ParserState::default();
            let parse = |input| parse_track_event(input, &mut state);
            let (chunk_body, events) = many1(parse)(chunk_body)?;
            Ok((chunk_body, MIDIFileChunk::Track { events }))
        }
        _ => Ok((
            chunk_body,
            MIDIFileChunk::Unknown {
                name: chunk_name,
                body: chunk_body,
            },
        )),
    }?;

    Ok((input, chunk))
}

fn parse_chunk_length(input: Input) -> Result<u32> {
    u32(nom::number::Endianness::Big)(input)
}

pub fn parse_midi_file(input: Input) -> Result<Vec<MIDIFileChunk>> {
    let (input, chunks) = many0(parse_chunk)(input)?;
    Ok((input, chunks))
}

pub fn parse_midi(input: Input) -> Result<Vec<MIDIMessage>> {
    many0(|input| parse_midi_event(input, &mut ParserState::default()))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_variable_length_quantity_length_1() {
        assert_eq!(127, parse_variable_length_num(&[0x7F]).unwrap().1);
    }

    #[test]
    fn test_parse_variable_length_quantity_length_more_than_2() {
        assert_eq!(128, parse_variable_length_num(&[0x81, 0x00]).unwrap().1);
        assert_eq!(
            16384,
            parse_variable_length_num(&[0x81, 0x80, 0x00]).unwrap().1
        );
    }

    #[test]
    fn test_parse_14bit_midi_number() {
        // Example pitch change on channel 3
        // let pitch_wheel_message = [0xE3, 0x39, 0x54];
        let (_, result) = parse_14bit_midi_number(&[0x39, 0x54]).unwrap();
        assert_eq!(result, 7380);
    }

    #[test]
    fn test_parse_pitch_wheel_event() {
        // Example pitch change on channel 3
        let pitch_wheel_message = [0xE3, 0x39, 0x54];
        let (_, result) =
            parse_midi_event(&pitch_wheel_message, &mut ParserState::default()).unwrap();
        assert_eq!(
            result,
            MIDIMessage::PitchWheelChange {
                channel: 3,
                value: 7380
            }
        );
    }

    #[test]
    fn test_parse_midi_file_smoke_test() {
        let input_path = format!("{}/bach_846.mid", env!("CARGO_MANIFEST_DIR"));
        let file_contents = std::fs::read(input_path).unwrap();
        // let file_contents: Vec<u8> = file_contents.into_iter().take(8000).collect();
        let (_rest, midi_stream) = parse_midi_file(&file_contents).unwrap();
        println!("{:?}", midi_stream);
    }
}
