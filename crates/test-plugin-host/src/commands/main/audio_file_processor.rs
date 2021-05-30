use crate::commands::main::audio_settings::AudioSettings;
use crate::commands::main::sample_rate_conversion::convert_sample_rate;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use symphonia::core::audio::{AudioBuffer, AudioBufferRef, Signal};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};

/// Opens an audio file with default options & trying to guess the format
pub fn default_read_audio_file(input_audio_path: &str) -> ProbeResult {
    let mut hint = Hint::new();
    let media_source = {
        let audio_input_path = Path::new(input_audio_path);
        if let Some(extension) = audio_input_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }
        }
        Box::new(File::open(audio_input_path).unwrap())
    };
    let audio_file = MediaSourceStream::new(media_source, Default::default());
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let audio_file = match symphonia::default::get_probe().format(
        &hint,
        audio_file,
        &format_opts,
        &metadata_opts,
    ) {
        Ok(probed) => probed,
        Err(err) => {
            log::error!("ERROR: Input file not supported: {}", err);
            exit(1);
        }
    };
    audio_file
}

fn concat_buffers(buffers: Vec<AudioBuffer<f32>>) -> AudioBuffer<f32> {
    let duration = buffers
        .iter()
        .map(|buffer| buffer.chan(0).len() as u64)
        .sum();

    let mut output: AudioBuffer<f32> = AudioBuffer::new(duration, *buffers[0].spec());
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

pub fn read_file_contents(audio_file: &mut ProbeResult) -> AudioBuffer<f32> {
    let audio_file_stream = audio_file
        .format
        .default_stream()
        .expect("Failed to open audio file stream");
    let mut decoder = symphonia::default::get_codecs()
        .make(&audio_file_stream.codec_params, &Default::default())
        .expect("Failed to get input file codec");
    let audio_file_stream_id = audio_file_stream.id;

    let mut audio_buffer: Vec<AudioBuffer<f32>> = Vec::new();
    loop {
        let packet = audio_file.format.next_packet().ok();
        if packet.is_none() {
            break;
        }
        let packet = packet.unwrap();

        if packet.stream_id() != audio_file_stream_id {
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

    concat_buffers(audio_buffer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reading_entire_file() {
        let mut probe = default_read_audio_file("../confirmation.mp3");
        let _ = read_file_contents(&mut probe);
    }
}

pub struct AudioFileSettings {
    audio_file: ProbeResult,
}

impl AudioFileSettings {
    pub fn new(audio_file: ProbeResult) -> Self {
        AudioFileSettings { audio_file }
    }
}

pub struct AudioFileProcessor {
    audio_file_settings: AudioFileSettings,
    audio_settings: AudioSettings,
    audio_file_cursor: usize,
    buffer: Vec<Vec<f32>>,
}

impl AudioFileProcessor {
    pub fn new(audio_file_settings: AudioFileSettings, audio_settings: AudioSettings) -> Self {
        AudioFileProcessor {
            audio_file_settings,
            audio_settings,
            audio_file_cursor: 0,
            buffer: Vec::new(),
        }
    }

    pub fn prepare(&mut self, audio_settings: AudioSettings) {
        self.audio_settings = audio_settings;

        self.buffer.clear();
        self.buffer.reserve(self.audio_settings.channels());
        let audio_file_contents = read_file_contents(&mut self.audio_file_settings.audio_file);
        for channel_number in 0..audio_file_contents.spec().channels.count() {
            let audio_file_channel = audio_file_contents.chan(channel_number);
            let input_rate = audio_file_contents.spec().rate as f32;
            let output_rate = self.audio_settings.sample_rate();
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
            convert_sample_rate(
                input_rate,
                audio_file_channel,
                output_rate,
                channel.as_mut_slice(),
            );

            self.buffer.push(channel);
        }
    }

    pub fn process(&mut self, data: &mut [f32]) {
        let num_channels = self.audio_settings.channels();

        for frame in data.chunks_mut(num_channels) {
            for (channel, sample) in frame.iter_mut().enumerate() {
                let audio_input = self.buffer[channel][self.audio_file_cursor];
                let value = audio_input;
                *sample = value;
            }

            self.audio_file_cursor += 1;
            if self.audio_file_cursor >= self.buffer[0].len() {
                self.audio_file_cursor = 0;
            }
        }
    }
}
