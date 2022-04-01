use std::borrow::Borrow;
use std::io::Write;

use cookie_factory::bytes::be_u8;
use cookie_factory::multi::all;
use cookie_factory::{gen, GenError};

use crate::{
    MIDIMessage, MIDIMessageNote, ACTIVE_SENSING_MASK, CHANNEL_PRESSURE_MASK, CONTINUE_MASK,
    CONTROL_CHANGE_MASK, NOTE_OFF_MASK, NOTE_ON_MASK, POLYPHONIC_KEY_PRESSURE_MASK, RESET_MASK,
    SONG_SELECT_MASK, START_MASK, STOP_MASK, SYSEX_MESSAGE_END_MASK, SYSEX_MESSAGE_MASK,
    TIMING_CLOCK_MASK,
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
            let status = POLYPHONIC_KEY_PRESSURE_MASK | channel;
            gen(all([be_u8(status), be_u8(program_number)].iter()), output)?
        }
        MIDIMessage::ChannelPressure { channel, pressure } => {
            let status = CHANNEL_PRESSURE_MASK | channel;
            gen(all([be_u8(status), be_u8(pressure)].iter()), output)?
        }
        MIDIMessage::PitchWheelChange {
            channel: _channel,
            value: _value,
        } => {
            Err(GenError::NotYetImplemented)?
            // let mut output = [0u8; 2];
            // let status = PITCH_WHEEL_CHANGE_MASK | channel;
            // // TODO: This is wrong, this should be the inverse of `parse_14bit_midi_number`
            // // let _ = be_u16(value);
            // gen(all([be_u8(status)].iter()), &mut output[..])?;
            // output.to_vec()
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
            let (output, _pos) = gen(all([be_u8(status), be_u8(end)].iter()), output)?;
            let (output, pos) = gen(all(message_bytes), output)?;
            (output, pos)
        }
        // TODO - need to do 14bit serialization here
        MIDIMessage::SongPositionPointer { .. } => Err(GenError::NotYetImplemented)?,
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
        _ => Err(GenError::NotYetImplemented)?,
    };
    Ok(result)
}

#[cfg(test)]
mod test {

    use crate::{parse_midi_event, MIDIMessage};

    use super::*;

    #[test]
    fn test_serialize_cc() {
        let message = MIDIMessage::ControlChange {
            channel: 1,
            controller_number: 22,
            value: 10,
        };
        let (bytes, _) = serialize_message(message.clone(), vec![]).unwrap();
        let mut state = Default::default();
        let (_, output_message) = parse_midi_event::<Vec<u8>>(&bytes, &mut state).unwrap();
        assert_eq!(message, output_message);
    }
}
