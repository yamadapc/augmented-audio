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
use std::{fs, io};

use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};

pub struct OutputFileSettings {
    audio_file_path: String,
}

pub struct OutputAudioFileProcessor {
    audio_settings: AudioProcessorSettings,
    output_file_settings: OutputFileSettings,
    writer: Option<hound::WavWriter<io::BufWriter<fs::File>>>,
}

impl OutputAudioFileProcessor {
    pub fn from_path(audio_settings: AudioProcessorSettings, audio_file_path: &str) -> Self {
        let output_file_settings = OutputFileSettings {
            audio_file_path: audio_file_path.to_string(),
        };
        Self::new(audio_settings, output_file_settings)
    }

    pub fn new(
        audio_settings: AudioProcessorSettings,
        output_file_settings: OutputFileSettings,
    ) -> Self {
        OutputAudioFileProcessor {
            audio_settings,
            output_file_settings,
            writer: None,
        }
    }
}

impl OutputAudioFileProcessor {
    pub fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.audio_settings = settings;
        let sample_rate = settings.sample_rate() as u32;
        log::info!("Wav file will be written with sample rate: {}", sample_rate);
        let spec = hound::WavSpec {
            channels: settings.output_channels() as u16,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        self.writer = Some(
            hound::WavWriter::create(&self.output_file_settings.audio_file_path, spec).unwrap(),
        );
    }

    pub fn process(&mut self, data: &mut AudioBuffer<f32>) -> hound::Result<()> {
        if let Some(writer) = self.writer.as_mut() {
            for sample_num in 0..data.num_samples() {
                for channel_num in 0..data.num_channels() {
                    let sample = *data.get(channel_num, sample_num);
                    writer.write_sample(sample)?;
                }
            }
        }

        Ok(())
    }
}
