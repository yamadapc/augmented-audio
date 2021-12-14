use std::time::Duration;

use midir::MidiOutput;

use augmented_midi::{MIDIFileChunk, MIDIMessage, MIDIMessageNote, MIDITrackEvent, MIDITrackInner};
use itertools::Itertools;

#[derive(Debug)]
struct MIDIBytes {
    bytes: Vec<u8>,
}

fn main() {
    let _ = wisual_logger::try_init_from_env();

    let args = std::env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    log::info!("Parsing MIDI file input={}", input_file_path);
    let midi_input_file = std::fs::read(input_file_path).unwrap();
    let (_, midi_input_file) =
        augmented_midi::parse_midi_file::<String, Vec<u8>>(&midi_input_file).unwrap();

    let mut connections = vec![];
    let output = MidiOutput::new("augmented-midi").unwrap();
    log::info!("Creating MIDI output");
    for port in &output.ports() {
        let output = MidiOutput::new("augmented-midi").unwrap();
        log::info!("MIDI output: {:?}", output.port_name(port));
        let connection = output.connect(&port, "main-port").unwrap();
        connections.push(connection);
    }

    let ticks_per_beat = midi_input_file.ticks_per_quarter_note();
    let track_events: Vec<MIDITrackEvent<Vec<u8>>> = midi_input_file
        .chunks
        .into_iter()
        .filter_map(|chunk| match chunk {
            MIDIFileChunk::Track { events } => Some(events),
            _ => None,
        })
        .flatten()
        .sorted_by_key(|event| event.delta_time)
        .collect();

    loop {
        log::info!("Starting playback");
        let midi_block: Vec<(u32, MIDIBytes)> = track_events
            .iter()
            .filter_map(|event| match event.inner {
                MIDITrackInner::Message(MIDIMessage::NoteOn(MIDIMessageNote {
                    velocity,
                    note,
                    ..
                })) => Some((
                    event.delta_time,
                    MIDIBytes {
                        bytes: vec![(0x9 << 4) + 1, note, velocity],
                    },
                )),
                MIDITrackInner::Message(MIDIMessage::NoteOff(MIDIMessageNote {
                    velocity,
                    note,
                    ..
                })) => Some((
                    event.delta_time,
                    MIDIBytes {
                        bytes: vec![(0x8 << 4) + 1, note, velocity],
                    },
                )),
                _ => None,
            })
            .collect();

        let beats_per_second = 120.0 / 60.0;
        for (delta_time, message) in midi_block {
            let delta_time_beats = (ticks_per_beat as f32) * (delta_time as f32);
            let delta_time_secs = (1.0 / beats_per_second) * delta_time_beats;

            log::info!(
                "Sleeping for delta_time={} delta_time_secs={}s",
                delta_time,
                delta_time_secs
            );
            std::thread::sleep(Duration::from_secs(delta_time_secs as u64));

            for connection in &mut connections {
                // let message = [0x90, 70, 80];
                log::info!(
                    "Flushing message message={:?} delta={}",
                    message,
                    delta_time_secs
                );
                connection.send(&message.bytes).unwrap();
            }
        }
    }
}
