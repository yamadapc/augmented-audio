use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use rayon::prelude::*;
use symphonia::core::audio::AudioBuffer as SymphoniaAudioBuffer;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::ProbeResult;
use symphonia::default::get_probe;
use thiserror::Error;

use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};
use convert_sample_rate::convert_sample_rate;

use crate::processors::audio_file_processor::file_io::AudioFileError;
use crate::timer;

pub(crate) mod file_io;

/// An audio processor which plays a file in loop
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
    audio_settings: AudioProcessorSettings,
    buffer: Vec<Vec<f32>>,
    audio_file_cursor: AtomicUsize,
    is_playing: AtomicBool,
}

impl AudioFileProcessor {
    pub fn from_path(
        audio_settings: AudioProcessorSettings,
        path: &str,
    ) -> Result<Self, AudioFileError> {
        let audio_file_settings = AudioFileSettings::new(file_io::default_read_audio_file(path)?);
        Ok(Self::new(audio_file_settings, audio_settings))
    }

    pub fn new(
        audio_file_settings: AudioFileSettings,
        audio_settings: AudioProcessorSettings,
    ) -> Self {
        AudioFileProcessor {
            audio_file_settings,
            audio_settings,
            buffer: Vec::new(),
            audio_file_cursor: AtomicUsize::new(0),
            is_playing: AtomicBool::new(true),
        }
    }

    /// Resume playback
    pub fn play(&self) {
        self.is_playing.store(true, Ordering::Relaxed);
    }

    /// Pause playback
    pub fn pause(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
        self.audio_file_cursor.store(0, Ordering::Relaxed);
    }

    /// Whether the file is being played back
    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    /// Unsafe get buffer for offline rendering
    pub fn buffer(&self) -> &Vec<Vec<f32>> {
        &self.buffer
    }

    /// Prepares for playback
    pub fn prepare(&mut self, audio_settings: AudioProcessorSettings) {
        log::info!("Preparing for audio file playback");
        self.audio_settings = audio_settings;

        self.buffer.clear();
        self.buffer.reserve(self.audio_settings.input_channels());

        let start = Instant::now();
        log::info!("Reading audio file onto memory");
        let audio_file_contents =
            file_io::read_file_contents(&mut self.audio_file_settings.audio_file);
        match audio_file_contents {
            Ok(audio_file_contents) => {
                log::info!("Read input file duration={}ms", start.elapsed().as_millis());
                self.set_audio_file_contents(audio_file_contents)
            }
            Err(err) => {
                log::error!("Failed to read input file {}", err);
            }
        }
    }

    /// Performs sample-rate conversion of the input file in multiple threads
    fn set_audio_file_contents(&mut self, audio_file_contents: SymphoniaAudioBuffer<f32>) {
        let start = Instant::now();
        log::info!("Performing sample rate conversion");
        let output_rate = self.audio_settings.sample_rate();
        let converted_channels: Vec<Vec<f32>> = (0..audio_file_contents.spec().channels.count())
            .into_par_iter()
            .map(|channel_number| {
                file_io::convert_audio_file_sample_rate(
                    &audio_file_contents,
                    output_rate,
                    channel_number,
                )
            })
            .collect();

        for channel in converted_channels {
            self.buffer.push(channel);
        }

        log::info!(
            "Performed sample rate conversion duration={}ms",
            start.elapsed().as_millis()
        );
    }

    pub fn process<BufferType: AudioBuffer<SampleType = f32>>(&mut self, data: &mut BufferType) {
        let is_playing = self.is_playing.load(Ordering::Relaxed);

        if !is_playing {
            for sample in data.slice_mut() {
                *sample = 0.0;
            }
            return;
        }

        let start_cursor = self.audio_file_cursor.load(Ordering::Relaxed);
        let mut audio_file_cursor = start_cursor;

        for frame in data.frames_mut() {
            for (channel_index, sample) in frame.iter_mut().enumerate() {
                let audio_input = self.buffer[channel_index][audio_file_cursor];
                let value = audio_input;
                *sample = value;
            }

            audio_file_cursor += 1;
            if audio_file_cursor >= self.buffer[0].len() {
                audio_file_cursor = 0;
            }
        }

        let _ = self.audio_file_cursor.compare_exchange(
            start_cursor,
            audio_file_cursor,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
}
