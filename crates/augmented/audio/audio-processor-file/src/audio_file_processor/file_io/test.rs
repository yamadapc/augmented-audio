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
use tempdir::TempDir;

use super::*;

fn create_test_file(tempdir: &TempDir) -> PathBuf {
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
    let num_samples = 44100 * 5;
    for sample in 0..num_samples {
        let time = sample as f32 / 44100.0;
        let value = (time * 440.0 * 2.0 * std::f32::consts::PI).sin();
        writer.write_sample(value).unwrap();
        writer.write_sample(value).unwrap();
    }
    writer.finalize().unwrap();
    file_path
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
