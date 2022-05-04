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
use std::borrow::Borrow;

use nom::bytes::complete::tag;
use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::take_till,
    error::FromExternalError,
    error::{Error, ErrorKind},
    multi::many0,
    number::complete::*,
    Err, IResult,
};

pub use crate::types::*;

/// These are bit-masks

// The first 4 bits of the status byte indicate message type. This bitmask extracts that
// section to match against the masks below.
/// bit-mask to match the status byte
pub const STATUS_BYTE_MASK: u8 = 0b1111_0000;

// Bit-masks for each of the statuses, the 2nd 4 bits indicate the MIDI channel
pub const CONTROL_CHANGE_MASK: u8 = 0b1011_0000;
pub const NOTE_OFF_MASK: u8 = 0b1000_0000;
pub const NOTE_ON_MASK: u8 = 0b1001_0000;
pub const POLYPHONIC_KEY_PRESSURE_MASK: u8 = 0b1010_0000;
pub const PROGRAM_CHANGE_MASK: u8 = 0b1100_0000;
pub const CHANNEL_PRESSURE_MASK: u8 = 0b1101_0000;
pub const PITCH_WHEEL_CHANGE_MASK: u8 = 0b1110_0000;

// All these messages start with 0b1111, the 2nd 4 bits are part of the status
pub const SONG_POSITION_POINTER_MASK: u8 = 0b1111_0010;
pub const SONG_SELECT_MASK: u8 = 0b1111_0011;
pub const TIMING_CLOCK_MASK: u8 = 0b1111_1000;
pub const START_MASK: u8 = 0b1111_1010;
pub const CONTINUE_MASK: u8 = 0b1111_1011;
pub const STOP_MASK: u8 = 0b1111_1100;
pub const ACTIVE_SENSING_MASK: u8 = 0b1111_1110;
pub const RESET_MASK: u8 = 0b1111_1111;
pub const TUNE_REQUEST_MASK: u8 = 0b1111_0110;

pub const SYSEX_MESSAGE_MASK: u8 = 0b1111_0000;
pub const SYSEX_MESSAGE_END_MASK: u8 = 0b11110111;

pub type MIDIParseResult<'a, Output> = IResult<Input<'a>, Output>;

/// Parses 3 16bit words. In order:
///
/// * File format
/// * Number of tracks
/// * Division
pub fn parse_header_body(input: Input) -> MIDIParseResult<MIDIFileHeader> {
    let (input, format) = parse_file_format(input)?;
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
            let format = ((division_word << 1) >> 9) as u8;
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
        MIDIFileHeader {
            format,
            num_tracks,
            division,
        },
    ))
}

fn parse_file_format(input: Input) -> MIDIParseResult<MIDIFileFormat> {
    let (input, format) = be_u16(input)?;
    let format = match format {
        0 => Ok(MIDIFileFormat::Single),
        1 => Ok(MIDIFileFormat::Simultaneous),
        2 => Ok(MIDIFileFormat::Sequential),
        _ => Ok(MIDIFileFormat::Unknown),
    }?;
    Ok((input, format))
}

// https://en.wikipedia.org/wiki/Variable-length_quantity
pub fn parse_variable_length_num(input: Input) -> MIDIParseResult<u32> {
    use nom::bytes::complete::*;

    let mut result: u32 = 0;

    let (input, parts) = take_till(|b| b & 0b10000000 == 0)(input)?;
    let (input, extra_part) = take(1u8)(input)?;

    let mut i = parts.len() + 1;
    for part in parts.iter().chain(extra_part.iter()) {
        i -= 1;
        let part = (part << 1) >> 1;
        let part32 = part as u32;
        result += part32 << (i * 7);
    }

    Ok((input, result))
}

pub fn parse_midi_event<'a, Buffer: Borrow<[u8]> + From<Input<'a>>>(
    input: Input<'a>,
    state: &mut ParserState,
) -> MIDIParseResult<'a, MIDIMessage<Buffer>> {
    let (tmp_input, tmp_status) = be_u8(input)?;
    // Handle rolling status, this is look-ahead into the status byte and check
    // if it's valid, otherwise try using the previous status.
    let (input, status) = if tmp_status >= 0x7F {
        state.last_status = Some(tmp_status);
        Ok((tmp_input, tmp_status))
    } else if let Some(status) = state.last_status {
        Ok((input, status))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Fail)))
    }?;

    let status_start = status & STATUS_BYTE_MASK;
    let (input, message) = if status_start == NOTE_OFF_MASK {
        let channel = parse_channel(status);
        let (input, note) = be_u8(input)?;
        let (input, velocity) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::NoteOff(MIDIMessageNote {
                channel,
                note,
                velocity,
            }),
        ))
    } else if status_start == NOTE_ON_MASK {
        let channel = parse_channel(status);
        let (input, note) = be_u8(input)?;
        let (input, velocity) = be_u8(input)?;
        let note = MIDIMessageNote {
            channel,
            note,
            velocity,
        };
        Ok((input, MIDIMessage::NoteOn(note)))
    } else if status_start == POLYPHONIC_KEY_PRESSURE_MASK {
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
    } else if status_start == CONTROL_CHANGE_MASK {
        // Could potentially detect channel mode change here, but message is the same, the
        // applications can handle this.
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
    } else if status_start == PROGRAM_CHANGE_MASK {
        let channel = parse_channel(status);
        let (input, program_number) = be_u8(input)?;
        Ok((
            input,
            MIDIMessage::ProgramChange {
                channel,
                program_number,
            },
        ))
    } else if status_start == CHANNEL_PRESSURE_MASK {
        let channel = parse_channel(status);
        let (input, pressure) = be_u8(input)?;
        Ok((input, MIDIMessage::ChannelPressure { channel, pressure }))
    } else if status_start == PITCH_WHEEL_CHANGE_MASK {
        let channel = parse_channel(status);
        let (input, value) = parse_14bit_midi_number(input)?;
        Ok((input, MIDIMessage::PitchWheelChange { channel, value }))
    } else if status == SYSEX_MESSAGE_MASK {
        let (input, sysex_message) = take_till(|b| b == SYSEX_MESSAGE_END_MASK)(input)?;
        let (input, _extra) = take(1u8)(input)?;
        // assert!(extra.is_empty() && extra[0] == SYSEX_MESSAGE_END_MASK);
        let sysex_message = MIDISysExEvent {
            message: sysex_message.into(),
        };
        Ok((input, MIDIMessage::SysExMessage(sysex_message)))
    } else if status == SONG_POSITION_POINTER_MASK {
        let (input, value) = parse_14bit_midi_number(input)?;
        Ok((input, MIDIMessage::SongPositionPointer { beats: value }))
    } else if status == SONG_SELECT_MASK {
        let (input, song) = be_u8(input)?;
        Ok((input, MIDIMessage::SongSelect { song }))
    } else if status == TIMING_CLOCK_MASK {
        Ok((input, MIDIMessage::TimingClock))
    } else if status == START_MASK {
        Ok((input, MIDIMessage::Start))
    } else if status == CONTINUE_MASK {
        Ok((input, MIDIMessage::Continue))
    } else if status == STOP_MASK {
        Ok((input, MIDIMessage::Stop))
    } else if status == ACTIVE_SENSING_MASK {
        Ok((input, MIDIMessage::ActiveSensing))
    } else if status == RESET_MASK {
        Ok((input, MIDIMessage::Reset))
    } else if status == TUNE_REQUEST_MASK {
        Ok((input, MIDIMessage::TuneRequest))
    } else {
        Ok((input, MIDIMessage::Other { status }))
    }?;

    Ok((input, message))
}

fn parse_channel(status: u8) -> u8 {
    status & 0b0000_1111
}

/// Input is a 14-bit number
/// 0b0lllllll - 1st 7 bits are the least significant bits
/// 0b0mmmmmmm - 2nd 7 bits are the most significant bits
pub(crate) fn parse_14bit_midi_number(input: Input) -> MIDIParseResult<u16> {
    let (input, value1) = be_u8(input)?;
    let (input, value2) = be_u8(input)?;
    let value1 = (value1 & !0b1000_0000) as u16;
    let value2 = ((value2 & !0b1000_0000) as u16) << 7;
    let value = value1 + value2;
    Ok((input, value))
}

pub fn parse_meta_event<'a, Buffer: Borrow<[u8]> + From<Input<'a>>>(
    input: Input<'a>,
) -> MIDIParseResult<'a, MIDIMetaEvent<Buffer>> {
    let (input, _) = tag([0xFF])(input)?;
    let (input, meta_type) = be_u8(input)?;
    let (input, length) = parse_variable_length_num(input)?;
    let (input, bytes) = take(length)(input)?;

    Ok((
        input,
        MIDIMetaEvent {
            meta_type,
            length,
            bytes: bytes.into(),
        },
    ))
}

pub fn parse_track_event<'a, Buffer: Borrow<[u8]> + From<Input<'a>>>(
    input: Input<'a>,
    state: &mut ParserState,
) -> MIDIParseResult<'a, MIDITrackEvent<Buffer>> {
    let (input, delta_time) = parse_variable_length_num(input)?;
    let (input, event) = alt((
        |input| parse_meta_event(input).map(|(input, event)| (input, MIDITrackInner::Meta(event))),
        |input| {
            parse_midi_event(input, state)
                .map(|(input, event)| (input, MIDITrackInner::Message(event)))
        },
    ))(input)?;

    match event {
        MIDITrackInner::Meta(_) => {
            state.last_status = None;
        }
        MIDITrackInner::Message(MIDIMessage::SysExMessage(_)) => {
            state.last_status = None;
        }
        _ => {}
    }

    Ok((
        input,
        MIDITrackEvent {
            delta_time,
            inner: event,
        },
    ))
}

#[derive(Default)]
pub struct ParserState {
    last_status: Option<u8>,
}

pub fn parse_chunk<
    'a,
    StringRepr: Borrow<str> + From<&'a str>,
    Buffer: Borrow<[u8]> + From<Input<'a>>,
>(
    input: Input<'a>,
) -> MIDIParseResult<'a, MIDIFileChunk<StringRepr, Buffer>> {
    let (input, chunk_name) = take(4u32)(input)?;
    let chunk_name: &str = std::str::from_utf8(chunk_name)
        .map_err(|err| Err::Failure(Error::from_external_error(input, ErrorKind::Fail, err)))?;

    let (input, chunk_length) = parse_chunk_length(input)?;
    let (input, chunk_body) = take(chunk_length)(input)?;

    let (_, chunk) = match chunk_name {
        "MThd" => {
            // assert_eq!(chunk_length, 6);
            parse_header_body(chunk_body)
                .map(|(rest, header)| (rest, MIDIFileChunk::Header(header)))
        }
        "MTrk" => {
            let mut state = ParserState::default();
            let mut parse = |input| parse_track_event(input, &mut state);
            let mut events = Vec::with_capacity((chunk_length / 3) as usize);
            let mut chunk_body = chunk_body;
            loop {
                let (new_chunk_body, event) = parse(chunk_body)?;
                events.push(event);
                chunk_body = new_chunk_body;

                if chunk_body.is_empty() {
                    break;
                }
            }
            Ok((chunk_body, MIDIFileChunk::Track { events }))
        }
        _ => Ok((
            chunk_body,
            MIDIFileChunk::Unknown {
                name: chunk_name.into(),
                body: chunk_body.into(),
            },
        )),
    }?;

    Ok((input, chunk))
}

fn parse_chunk_length(input: Input) -> MIDIParseResult<u32> {
    u32(nom::number::Endianness::Big)(input)
}

pub fn parse_midi_file<
    'a,
    StringRepr: Borrow<str> + From<&'a str>,
    Buffer: Borrow<[u8]> + From<&'a [u8]>,
>(
    input: Input<'a>,
) -> MIDIParseResult<'a, MIDIFile<StringRepr, Buffer>> {
    let mut chunks = Vec::with_capacity(input.len() / 10);
    let mut input = input;
    loop {
        let (new_input, chunk) = parse_chunk(input)?;
        chunks.push(chunk);
        input = new_input;

        if input.is_empty() {
            break;
        }
    }
    Ok((input, MIDIFile { chunks }))
}

pub fn parse_midi<'a, Buffer: Borrow<[u8]> + From<Input<'a>>>(
    input: Input<'a>,
) -> MIDIParseResult<'a, Vec<MIDIMessage<Buffer>>> {
    many0(|input| parse_midi_event(input, &mut ParserState::default()))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_file_format_single() {
        let format = [0, 0];
        let (_rest, result) = parse_file_format(&format).unwrap();
        assert_eq!(result, MIDIFileFormat::Single);
    }

    #[test]
    fn test_parse_file_format_simultaneous() {
        let format = [0, 1];
        let (_rest, result) = parse_file_format(&format).unwrap();
        assert_eq!(result, MIDIFileFormat::Simultaneous);
    }

    #[test]
    fn test_parse_file_format_sequential() {
        let format = [0, 2];
        let (_rest, result) = parse_file_format(&format).unwrap();
        assert_eq!(result, MIDIFileFormat::Sequential);
    }

    #[test]
    fn test_parse_file_format_unknown() {
        let format = [0, 8];
        let (_rest, result) = parse_file_format(&format).unwrap();
        assert_eq!(result, MIDIFileFormat::Unknown);
    }

    #[test]
    fn test_parse_header_body_tick_based() {
        let input = [
            // Single
            0,
            0,
            // 1 track
            0,
            1,
            // 2 ticks
            0b0_000_0000,
            2,
        ];
        let (_rest, result) = parse_header_body(&input).unwrap();
        assert_eq!(
            result,
            MIDIFileHeader {
                format: MIDIFileFormat::Single,
                num_tracks: 1,
                division: MIDIFileDivision::TicksPerQuarterNote {
                    ticks_per_quarter_note: 2
                }
            }
        );
    }

    #[test]
    fn test_parse_header_body_smpte_time_based() {
        let input = [
            // Single
            0b0_u8,
            0,
            // 3 track
            0,
            3,
            // SMPTE, format is 1 (which isn't valid, but for simplicity)
            // ticks is 2
            0b1_000_0001,
            2,
        ];
        let (_rest, result) = parse_header_body(&input).unwrap();
        assert_eq!(
            result,
            MIDIFileHeader {
                format: MIDIFileFormat::Single,
                num_tracks: 3,
                division: MIDIFileDivision::SMPTE {
                    format: 1,
                    ticks_per_frame: 2
                }
            }
        );
    }

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
        // let pitch_wheel_message = [0xE3, 0x54, 0x39];
        let (_, result) = parse_14bit_midi_number(&[0x54, 0x39]).unwrap();
        assert_eq!(result, 7380);
    }

    #[test]
    fn test_parse_pitch_wheel_event() {
        // Example pitch change on channel 3
        let pitch_wheel_message = [0xE3, 0x54, 0x39];
        let (_, result) =
            parse_midi_event::<Vec<u8>>(&pitch_wheel_message, &mut ParserState::default()).unwrap();
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
        let (_rest, _midi_stream) = assert_no_alloc::assert_no_alloc(|| {
            parse_midi_file::<String, Vec<u8>>(&file_contents).unwrap()
        });
    }

    #[test]
    fn test_parse_midi_file_smoke_test_no_alloc() {
        let input_path = format!("{}/bach_846.mid", env!("CARGO_MANIFEST_DIR"));
        let file_contents = std::fs::read(input_path).unwrap();
        // let file_contents: Vec<u8> = file_contents.into_iter().take(8000).collect();
        let (_rest, _midi_stream) = assert_no_alloc::assert_no_alloc(|| {
            parse_midi_file::<&str, &[u8]>(&file_contents).unwrap()
        });
        // println!("{:?}", midi_stream);
    }

    #[test]
    fn test() {
        let input_path = format!(
            "{}/test-files/c1_4over4_1bar.mid",
            env!("CARGO_MANIFEST_DIR")
        );
        let file_contents = std::fs::read(input_path).unwrap();
        let (_rest, midi_file) = parse_midi_file::<String, Vec<u8>>(&file_contents).unwrap();
        assert_eq!(midi_file.ticks_per_quarter_note(), 96);
        let quarter_length = midi_file.ticks_per_quarter_note() as u32;
        let sixteenth_length = quarter_length / 4;

        let header = midi_file.header().unwrap();
        assert_eq!(header.format, MIDIFileFormat::Single);
        assert_eq!(header.num_tracks, 1);

        let events: Vec<MIDITrackEvent<Vec<u8>>> = midi_file.track_chunks().cloned().collect();
        let note_on_events: Vec<(u32, MIDIMessageNote)> = events
            .iter()
            .filter_map(|event| match event {
                MIDITrackEvent {
                    delta_time,
                    inner: MIDITrackInner::Message(MIDIMessage::NoteOn(note)),
                } => Some((*delta_time, note.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(note_on_events.len(), 4);
        assert_eq!(note_on_events[0].0, 0);
        assert_eq!(note_on_events[1].0, quarter_length - sixteenth_length);
        assert_eq!(note_on_events[2].0, quarter_length - sixteenth_length);
        assert_eq!(note_on_events[3].0, quarter_length - sixteenth_length);
        for (_, evt) in &note_on_events {
            assert_eq!(evt.velocity, 100);
            assert_eq!(evt.note, 36);
        }

        let note_off_events: Vec<(u32, MIDIMessageNote)> = events
            .iter()
            .filter_map(|event| match event {
                MIDITrackEvent {
                    delta_time,
                    inner: MIDITrackInner::Message(MIDIMessage::NoteOff(note)),
                } => Some((*delta_time, note.clone())),
                _ => None,
            })
            .collect();

        assert_eq!(note_off_events.len(), 4);
        assert_eq!(note_off_events[0].0, sixteenth_length);
        assert_eq!(note_off_events[1].0, sixteenth_length);
        assert_eq!(note_off_events[2].0, sixteenth_length);
        assert_eq!(note_off_events[3].0, sixteenth_length);
    }
}
