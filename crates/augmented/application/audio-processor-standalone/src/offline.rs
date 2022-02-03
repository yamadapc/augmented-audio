use audio_garbage_collector::Handle;
use audio_processor_traits::{
    audio_buffer::VecAudioBuffer, AudioBuffer, AudioProcessor, AudioProcessorSettings,
    MidiEventHandler, MidiMessageLike,
};
use augmented_midi::{
    MIDIFile, MIDIFileChunk, MIDIMessage, MIDIMessageNote, MIDITrackEvent, MIDITrackInner,
};
use itertools::Itertools;

use crate::StandaloneProcessor;

/// Offline rendering options
pub struct OfflineRenderOptions<'a, Processor: StandaloneProcessor> {
    /// The audio/MIDI processor
    pub app: Processor,
    /// GC handle, see <https://crates.io/crates/audio-garbage-collector>
    pub handle: Option<&'a Handle>,
    /// Input audio file path
    pub input_path: &'a str,
    /// Output audio file path
    pub output_path: &'a str,
    /// MIDI input file path
    pub midi_input_file: Option<MIDIFile<String, Vec<u8>>>,
}

/// Render a processor offline into a file.
pub fn run_offline_render<Processor>(options: OfflineRenderOptions<Processor>)
where
    Processor: StandaloneProcessor,
{
    let OfflineRenderOptions {
        mut app,
        handle,
        input_path,
        output_path,
        midi_input_file,
    } = options;

    let _ = wisual_logger::try_init_from_env();

    let handle = handle.unwrap_or_else(|| audio_garbage_collector::handle());

    log::info!(
        "Rendering offline input={} output={}",
        input_path,
        output_path
    );

    let buffer_size = 16;
    let sample_rate = 44100.0;
    let audio_processor_settings = AudioProcessorSettings::new(sample_rate, 2, 2, buffer_size);

    let audio_file_settings = audio_processor_file::AudioFileSettings::from_path(input_path)
        .expect("Failed to read input file");

    // Set-up input file
    log::info!("Loading input file");
    let mut audio_file_processor = audio_processor_file::AudioFileProcessor::new(
        handle,
        audio_file_settings,
        audio_processor_settings,
    );
    audio_file_processor.prepare(audio_processor_settings);
    let audio_file_buffer = audio_file_processor.buffer();
    let audio_file_total_samples = audio_file_buffer[0].len();

    // Set-up output file
    log::info!("Setting-up output buffers");
    let mut output_file_processor = audio_processor_file::OutputAudioFileProcessor::from_path(
        audio_processor_settings,
        output_path,
    );
    output_file_processor.prepare(audio_processor_settings);

    // Set-up output buffer
    let block_size = audio_processor_settings.block_size() as usize;
    let total_blocks = audio_file_total_samples / block_size;
    let mut buffer = Vec::new();
    buffer.resize(block_size * audio_processor_settings.input_channels(), 0.0);
    let mut buffer = VecAudioBuffer::new_with(buffer, 2, buffer_size);

    log::info!("Setting-up audio processor");
    app.processor().prepare(audio_processor_settings);

    let midi_input_blocks = midi_input_file.map(|midi_input_file| {
        build_midi_input_blocks(&audio_processor_settings, total_blocks, midi_input_file)
    });

    log::info!("Rendering. total_blocks={}", total_blocks);
    for block_num in 0..total_blocks {
        for sample in buffer.slice_mut() {
            *sample = 0.0;
        }

        audio_file_processor.process(&mut buffer);

        if let Some(midi) = app.midi() {
            if let Some(midi_input_blocks) = &midi_input_blocks {
                let midi_block = &midi_input_blocks[block_num];
                if !midi_block.is_empty() {
                    log::debug!("Forwarding events {:?}", midi_block);
                    midi.process_midi_events(midi_block);
                }
            }
        }

        app.processor().process(&mut buffer);
        output_file_processor.process(buffer.slice_mut());
    }
}

#[derive(Debug)]
struct MIDIBytes {
    bytes: Vec<u8>,
}

impl MidiMessageLike for MIDIBytes {
    fn is_midi(&self) -> bool {
        true
    }

    fn bytes(&self) -> Option<&[u8]> {
        Some(&self.bytes)
    }
}

/// Converts a MIDI stream's delta_time into absolute ticks.
fn convert_to_absolute_time(
    mut events: Vec<MIDITrackEvent<Vec<u8>>>,
) -> Vec<MIDITrackEvent<Vec<u8>>> {
    let mut current_time = 0;
    for event in &mut events {
        current_time += event.delta_time;
        event.delta_time = current_time;
    }
    events
}

/// Builds chunks containing MIDI messages over each block, aligned with their
/// timing and a 120bpm tempo.
fn build_midi_input_blocks(
    settings: &AudioProcessorSettings,
    total_blocks: usize,
    midi_input_file: MIDIFile<String, Vec<u8>>,
) -> Vec<Vec<MIDIBytes>> {
    let tempo = 120_f32;
    let ticks_per_quarter_note = midi_input_file.ticks_per_quarter_note() as f32;
    let chunks = midi_input_file.chunks;
    let track_events: Vec<MIDITrackEvent<Vec<u8>>> = chunks
        .into_iter()
        .filter_map(|chunk| match chunk {
            MIDIFileChunk::Track { events } => {
                let events = convert_to_absolute_time(events);
                Some(events)
            }
            _ => None,
        })
        .flatten()
        .sorted_by_key(|event| event.delta_time)
        .collect();
    let mut track_events_position = 0;
    let mut result = Vec::with_capacity(total_blocks);
    let block_size = settings.block_size as f32;
    let inverse_sample_rate = 1.0 / settings.sample_rate;

    for i in 0..total_blocks {
        let delta_time_ticks = get_delta_time_ticks(
            tempo,
            ticks_per_quarter_note,
            block_size,
            inverse_sample_rate,
            i,
        );

        log::debug!(
            "Block - {} - ticks_per_beat={} - ticks={} input_len={} dt={}",
            i,
            ticks_per_quarter_note,
            delta_time_ticks,
            track_events.len(),
            track_events[track_events_position].delta_time
        );

        let midi_track_events: Vec<&MIDITrackEvent<Vec<u8>>> = track_events
            .iter()
            .skip(track_events_position)
            .filter(|event| event.delta_time <= delta_time_ticks as u32)
            .collect();

        let midi_block: Vec<MIDIBytes> = midi_track_events
            .iter()
            .filter_map(|event| {
                log::debug!("Filtering MIDI event {:?}", event);
                if let MIDITrackInner::Message(inner) = &event.inner {
                    Some(inner)
                } else {
                    None
                }
            })
            .filter_map(|event| match event {
                MIDIMessage::NoteOn(MIDIMessageNote { velocity, note, .. }) => Some(MIDIBytes {
                    bytes: vec![0x90, *note, *velocity],
                }),
                MIDIMessage::NoteOff(MIDIMessageNote { velocity, note, .. }) => Some(MIDIBytes {
                    bytes: vec![0x80, *note, *velocity],
                }),
                _ => None,
            })
            .collect();

        track_events_position += midi_track_events.len();
        result.push(midi_block);
    }

    result
}

/// Returns the number of elapsed MIDI ticks based on the current block index
fn get_delta_time_ticks(
    tempo: f32,
    ticks_per_quarter_note: f32,
    block_size: f32,
    inverse_sample_rate: f32,
    i: usize,
) -> f32 {
    let time_per_block = block_size * inverse_sample_rate;
    let delta_time_secs = (i as f32) * time_per_block;
    let beats_per_second = tempo / 60.0;
    let delta_time_beats = delta_time_secs * beats_per_second;

    ticks_per_quarter_note * delta_time_beats
}

#[cfg(test)]
mod test {
    use crate::StandaloneAudioOnlyProcessor;
    use audio_processor_testing_helpers::relative_path;
    use audio_processor_traits::{AudioProcessorSettings, NoopAudioProcessor};
    use augmented_midi::{
        MIDIFile, MIDIFileChunk, MIDIFileDivision, MIDIFileFormat, MIDIFileHeader, MIDITrackEvent,
        MIDITrackInner,
    };

    use super::*;

    #[test]
    fn test_run_offline_render() {
        let _ = wisual_logger::try_init_from_env();
        let input_path = relative_path!("../../../../input-files/1sec-sine.mp3");
        let output_path = relative_path!("./test-output/offline-render-test-output.wav");
        let options = OfflineRenderOptions {
            app: StandaloneAudioOnlyProcessor::new(NoopAudioProcessor::new()),
            handle: Some(audio_garbage_collector::handle()),
            input_path: &input_path,
            output_path: &output_path,
            midi_input_file: None,
        };
        run_offline_render(options);
    }

    #[test]
    fn test_build_midi_input_blocks_with_no_blocks() {
        let chunks = vec![
            MIDIFileChunk::Header(MIDIFileHeader {
                format: MIDIFileFormat::Single,
                num_tracks: 1,
                division: MIDIFileDivision::TicksPerQuarterNote {
                    ticks_per_quarter_note: 10,
                },
            }),
            MIDIFileChunk::Track {
                events: vec![
                    MIDITrackEvent {
                        delta_time: 0,
                        inner: MIDITrackInner::Message(MIDIMessage::NoteOn(MIDIMessageNote {
                            channel: 1,
                            note: 120,
                            velocity: 120,
                        })),
                    },
                    MIDITrackEvent {
                        delta_time: 40,
                        inner: MIDITrackInner::Message(MIDIMessage::NoteOn(MIDIMessageNote {
                            channel: 1,
                            note: 120,
                            velocity: 120,
                        })),
                    },
                ],
            },
        ];

        let midi_file = MIDIFile::new(chunks);
        let settings = AudioProcessorSettings::default();
        let result = build_midi_input_blocks(&settings, 0, midi_file);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_build_midi_input_blocks() {
        let chunks = vec![
            MIDIFileChunk::Header(MIDIFileHeader {
                format: MIDIFileFormat::Single,
                num_tracks: 1,
                division: MIDIFileDivision::TicksPerQuarterNote {
                    ticks_per_quarter_note: 1,
                },
            }),
            MIDIFileChunk::Track {
                events: vec![
                    MIDITrackEvent {
                        // 0
                        delta_time: 0,
                        inner: MIDITrackInner::Message(MIDIMessage::NoteOn(MIDIMessageNote {
                            channel: 1,
                            note: 120,
                            velocity: 120,
                        })),
                    },
                    MIDITrackEvent {
                        // 2 quarter note offset (1 secs in)
                        delta_time: 2,
                        inner: MIDITrackInner::Message(MIDIMessage::NoteOff(MIDIMessageNote {
                            channel: 1,
                            note: 120,
                            velocity: 120,
                        })),
                    },
                ],
            },
        ];

        let midi_file = MIDIFile::new(chunks);
        // 1000.0 samples a sec
        // 50.0 50ms per block
        let settings = AudioProcessorSettings::new(1000.0, 1, 1, 50);
        let result = build_midi_input_blocks(&settings, 21, midi_file);
        assert_eq!(result.len(), 21);
        assert_eq!(
            result[0].len(),
            1,
            "Expected 1st block to have note-on event"
        );
        for i in 1..19 {
            assert!(result[i].is_empty());
        }
        assert_eq!(result[20].len(), 1);
    }

    #[test]
    fn test_get_delta_time_ticks() {
        let delta_time_ticks = get_delta_time_ticks(
            // 120bpm (8ms per beat)
            120.0,
            // 1/10 beats per tick
            10.0,
            // 50 samples per block (50ms per block)
            50.0,
            // 1000Hz - 1000 samples a second, 1ms per sample
            1.0 / 1000.0,
            0,
        );
        // First index.
        // 0-50ms -> ~5 beats
        assert!((delta_time_ticks - 0.0).abs() < 0.05);

        let delta_time_ticks = get_delta_time_ticks(
            // 120bpm - 500ms per beat
            120.0,
            // 1/10 beats per tick
            10.0,
            // 50 samples per block (50ms per block)
            50.0,
            // 1000Hz - 1000 samples a second, 1ms per sample
            1.0 / 1000.0,
            1,
        );
        // 100-150ms -> ~18 beats
        // println!("delta_time_ticks={}", delta_time_ticks);
        assert!((delta_time_ticks - 1.0).abs() < 0.05);
    }
}
