use std::fs::File;
use std::path::Path;

use symphonia::core::audio::Signal;
use symphonia::core::audio::{AudioBuffer as SymphoniaAudioBuffer, AudioBufferRef};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::default::get_probe;
use thiserror::Error;

use augmented_audio_metrics as metrics;

#[derive(Error, Debug)]
pub enum AudioFileError {
    #[error("Failed to decode input file")]
    DecodeError(#[from] symphonia::core::errors::Error),
    #[error("Failed to read input file")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to open read stream")]
    OpenStreamError,
}

/// Opens an audio file with default options & trying to guess the format
pub fn default_read_audio_file(input_audio_path: &str) -> Result<ProbeResult, AudioFileError> {
    log::info!(
        "Trying to open and probe audio file at {}",
        input_audio_path
    );

    let mut hint = Hint::new();
    let media_source = {
        let audio_input_path = Path::new(input_audio_path);
        let _ = try_set_audio_file_hint(&mut hint, audio_input_path);
        File::open(audio_input_path)?
    };
    let audio_file = MediaSourceStream::new(Box::new(media_source), Default::default());
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let audio_file = get_probe().format(&hint, audio_file, &format_opts, &metadata_opts)?;
    Ok(audio_file)
}

/// Attempt to set a hint on codec based on the input file extension
fn try_set_audio_file_hint(hint: &mut Hint, audio_input_path: &Path) -> Option<()> {
    let extension = audio_input_path.extension()?;
    let extension_str = extension.to_str()?;
    hint.with_extension(extension_str);
    Some(())
}

pub fn read_file_contents(
    audio_file: &mut ProbeResult,
) -> Result<SymphoniaAudioBuffer<f32>, AudioFileError> {
    let audio_file_stream = audio_file
        .format
        .default_track()
        .ok_or(AudioFileError::OpenStreamError)?;
    let mut decoder = symphonia::default::get_codecs()
        .make(&audio_file_stream.codec_params, &Default::default())?;
    let audio_file_stream_id = audio_file_stream.id;

    let mut audio_buffer: Vec<SymphoniaAudioBuffer<f32>> = Vec::new();
    metrics::time("AudioFileProcessor - Reading file packages", || loop {
        match audio_file.format.next_packet().ok() {
            None => break,
            Some(packet) => {
                if packet.track_id() != audio_file_stream_id {
                    break;
                }

                let decoded = decoder.decode(&packet).ok();
                match decoded {
                    Some(AudioBufferRef::F32(packet_buffer)) => {
                        audio_buffer.push(packet_buffer.into_owned());
                    }
                    _ => break,
                }
            }
        }
    });

    Ok(metrics::time(
        "AudioFileProcessor - Concatenating packets",
        || concat_buffers(audio_buffer),
    ))
}

pub fn convert_audio_file_sample_rate(
    audio_file_contents: &SymphoniaAudioBuffer<f32>,
    output_rate: f32,
    channel_number: usize,
) -> Vec<f32> {
    let audio_file_channel = audio_file_contents.chan(channel_number);
    let input_rate = audio_file_contents.spec().rate as f32;
    let audio_file_duration = audio_file_channel.len() as f32 / input_rate;

    let output_size = (audio_file_duration * output_rate).ceil() as usize;
    let mut channel = Vec::new();
    channel.resize(output_size, 0.0);
    let audio_file_channel = audio_file_contents.chan(channel_number);

    // Convert sample rate from audio file to in-memory
    log::info!(
        "Converting sample_rate channel={} input_rate={} output_rate={}",
        channel_number,
        input_rate,
        output_rate
    );
    augmented_convert_sample_rate::convert_sample_rate(
        input_rate,
        audio_file_channel,
        output_rate,
        channel.as_mut_slice(),
    );

    channel
}

fn concat_buffers(buffers: Vec<SymphoniaAudioBuffer<f32>>) -> SymphoniaAudioBuffer<f32> {
    let duration = buffers
        .iter()
        .map(|buffer| buffer.chan(0).len() as u64)
        .sum();

    // TODO - Check there're buffers
    let mut output = SymphoniaAudioBuffer::new(duration, *buffers[0].spec());
    let _ = output.fill(|_, _| Ok(()));
    let mut output_cursor = 0;
    for buffer in buffers {
        let mut channel_size = 0;

        for channel_num in 0..2 {
            let mut cursor = output_cursor; // reading channels copy cursor to reset for each channel

            let output_channel = output.chan_mut(channel_num);
            let channel = buffer.chan(channel_num);
            channel_size = channel.len();

            for sample in channel {
                output_channel[cursor] = *sample;
                cursor += 1;
            }
        }

        output_cursor += channel_size;
    }
    output
}
