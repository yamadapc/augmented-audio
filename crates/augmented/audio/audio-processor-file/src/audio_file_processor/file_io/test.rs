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

use std::borrow::Cow;
use std::path::PathBuf;

use hound::WavSpec;
use symphonia::core::audio::Channels;
use symphonia::core::units::Duration;
use tempdir::TempDir;

use super::*;

fn create_test_file_with_samples(tempdir: &TempDir, num_samples: i32) -> PathBuf {
    let file_path = tempdir.path().join("test_default_read_audio_file.wav");
    let mut writer = hound::WavWriter::create(
        &file_path,
        WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        },
    )
    .unwrap();
    for sample in 0..num_samples {
        let time = sample as f32 / 44100.0;
        let value = (time * 440.0 * 2.0 * std::f32::consts::PI).sin();
        writer.write_sample(value).unwrap();
        writer.write_sample(value).unwrap();
    }
    writer.finalize().unwrap();
    file_path
}

fn create_test_file(tempdir: &TempDir) -> PathBuf {
    let num_samples = 44100 * 5;
    create_test_file_with_samples(tempdir, num_samples)
}

#[test]
fn test_default_read_audio_file_wav() {
    wisual_logger::init_from_env();
    let tempdir = TempDir::new("test_default_read_audio_file").unwrap();

    let test_file = create_test_file(&tempdir);
    log::info!("Got testing file at {:?}", test_file);

    let probe_result = default_read_audio_file(test_file.to_str().unwrap()).unwrap();
    let default_track = probe_result.format.default_track().unwrap();
    let codec_params = &default_track.codec_params;
    let channels = codec_params.channels.unwrap();
    assert_eq!(channels.count(), 2);
    assert_eq!(codec_params.sample_rate.unwrap(), 44100);
    assert_eq!(codec_params.n_frames.unwrap(), 44100 * 5);
}

#[test]
fn test_convert_audio_buffer_sample_type() {
    let mut symphonia_buffer = symphonia::core::audio::AudioBuffer::new(
        4410,
        symphonia::core::audio::SignalSpec::new(
            44100,
            Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        ),
    );
    symphonia_buffer
        .fill(|_, _| Ok(()))
        .expect("TODO: panic message");

    assert_eq!(symphonia_buffer.chan(0).len(), 4410);
    symphonia_buffer.chan_mut(0).fill(u32::MAX);
    symphonia_buffer.chan_mut(1).fill(u32::MIN);
    let input_buffer: AudioBufferRef = AudioBufferRef::U32(Cow::Owned(symphonia_buffer));
    let output_buffer = convert_audio_buffer_sample_type(input_buffer);

    let left = output_buffer.chan(0);
    let right = output_buffer.chan(1);

    assert_eq!(left.len(), 4410);
    assert_eq!(right.len(), 4410);
    for sample in left {
        assert_eq!(*sample, 1.0);
    }
    for sample in right {
        assert_eq!(*sample, -1.0);
    }
}

#[test]
fn test_new_file_contents_stream() {
    let tempdir = TempDir::new("test_file_contents_stream").unwrap();
    let test_file = create_test_file(&tempdir);
    let mut probe_result = default_read_audio_file(test_file.to_str().unwrap()).unwrap();

    FileContentsStream::new(&mut probe_result).expect("Did not work");
}

#[test]
fn test_file_contents_stream_len() {
    let tempdir = TempDir::new("test_file_contents_stream").unwrap();
    let test_file = create_test_file(&tempdir);
    let mut probe_result = default_read_audio_file(test_file.to_str().unwrap()).unwrap();

    let file_contents_stream =
        FileContentsStream::new(&mut probe_result).expect("Failed to create stream");
    assert_eq!(file_contents_stream.len(), 44100 * 5);
}

#[test]
fn test_file_contents_stream_next() {
    let tempdir = TempDir::new("test_file_contents_stream").unwrap();
    let test_file = create_test_file(&tempdir);
    let mut probe_result = default_read_audio_file(test_file.to_str().unwrap()).unwrap();

    let mut file_contents_stream =
        FileContentsStream::new(&mut probe_result).expect("Failed to create stream");

    let mut all_samples = vec![];
    let mut num_samples = 0;
    while let Some(buffer) = file_contents_stream.next() {
        num_samples += buffer.chan(0).len();
        for sample in buffer.chan(0) {
            all_samples.push(*sample);
        }
    }
    assert_eq!(num_samples, 44100 * 5);

    let rms = (all_samples.iter().map(|x| x * x).sum::<f32>() / all_samples.len() as f32).sqrt();
    assert!(rms.abs() > 0.7);
}

#[test]
fn test_concat_buffers() {
    let mut buffer1 = symphonia::core::audio::AudioBuffer::new(
        2,
        symphonia::core::audio::SignalSpec::new(
            44100,
            Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        ),
    );
    let mut buffer2 = symphonia::core::audio::AudioBuffer::new(
        2,
        symphonia::core::audio::SignalSpec::new(
            44100,
            Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        ),
    );

    buffer1.fill(|_, _| Ok(())).unwrap();
    buffer1.chan_mut(0).fill(1.0);
    buffer1.chan_mut(1).fill(1.0);
    buffer2.fill(|_, _| Ok(())).unwrap();
    buffer2.chan_mut(0).fill(3.0);
    buffer2.chan_mut(1).fill(3.0);

    let result = concat_buffers(vec![buffer1, buffer2]);
    assert_eq!(result.chan(0).len(), 4);
    assert_eq!(result.chan(0)[0], 1.0);
    assert_eq!(result.chan(0)[1], 1.0);
    assert_eq!(result.chan(0)[2], 3.0);
    assert_eq!(result.chan(0)[3], 3.0);
    assert_eq!(result.chan(1)[0], 1.0);
    assert_eq!(result.chan(1)[1], 1.0);
    assert_eq!(result.chan(1)[2], 3.0);
    assert_eq!(result.chan(1)[3], 3.0);
}

#[test]
fn test_convert_audio_file_stream_sample_rate() {
    let tempdir = TempDir::new("test_file_contents_stream").unwrap();
    let test_file = create_test_file(&tempdir);
    let mut probe_result = default_read_audio_file(test_file.to_str().unwrap()).unwrap();
    let file_contents_stream = FileContentsStream::new(&mut probe_result).unwrap();

    let mut converted_stream = convert_audio_file_stream_sample_rate(file_contents_stream, 22050.0);
    let mut all_samples = vec![];
    let mut num_samples = 0;

    while let Some(buffer) = converted_stream.next() {
        num_samples += buffer.channel(0).len();
        for sample in buffer.channel(0) {
            all_samples.push(*sample);
        }
    }

    assert_eq!(num_samples, 22050 * 5);
}

#[test]
fn test_file_frames_stream_if_input_sample_count_is_a_multiple_of_block_size() {
    let mut buffer1 = symphonia::core::audio::AudioBuffer::new(
        (BLOCK_SIZE * 4) as Duration,
        symphonia::core::audio::SignalSpec::new(
            44100,
            Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        ),
    );

    buffer1.fill(|_, _| Ok(())).unwrap();
    buffer1.chan_mut(0).fill(1.0);
    buffer1.chan_mut(1).fill(1.0);

    let buffer_iterator = vec![buffer1].into_iter();
    let file_frames_stream = FileFramesStream::new(buffer_iterator, BLOCK_SIZE);
    let result: Vec<(Vec<Vec<f32>>, usize)> = file_frames_stream.collect();

    assert_eq!(result.len(), 4, "Wrong number of frames");
    assert_eq!(result[0].0.len(), 2, "Wrong number of channels");
    assert_eq!(result[1].0.len(), 2, "Wrong number of channels");

    assert_eq!(result[0].0[0].len(), BLOCK_SIZE, "Wrong number of samples");
    assert_eq!(result[0].0[0][0], 1.0, "Wrong sample value");
}

#[test]
fn test_file_frames_stream_if_input_is_not_a_multiple_of_file_frame_count() {
    let block_size = 4;
    let mut buffer1 = symphonia::core::audio::AudioBuffer::new(
        (block_size * 2 + 3) as Duration,
        symphonia::core::audio::SignalSpec::new(44100, Channels::FRONT_LEFT),
    );

    buffer1.fill(|_, _| Ok(())).unwrap();
    buffer1.chan_mut(0).fill(1.0);

    let buffer_iterator = vec![buffer1].into_iter();
    let mut file_frames_stream = FileFramesStream::new(buffer_iterator, block_size);
    let frame1 = file_frames_stream.next().unwrap();
    assert_eq!(frame1.0.len(), 1);
    assert_eq!(frame1.0[0].len(), block_size);
    assert_eq!(frame1.1, block_size, "Frame 1 has wrong size");
    let frame2 = file_frames_stream.next().unwrap();
    assert_eq!(frame2.0.len(), 1);
    assert_eq!(frame2.0[0].len(), block_size);
    println!("{:?}", frame2.1);
    assert_eq!(frame2.1, block_size, "Frame 2 has wrong size");
    let frame3 = file_frames_stream.next().unwrap();
    assert_eq!(frame3.0.len(), 1);
    assert_eq!(frame3.0[0].len(), block_size);
    assert_eq!(frame3.1, 3, "Frame 3 has wrong size");
}
