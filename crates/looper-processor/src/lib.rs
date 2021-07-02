use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use basedrop::{Handle, Shared};

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use circular_data_structures::CircularVec;

struct CircularAudioBuffer {
    channels: Vec<CircularVec<f32>>,
}

impl CircularAudioBuffer {
    pub fn new() -> Self {
        CircularAudioBuffer {
            channels: Vec::new(),
        }
    }

    pub fn resize(&mut self, num_channels: usize, sample_rate: f32, duration: Duration) {
        let duration_samples = (duration.as_secs_f32() * sample_rate) as usize;
        self.channels.clear();
        for _channel in 0..num_channels {
            let channel_buffer = CircularVec::with_size(duration_samples, 0.0);
            self.channels.push(channel_buffer);
        }
    }
}

/// Public API types, which should be thread-safe
pub struct LooperProcessorHandle {
    is_recording: AtomicBool,
    is_playing_back: AtomicBool,
    playback_input: AtomicBool,
}

impl LooperProcessorHandle {
    pub fn new() -> Self {
        LooperProcessorHandle {
            is_recording: AtomicBool::new(false),
            is_playing_back: AtomicBool::new(false),
            playback_input: AtomicBool::new(true),
        }
    }

    pub fn set_playback_input(&self, value: bool) {
        self.playback_input.store(value, Ordering::Relaxed);
    }

    pub fn start_recording(&self) {
        self.is_recording.store(true, Ordering::Relaxed);
    }

    pub fn stop_recording(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.is_playing_back.store(false, Ordering::Relaxed);
    }
}

/// Private not thread safe mutable state
struct LooperProcessorState {
    always_recording_cursor: usize,
    num_channels: usize,
    looper_cursor: usize,
    loop_size: usize,
    looped_clip: CircularAudioBuffer,
    always_recording_buffer: CircularAudioBuffer,
}

impl LooperProcessorState {
    pub fn new() -> Self {
        LooperProcessorState {
            num_channels: 2,
            always_recording_cursor: 0,
            looper_cursor: 0,
            loop_size: 0,
            looped_clip: CircularAudioBuffer::new(),
            always_recording_buffer: CircularAudioBuffer::new(),
        }
    }
}

/// A single stereo looper
pub struct LooperProcessor {
    state: LooperProcessorState,
    handle: Shared<LooperProcessorHandle>,
}

impl LooperProcessor {
    pub fn new(handle: &Handle) -> Self {
        LooperProcessor {
            state: LooperProcessorState::new(),
            handle: Shared::new(handle, LooperProcessorHandle::new()),
        }
    }

    pub fn handle(&self) -> Shared<LooperProcessorHandle> {
        self.handle.clone()
    }
}

impl AudioProcessor for LooperProcessor {
    fn prepare(&mut self, settings: AudioProcessorSettings) {
        if settings.output_channels() != settings.input_channels() {
            log::error!("Prepare failed. Output/input channels mismatch");
            return;
        }

        let num_channels = settings.input_channels();
        self.state.num_channels = num_channels;
        self.state.looped_clip.resize(
            num_channels,
            settings.sample_rate(),
            Duration::from_secs(300),
        );
        self.state.always_recording_buffer.resize(
            num_channels,
            settings.sample_rate(),
            Duration::from_secs(3),
        );
    }

    fn process(&mut self, data: &mut [f32]) {
        for (_sample_index, frame) in data.chunks_mut(self.state.num_channels).enumerate() {
            let should_playback_input = self.handle.playback_input.load(Ordering::Relaxed);
            let is_playing = self.handle.is_playing_back.load(Ordering::Relaxed);
            let is_recording = self.handle.is_recording.load(Ordering::Relaxed);

            for (channel_num, channel_sample) in frame.iter_mut().enumerate() {
                let always_recording_channel =
                    &mut self.state.always_recording_buffer.channels[channel_num];
                let loop_channel = &mut self.state.looped_clip.channels[channel_num];

                // PLAYBACK SECTION:
                let input = *channel_sample;
                let current_looper_cursor = self.state.looper_cursor;

                if !should_playback_input {
                    *channel_sample = 0.0;
                }
                if is_playing {
                    *channel_sample = loop_channel[current_looper_cursor % self.state.loop_size];
                }

                // RECORDING SECTION:
                if !is_recording {
                    // When not recording we'll store samples in a buffer. These samples will be
                    // used to fix timing mistakes from the user
                    let cursor = self.state.always_recording_cursor;
                    always_recording_channel[cursor] = input;
                } else {
                    // When recording starts we'll store samples in the looper buffer
                    loop_channel[current_looper_cursor] = input;
                }
            }

            self.state.looper_cursor += 1;
            self.state.always_recording_cursor += 1;

            // If this is the first loop, measure loop size
            if is_recording && !is_playing {
                self.state.loop_size += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::{rms_level, sine_buffer, test_level_equivalence};
    use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};

    use crate::LooperProcessor;

    fn test_settings() -> AudioProcessorSettings {
        AudioProcessorSettings::new(44100.0, 1, 1, 512)
    }

    #[test]
    fn test_looper_produces_silence_when_started() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings.clone());

        let mut silence_buffer = Vec::new();
        // Produce 0.1 second empty buffer
        silence_buffer.resize((0.1 * settings.sample_rate()) as usize, 0.0);

        let mut audio_buffer = silence_buffer.clone();
        looper.process(&mut audio_buffer);
        assert_eq!(rms_level(&audio_buffer), 0.0);
    }

    #[test]
    fn test_looper_plays_its_input_back() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings.clone());

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        let mut audio_buffer = sine_buffer.clone();
        looper.process(&mut audio_buffer);

        test_level_equivalence(&sine_buffer, &audio_buffer, 1, 1, 0.001);
    }

    #[test]
    fn test_looper_does_not_play_back_input_if_specified() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings.clone());

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        assert_ne!(sine_buffer.len(), 0);
        println!("Sine samples: {:?}", sine_buffer);
        println!("Sine RMS: {}", rms_level(&sine_buffer));
        assert_ne!(rms_level(&sine_buffer), 0.0);

        let mut audio_buffer = sine_buffer.clone();
        looper.handle().set_playback_input(false);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(&audio_buffer), 0.0);
    }
}
