use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use basedrop::{Handle, Shared};

use crate::LoopState::PlayOrOverdub;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use circular_data_structures::CircularVec;
use num::FromPrimitive;
use rimd::Status;

struct InternalBuffer<SampleType> {
    size: usize,
    channels: Vec<CircularVec<SampleType>>,
}

impl<SampleType: num::Float> InternalBuffer<SampleType> {
    pub fn new() -> Self {
        InternalBuffer {
            channels: Vec::new(),
            size: 0,
        }
    }

    pub fn num_samples(&self) -> usize {
        self.size
    }

    pub fn channel(&mut self, channel_index: usize) -> &mut CircularVec<SampleType> {
        &mut self.channels[channel_index]
    }

    pub fn resize(&mut self, num_channels: usize, sample_rate: f32, duration: Duration) {
        let duration_samples = (duration.as_secs_f32() * sample_rate) as usize;
        self.size = duration_samples;
        self.channels.clear();
        for _channel in 0..num_channels {
            let channel_buffer = CircularVec::with_size(duration_samples, SampleType::zero());
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

impl Default for LooperProcessorHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl LooperProcessorHandle {
    pub fn new() -> Self {
        LooperProcessorHandle {
            is_recording: AtomicBool::new(false),
            is_playing_back: AtomicBool::new(false),
            playback_input: AtomicBool::new(true),
        }
    }

    pub fn store_playback_input(&self, value: bool) {
        self.playback_input.store(value, Ordering::Relaxed);
    }

    pub fn start_recording(&self) {
        self.is_recording.store(true, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.is_playing_back.store(false, Ordering::Relaxed);
    }

    pub fn toggle_recording(&self) {
        let is_recording = self.is_recording.load(Ordering::Relaxed);
        if is_recording {
            self.stop_recording();
        } else {
            self.start_recording();
        }
    }

    fn stop_recording(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.is_playing_back.store(true, Ordering::Relaxed);
    }
}

enum LoopState {
    Empty,
    Recording { start: usize },
    PlayOrOverdub { start: usize, end: usize },
}

/// Private not thread safe mutable state
struct LooperProcessorState<SampleType: num::Float> {
    loop_state: LoopState,
    looper_cursor: usize,
    num_channels: usize,
    looped_clip: InternalBuffer<SampleType>,
}

impl<SampleType: num::Float> LooperProcessorState<SampleType> {
    pub fn new() -> Self {
        LooperProcessorState {
            num_channels: 2,
            looper_cursor: 0,
            loop_state: LoopState::Empty,
            looped_clip: InternalBuffer::new(),
        }
    }

    pub fn increment_cursor(&mut self) {
        if let PlayOrOverdub { start, end } = self.loop_state {
            self.looper_cursor += 1;
            if self.looper_cursor == end + 1 {
                self.looper_cursor = start;
            }
        } else {
            self.looper_cursor += 1;
            self.looper_cursor %= self.looped_clip.num_samples();
        }
    }

    fn on_tick(&mut self, is_recording: bool, looper_cursor: usize) {
        match self.loop_state {
            // Loop has ended
            LoopState::Recording { start } if !is_recording => {
                self.loop_state = LoopState::PlayOrOverdub {
                    start,
                    end: looper_cursor,
                };
            }
            // Loop has started
            LoopState::Empty if is_recording => {
                self.loop_state = LoopState::Recording {
                    start: looper_cursor,
                };
            }
            _ => {}
        }

        self.increment_cursor();
    }
}

/// A single stereo looper
pub struct LooperProcessor<SampleType: num::Float> {
    pub id: String,
    state: LooperProcessorState<SampleType>,
    handle: Shared<LooperProcessorHandle>,
}

impl<SampleType: num::Float> LooperProcessor<SampleType> {
    pub fn new(handle: &Handle) -> Self {
        LooperProcessor {
            id: uuid::Uuid::new_v4().to_string(),
            state: LooperProcessorState::new(),
            handle: Shared::new(handle, LooperProcessorHandle::new()),
        }
    }

    pub fn handle(&self) -> Shared<LooperProcessorHandle> {
        self.handle.clone()
    }
}

impl<SampleType: num::Float + Send + Sync + std::ops::AddAssign> AudioProcessor
    for LooperProcessor<SampleType>
{
    type SampleType = SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        log::info!("Prepare looper {}", self.id);
        if settings.output_channels() != settings.input_channels() {
            log::error!("Prepare failed. Output/input channels mismatch");
            return;
        }

        let num_channels = settings.input_channels();
        self.state.num_channels = num_channels;
        self.state.looped_clip.resize(
            num_channels,
            settings.sample_rate(),
            Duration::from_secs(10),
        );
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        let zero = BufferType::SampleType::zero();
        for frame in data.frames_mut() {
            let should_playback_input = self.handle.playback_input.load(Ordering::Relaxed);
            let is_playing = self.handle.is_playing_back.load(Ordering::Relaxed);
            let is_recording = self.handle.is_recording.load(Ordering::Relaxed);
            let looper_cursor = self.state.looper_cursor;

            for (channel_num, sample) in frame.iter_mut().enumerate() {
                let loop_channel = self.state.looped_clip.channel(channel_num);

                // PLAYBACK SECTION:
                let dry_output = if !should_playback_input {
                    zero
                } else {
                    *sample
                };
                let looper_output = loop_channel[looper_cursor];
                let looper_output = if is_playing { looper_output } else { zero };

                let mixed_output = looper_output + dry_output;
                *sample = mixed_output;

                // RECORDING SECTION:
                if is_recording {
                    // When recording starts we'll store samples in the looper buffer
                    loop_channel[looper_cursor] = *sample + loop_channel[looper_cursor];
                }
            }

            self.state.on_tick(is_recording, looper_cursor);
        }
    }
}

impl<SampleType: num::Float> LooperProcessor<SampleType> {
    pub fn toggle_recording(&mut self) {
        self.handle.toggle_recording();
    }

    fn stop(&mut self) {
        self.handle.stop();
    }
}

impl<SampleType: num::Float + Send + Sync + std::ops::AddAssign> MidiEventHandler
    for LooperProcessor<SampleType>
{
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        for message in midi_messages {
            let status = message
                .bytes()
                .map(|bytes| rimd::Status::from_u8(bytes[0]).map(|status| (status, bytes)))
                .flatten();
            if let Some((status, bytes)) = status {
                match status {
                    Status::ControlChange => {
                        let cc_number = bytes[1];
                        let cc_value = bytes[2];
                        match (cc_number, cc_value) {
                            (80, 127) => self.toggle_recording(),
                            (81, 127) => {
                                self.stop();
                            }
                            _ => {}
                        }
                    }
                    Status::ProgramChange => {}
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::{rms_level, sine_buffer, test_level_equivalence};
    use audio_processor_traits::{
        AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer,
    };

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
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.process(&mut audio_buffer);
        assert_eq!(rms_level(&audio_buffer.slice()), 0.0);
    }

    #[test]
    fn test_looper_plays_its_input_back() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings.clone());

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        let mut audio_buffer = sine_buffer.clone();
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.process(&mut audio_buffer);

        test_level_equivalence(&sine_buffer, audio_buffer.slice(), 1, 1, 0.001);
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
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.handle().store_playback_input(false);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }
}
