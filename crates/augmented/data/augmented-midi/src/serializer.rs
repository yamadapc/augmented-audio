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
use std::io::Write;

use cookie_factory::bytes::be_u8;
use cookie_factory::multi::all;
use cookie_factory::{gen, GenError};

use crate::{
    MIDIMessage, MIDIMessageNote, ACTIVE_SENSING_MASK, CHANNEL_PRESSURE_MASK, CONTINUE_MASK,
    CONTROL_CHANGE_MASK, NOTE_OFF_MASK, NOTE_ON_MASK, PITCH_WHEEL_CHANGE_MASK,
    POLYPHONIC_KEY_PRESSURE_MASK, PROGRAM_CHANGE_MASK, RESET_MASK, SONG_POSITION_POINTER_MASK,
    SONG_SELECT_MASK, START_MASK, STOP_MASK, SYSEX_MESSAGE_END_MASK, SYSEX_MESSAGE_MASK,
    TIMING_CLOCK_MASK, TUNE_REQUEST_MASK,
};

pub fn serialize_message<W: Write, Buffer: Borrow<[u8]>>(
    message: MIDIMessage<Buffer>,
    output: W,
) -> Result<(W, u64), GenError> {
    let result = match message {
        MIDIMessage::NoteOff(MIDIMessageNote {
            channel,
            note,
            velocity,
        }) => {
            let status = NOTE_OFF_MASK | channel;
            gen(
                all([be_u8(status), be_u8(note), be_u8(velocity)].iter()),
                output,
            )?
        }
        MIDIMessage::NoteOn(MIDIMessageNote {
            channel,
            note,
            velocity,
        }) => {
            let status = NOTE_ON_MASK | channel;
            gen(
                all([be_u8(status), be_u8(note), be_u8(velocity)].iter()),
                output,
            )?
        }
        MIDIMessage::PolyphonicKeyPressure {
            channel,
            note,
            pressure,
        } => {
            let status = POLYPHONIC_KEY_PRESSURE_MASK | channel;
            gen(
                all([be_u8(status), be_u8(note), be_u8(pressure)].iter()),
                output,
            )?
        }
        MIDIMessage::ProgramChange {
            channel,
            program_number,
        } => {
            let status = PROGRAM_CHANGE_MASK | channel;
            gen(all([be_u8(status), be_u8(program_number)].iter()), output)?
        }
        MIDIMessage::ChannelPressure { channel, pressure } => {
            let status = CHANNEL_PRESSURE_MASK | channel;
            gen(all([be_u8(status), be_u8(pressure)].iter()), output)?
        }
        MIDIMessage::PitchWheelChange { channel, value } => {
            let status = PITCH_WHEEL_CHANGE_MASK | channel;
            let (lsb, msb) = serialize_14_bit_midi_number(value);
            gen(all([be_u8(status), be_u8(lsb), be_u8(msb)].iter()), output)?
        }
        MIDIMessage::ControlChange {
            channel,
            controller_number,
            value,
        } => {
            let status = CONTROL_CHANGE_MASK | channel;
            gen(
                all([be_u8(status), be_u8(controller_number), be_u8(value)].iter()),
                output,
            )?
        }
        MIDIMessage::SysExMessage(message) => {
            let status = SYSEX_MESSAGE_MASK;
            let end = SYSEX_MESSAGE_END_MASK;
            let message_bytes = message.message.borrow().iter().cloned().map(|b| be_u8(b));
            let (output, _pos) = gen(be_u8(status), output)?;
            let (output, _pos) = gen(all(message_bytes), output)?;
            let (output, pos) = gen(be_u8(end), output)?;
            (output, pos)
        }
        MIDIMessage::SongPositionPointer { beats } => {
            let status = SONG_POSITION_POINTER_MASK;
            let (lsb, msb) = serialize_14_bit_midi_number(beats);
            gen(all([be_u8(status), be_u8(lsb), be_u8(msb)].iter()), output)?
        }
        MIDIMessage::SongSelect { song } => {
            let status = SONG_SELECT_MASK;
            gen(all([be_u8(status), be_u8(song)].iter()), output)?
        }
        MIDIMessage::TimingClock => {
            let status = TIMING_CLOCK_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::Start => {
            let status = START_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::Continue => {
            let status = CONTINUE_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::Stop => {
            let status = STOP_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::ActiveSensing => {
            let status = ACTIVE_SENSING_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::Reset => {
            let status = RESET_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::TuneRequest => {
            let status = TUNE_REQUEST_MASK;
            gen(all([be_u8(status)].iter()), output)?
        }
        MIDIMessage::Other { status } => gen(be_u8(status), output)?,
    };
    Ok(result)
}

/// Input is a 14-bit number
/// 0b0lllllll - 1st 7 bits are the least significant bits
/// 0b0mmmmmmm - 2nd 7 bits are the most significant bits
///
/// Returns both bytes split
fn serialize_14_bit_midi_number(input: u16) -> (u8, u8) {
    let value1 = input & 0b00_0000_0111_1111;
    let value2 = (input & 0b11_1111_1000_0000) >> 7;
    (value1 as u8, value2 as u8)
}

#[cfg(test)]
mod test {
    use crate::{parse_midi_event, MIDIMessage, MIDISysExEvent};

    use super::*;

    #[test]
    fn test_serialize_14_bit_midi_number() {
        let (_, result) = crate::parse_14bit_midi_number(&[0x54, 0x39]).unwrap();
        assert_eq!(result, 7380);
        let (v1, v2) = serialize_14_bit_midi_number(result);
        assert_eq!(v1, 0x54);
        assert_eq!(v2, 0x39);
    }

    fn test_roundtrip(input: MIDIMessage<Vec<u8>>) {
        let (bytes, _) = serialize_message(input.clone(), vec![]).unwrap();
        let mut state = Default::default();
        let (_, output) = parse_midi_event::<Vec<u8>>(&bytes, &mut state).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_roundtrip_control_change() {
        let message = MIDIMessage::ControlChange {
            channel: 1,
            controller_number: 22,
            value: 10,
        };
        test_roundtrip(message);
    }

    #[test]
    fn test_roundtrip_note_on() {
        let note_on = MIDIMessage::<Vec<u8>>::note_on(8, 14, 20);
        test_roundtrip(note_on);
    }

    #[test]
    fn test_roundtrip_note_off() {
        let note_off = MIDIMessage::<Vec<u8>>::note_off(8, 14, 20);
        test_roundtrip(note_off);
    }

    #[test]
    fn test_roundtrip_polyphonic_key_pressure() {
        let input = MIDIMessage::<Vec<u8>>::PolyphonicKeyPressure {
            channel: 8,
            note: 10,
            pressure: 15,
        };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_program_change() {
        let input = MIDIMessage::<Vec<u8>>::ProgramChange {
            channel: 0,
            program_number: 15,
        };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_channel_pressure() {
        let input = MIDIMessage::<Vec<u8>>::ChannelPressure {
            channel: 10,
            pressure: 5,
        };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_pitch_wheel_change() {
        let input = MIDIMessage::<Vec<u8>>::PitchWheelChange {
            channel: 3,
            value: 55,
        };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_sysex() {
        let input = MIDIMessage::<Vec<u8>>::SysExMessage(MIDISysExEvent {
            message: vec![0, 1, 2, 3, 4],
        });
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_song_position_pointer() {
        let input = MIDIMessage::<Vec<u8>>::SongPositionPointer { beats: 4 };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_song_select() {
        let input = MIDIMessage::<Vec<u8>>::SongSelect { song: 3 };
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_timing_clock() {
        let input = MIDIMessage::<Vec<u8>>::TimingClock;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_start() {
        let input = MIDIMessage::<Vec<u8>>::Start;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_continue() {
        let input = MIDIMessage::<Vec<u8>>::Continue;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_stop() {
        let input = MIDIMessage::<Vec<u8>>::Stop;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_active_sensing() {
        let input = MIDIMessage::<Vec<u8>>::ActiveSensing;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_reset() {
        let input = MIDIMessage::<Vec<u8>>::Reset;
        test_roundtrip(input);
    }

    #[test]
    fn test_roundtrip_tune_request() {
        let input = MIDIMessage::<Vec<u8>>::TuneRequest;
        test_roundtrip(input);
    }
}
