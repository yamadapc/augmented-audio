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
use std::fs::File;
use std::path::Path;

use symphonia::core::audio::{AudioBuffer as SymphoniaAudioBuffer, AudioBufferRef};
use symphonia::core::audio::{AudioBuffer, Signal};
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
    #[error("File has no buffers")]
    EmptyFileError,
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

    let mut channel_buffers: Vec<SymphoniaAudioBuffer<f32>> = Vec::new();
    metrics::time("AudioFileProcessor - Reading file packages", || loop {
        match audio_file.format.next_packet().ok() {
            None => break,
            Some(packet) => {
                if packet.track_id() != audio_file_stream_id {
                    break;
                }

                let decoded = decoder.decode(&packet).ok();
                match decoded {
                    Some(audio_buffer) => {
                        let destination = convert_audio_buffer_sample_type(audio_buffer);
                        channel_buffers.push(destination);
                    }
                    _ => break,
                }
            }
        }
    });

    if channel_buffers.is_empty() {
        return Err(AudioFileError::EmptyFileError);
    }

    Ok(metrics::time(
        "AudioFileProcessor - Concatenating packets",
        || concat_buffers(channel_buffers),
    ))
}

fn convert_audio_buffer_sample_type(audio_buffer: AudioBufferRef) -> AudioBuffer<f32> {
    let mut destination =
        SymphoniaAudioBuffer::new(audio_buffer.capacity() as u64, *audio_buffer.spec());
    let _ = destination.fill(|_, _| Ok(()));
    destination.truncate(audio_buffer.frames());
    assert_eq!(audio_buffer.frames(), destination.frames());
    assert_eq!(audio_buffer.capacity(), destination.capacity());

    match audio_buffer {
        AudioBufferRef::U8(inner) => inner.convert(&mut destination),
        AudioBufferRef::U16(inner) => inner.convert(&mut destination),
        AudioBufferRef::U24(inner) => inner.convert(&mut destination),
        AudioBufferRef::U32(inner) => inner.convert(&mut destination),
        AudioBufferRef::S8(inner) => inner.convert(&mut destination),
        AudioBufferRef::S16(inner) => inner.convert(&mut destination),
        AudioBufferRef::S24(inner) => inner.convert(&mut destination),
        AudioBufferRef::S32(inner) => inner.convert(&mut destination),
        AudioBufferRef::F32(inner) => inner.convert(&mut destination),
        AudioBufferRef::F64(inner) => inner.convert(&mut destination),
    }
    destination
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

/// buffers must be non-empty
fn concat_buffers(buffers: Vec<SymphoniaAudioBuffer<f32>>) -> SymphoniaAudioBuffer<f32> {
    let duration = buffers
        .iter()
        .map(|buffer| buffer.chan(0).len() as u64)
        .sum();

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
