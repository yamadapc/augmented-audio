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
use augmented_midi::parse_midi_event;
use midir::{MidiInput, MidiInputConnection};
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
        let message = parse_midi_event::<&[u8]>(bytes, &mut Default::default());
        log::info!("{:?}", message)
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
