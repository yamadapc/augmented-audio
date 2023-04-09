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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use symphonia::core::probe::ProbeResult;

use audio_garbage_collector::{Handle, Shared};
#[cfg(feature = "samplerate")]
use audio_processor_traits::OwnedAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor, AudioProcessorSettings};
use file_io::AudioFileError;

pub mod file_io;

pub struct InMemoryAudioFile {
    audio_file: ProbeResult,
}

impl InMemoryAudioFile {
    pub fn new(audio_file: ProbeResult) -> Self {
        InMemoryAudioFile { audio_file }
    }

    pub fn from_path(path: &str) -> Result<Self, AudioFileError> {
        Ok(Self::new(file_io::default_read_audio_file(path)?))
    }

    /// Eagerly read the file onto memory, do sample rate conversion into the target
    /// AudioProcessorSettings and return a VecAudioBuffer containing the file's contents.
    #[cfg(feature = "samplerate")]
    pub fn read_into_vec_audio_buffer(
        &mut self,
        settings: &AudioProcessorSettings,
    ) -> Result<audio_processor_traits::VecAudioBuffer<f32>, AudioFileError> {
        use rayon::prelude::*;

        let output_rate = settings.sample_rate();
        let contents = file_io::read_file_contents(&mut self.audio_file)?;
        let converted_channels: Vec<Vec<f32>> = (0..contents.spec().channels.count())
            .into_par_iter()
            .map(|channel_number| {
                file_io::convert_audio_file_sample_rate(&contents, output_rate, channel_number)
            })
            .collect();

        let mut output_buffer = audio_processor_traits::VecAudioBuffer::new();
        output_buffer.resize(settings.output_channels(), converted_channels[0].len(), 0.0);
        for (channel_index, channel) in converted_channels.iter().enumerate() {
            for (sample_index, sample) in channel.iter().enumerate() {
                output_buffer.set(channel_index, sample_index, *sample);
            }
        }

        Ok(output_buffer)
    }
}

pub struct AudioFileProcessorHandle {
    audio_file_cursor: AtomicUsize,
    is_playing: AtomicBool,
    should_loop: AtomicBool,
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

    pub fn set_should_loop(&self, should_loop: bool) {
        self.should_loop.store(should_loop, Ordering::Relaxed);
    }

    pub fn should_loop(&self) -> bool {
        self.should_loop.load(Ordering::Relaxed)
    }
}

/// An audio processor which plays a file in loop
pub struct AudioFileProcessor {
    audio_file_settings: InMemoryAudioFile,
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
        let audio_file_settings = InMemoryAudioFile::new(file_io::default_read_audio_file(path)?);
        Ok(Self::new(handle, audio_file_settings, audio_settings))
    }

    pub fn new(
        gc_handle: &Handle,
        audio_file_settings: InMemoryAudioFile,
        audio_settings: AudioProcessorSettings,
    ) -> Self {
        let handle = Shared::new(
            gc_handle,
            AudioFileProcessorHandle {
                audio_file_cursor: AtomicUsize::new(0),
                is_playing: AtomicBool::new(true),
                should_loop: AtomicBool::new(true),
            },
        );

        AudioFileProcessor {
            audio_file_settings,
            audio_settings,
            buffer: Vec::new(),
            handle,
        }
    }

    /// Unsafe get buffer for offline rendering
    pub fn num_samples(&self) -> usize {
        if self.buffer.is_empty() {
            0
        } else {
            self.buffer[0].len()
        }
    }

    /// Unsafe get buffer for offline rendering
    pub fn buffer(&self) -> &Vec<Vec<f32>> {
        &self.buffer
    }

    pub fn handle(&self) -> &Shared<AudioFileProcessorHandle> {
        &self.handle
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

    pub fn process_single(&self) -> impl Iterator<Item = f32> + '_ {
        let handle = &self.handle;
        let audio_file_cursor = handle.audio_file_cursor.load(Ordering::Relaxed);
        let iterator = self
            .buffer
            .iter()
            .map(move |channel| channel[audio_file_cursor]);

        let mut audio_file_cursor: usize = audio_file_cursor;
        audio_file_cursor += 1;
        if audio_file_cursor >= self.buffer[0].len() {
            audio_file_cursor = 0;
        }
        handle
            .audio_file_cursor
            .store(audio_file_cursor, Ordering::Relaxed);

        iterator
    }
}

impl AudioProcessor for AudioFileProcessor {
    type SampleType = f32;

    /// Prepares for playback
    ///
    /// Note: Currently this will load the audio file on the audio-thread.
    /// It'd be an interesting exercise to perform this on a background thread.
    fn prepare(&mut self, context: &mut AudioContext) {
        let audio_settings = context.settings;
        log::info!("Preparing for audio file playback");
        self.audio_settings = audio_settings;

        self.buffer.clear();
        self.buffer.reserve(self.audio_settings.output_channels());

        let start = Instant::now();
        log::info!("Reading audio file onto memory");

        let mut run = || -> Result<(), file_io::AudioFileError> {
            let input_stream =
                file_io::FileContentsStream::new(&mut self.audio_file_settings.audio_file)?;
            let converted_stream = file_io::convert_audio_file_stream_sample_rate(
                input_stream,
                audio_settings.sample_rate(),
            );

            let start = Instant::now();
            let mut samples_read = 0;
            let total_frames = converted_stream.len();

            for (i, buffer) in converted_stream.enumerate() {
                self.buffer.resize(buffer.num_channels(), vec![]);

                for (channel, source) in self.buffer.iter_mut().zip(buffer.channels()) {
                    for sample in source {
                        channel.push(*sample)
                    }
                }

                samples_read += buffer.num_samples();
                if i % 80 == 0 {
                    let rate = samples_read as f32 / start.elapsed().as_secs_f32();
                    let eta = (total_frames - samples_read) as f32 / rate;
                    log::info!(
                        "progress {:.1}% - elapsed: {:.1}s - rate: {:.0} samples/s - eta: {:.1}s",
                        (samples_read as f32 / total_frames as f32) * 100.0,
                        start.elapsed().as_secs_f32(),
                        rate,
                        eta,
                    );
                }
            }

            Ok(())
        };

        match run() {
            Ok(_) => {
                log::info!("Read input file duration={}ms", start.elapsed().as_millis());
            }
            Err(err) => {
                log::error!("Failed to read input file {}", err);
            }
        }
    }

    fn process(&mut self, _context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        let is_playing = self.handle.is_playing.load(Ordering::Relaxed);

        if !is_playing {
            return;
        }

        let should_loop = self.handle.should_loop();
        let start_cursor = self.handle.audio_file_cursor.load(Ordering::Relaxed);
        let mut audio_file_cursor = start_cursor;

        for sample_num in 0..data.num_samples() {
            for channel_index in 0..data.num_channels() {
                let audio_input = self.buffer[channel_index][audio_file_cursor];
                let value = audio_input;
                data.channel_mut(channel_index)[sample_num] += value;
            }

            audio_file_cursor += 1;
            if audio_file_cursor >= self.buffer[0].len() {
                audio_file_cursor = 0;

                if !should_loop {
                    self.handle.stop();
                    break;
                }
            }
        }

        let _ = self.handle.audio_file_cursor.compare_exchange(
            start_cursor,
            audio_file_cursor,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::GarbageCollector;
    use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};

    use super::*;

    fn setup() -> (GarbageCollector, InMemoryAudioFile) {
        wisual_logger::init_from_env();

        let garbage_collector = GarbageCollector::default();
        let path = format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../input-files/1sec-sine.mp3"
        );
        let audio_file_settings = InMemoryAudioFile::from_path(&path).unwrap();
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
        let mut context = AudioContext::default();
        audio_file_processor.prepare(&mut context, Default::default());

        audio_file_processor.stop();
        let mut sample_buffer = VecAudioBuffer::new();
        sample_buffer.resize(2, 44100, 0.0);
        audio_file_processor.process(&mut context, &mut sample_buffer);

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
        let mut context = AudioContext::default();
        audio_file_processor.prepare(&mut context, Default::default());

        let mut sample_buffer = VecAudioBuffer::new();
        sample_buffer.resize(2, 44100, 0.0);
        audio_file_processor.play();
        audio_file_processor.process(&mut context, &mut sample_buffer);

        assert!(audio_processor_testing_helpers::rms_level(sample_buffer.slice()) > f32::EPSILON);
    }
}
