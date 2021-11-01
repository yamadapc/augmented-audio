use midir::{MidiInput, MidiInputConnection};
use rimd::Status;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MidiError {
    #[error("Failed to initialize MIDI input")]
    InitError(#[from] midir::InitError),
    #[error("Failed to connect to MIDI input")]
    ConnectError(#[from] midir::ConnectError<MidiInput>),
}

pub type Result<T> = std::result::Result<T, MidiError>;

pub struct Data;

pub fn start_logging_midi_host() -> Result<Vec<MidiInputConnection<Data>>> {
    fn callback(_timestamp: u64, bytes: &[u8], _data: &mut Data) {
        let message = rimd::MidiMessage::from_bytes(Vec::from(bytes));
        match message.status() {
            Status::NoteOff => {
                log::info!("Note off - {:?}", message.data)
            }
            Status::NoteOn => {
                log::info!("Note on - {:?}", message.data)
            }
            Status::PolyphonicAftertouch => {
                log::info!("Polyphonic aftertouch - {:?}", message.data)
            }
            Status::ControlChange => {
                log::info!("Size: {:?}", bytes);
                log::info!("Control change - {:?}", message.data)
            }
            Status::ProgramChange => {
                log::info!("Program change - {:?}", message.data)
            }
            Status::ChannelAftertouch => {}
            Status::PitchBend => {
                log::info!("Pitch bend - {:?}", message.data)
            }
            Status::SysExStart => {}
            Status::MIDITimeCodeQtrFrame => {}
            Status::SongPositionPointer => {}
            Status::SongSelect => {}
            Status::TuneRequest => {}
            Status::SysExEnd => {}
            Status::TimingClock => {}
            Status::Start => {}
            Status::Continue => {}
            Status::Stop => {}
            Status::ActiveSensing => {}
            Status::SystemReset => {}
        }
    }

    log::info!("Creating MIDI input `plugin-host`");
    let input = midir::MidiInput::new("plugin-host")?;
    let mut connections = Vec::new();
    for port in &input.ports() {
        let input = midir::MidiInput::new("plugin-host")?;
        log::info!("MIDI input port: {:?}", input.port_name(port));
        let connection = input.connect(port, "main-port", callback, Data {})?;
        connections.push(connection);
    }
    Ok(connections)
}
