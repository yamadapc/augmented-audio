use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use rayon::prelude::*;
use symphonia::core::audio::AudioBuffer as SymphoniaAudioBuffer;
use symphonia::core::probe::ProbeResult;

use audio_garbage_collector::{Handle, Shared};
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};

use crate::processors::audio_file_processor::file_io::AudioFileError;

pub(crate) mod file_io;

/// An audio processor which plays a file in loop
pub struct AudioFileSettings {
    audio_file: ProbeResult,
}

impl AudioFileSettings {
    pub fn new(audio_file: ProbeResult) -> Self {
        AudioFileSettings { audio_file }
    }

    pub fn from_path(path: &str) -> Result<Self, AudioFileError> {
        Ok(Self::new(file_io::default_read_audio_file(path)?))
    }
}

pub struct AudioFileProcessorHandle {
    audio_file_cursor: AtomicUsize,
    is_playing: AtomicBool,
}

impl AudioFileProcessorHandle {
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
}

pub struct AudioFileProcessor {
    audio_file_settings: AudioFileSettings,
    audio_settings: AudioProcessorSettings,
    buffer: Vec<Vec<f32>>,
    handle: Shared<AudioFileProcessorHandle>,
}

impl AudioFileProcessor {
    pub fn from_path(
        handle: &Handle,
        audio_settings: AudioProcessorSettings,
        path: &str,
    ) -> Result<Self, AudioFileError> {
        let audio_file_settings = AudioFileSettings::new(file_io::default_read_audio_file(path)?);
        Ok(Self::new(handle, audio_file_settings, audio_settings))
    }

    pub fn new(
        handle: &Handle,
        audio_file_settings: AudioFileSettings,
        audio_settings: AudioProcessorSettings,
    ) -> Self {
        AudioFileProcessor {
            audio_file_settings,
            audio_settings,
            buffer: Vec::new(),
            handle: Shared::new(
                handle,
                AudioFileProcessorHandle {
                    audio_file_cursor: AtomicUsize::new(0),
                    is_playing: AtomicBool::new(true),
                },
            ),
        }
    }

    /// Unsafe get buffer for offline rendering
    pub fn buffer(&self) -> &Vec<Vec<f32>> {
        &self.buffer
    }

    /// Prepares for playback
    ///
    /// Note: Currently this will load the audio file on the audio-thread.
    /// It'd be an interesting exercise to perform this on a background thread.
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
        let is_playing = self.handle.is_playing.load(Ordering::Relaxed);

        if !is_playing {
            return;
        }

        let start_cursor = self.handle.audio_file_cursor.load(Ordering::Relaxed);
        let mut audio_file_cursor = start_cursor;

        for frame in data.frames_mut() {
            for (channel_index, sample) in frame.iter_mut().enumerate() {
                let audio_input = self.buffer[channel_index][audio_file_cursor];
                let value = audio_input;
                *sample += value;
            }

            audio_file_cursor += 1;
            if audio_file_cursor >= self.buffer[0].len() {
                audio_file_cursor = 0;
            }
        }

        let _ = self.handle.audio_file_cursor.compare_exchange(
            start_cursor,
            audio_file_cursor,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }

    /// Resume playback
    pub fn play(&self) {
        self.handle.play()
    }

    /// Pause playback
    pub fn pause(&self) {
        self.handle.pause()
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        self.handle.stop()
    }

    /// Whether the file is being played back
    pub fn is_playing(&self) -> bool {
        self.handle.is_playing()
    }
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::GarbageCollector;
    use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};

    use super::*;

    fn setup() -> (GarbageCollector, AudioFileSettings) {
        let garbage_collector = audio_garbage_collector::GarbageCollector::default();
        let path = format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../input-files/1sec-sine.mp3"
        );
        let audio_file_settings = AudioFileSettings::from_path(&path).unwrap();
        (garbage_collector, audio_file_settings)
    }

    /**
     * Test a stopped audio processor is silent
     */
    #[test]
    fn test_audio_file_processor_stopped_is_silent() {
        let (garbage_collector, audio_file_settings) = setup();

        let mut audio_file_processor = AudioFileProcessor::new(
            garbage_collector.handle(),
            audio_file_settings,
            Default::default(),
        );
        audio_file_processor.prepare(Default::default());

        audio_file_processor.stop();
        let mut sample_buffer = VecAudioBuffer::new();
        sample_buffer.resize(2, 44100, 0.0);
        audio_file_processor.process(&mut sample_buffer);

        assert!(audio_processor_testing_helpers::rms_level(sample_buffer.slice()) < f32::EPSILON);
    }

    /**
     * Test a running audio processor is not silent
     */
    #[test]
    fn test_audio_file_processor_playing_is_not_silent() {
        let (garbage_collector, audio_file_settings) = setup();

        let mut audio_file_processor = AudioFileProcessor::new(
            garbage_collector.handle(),
            audio_file_settings,
            Default::default(),
        );
        audio_file_processor.prepare(Default::default());

        let mut sample_buffer = VecAudioBuffer::new();
        sample_buffer.resize(2, 44100, 0.0);
        audio_file_processor.play();
        audio_file_processor.process(&mut sample_buffer);

        assert!(audio_processor_testing_helpers::rms_level(sample_buffer.slice()) > f32::EPSILON);
    }
}
