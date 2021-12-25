use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use basedrop::SharedCell;
use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

use audio_garbage_collector::{make_shared, make_shared_cell, Handle, Shared};
use audio_processor_standalone::standalone_vst::vst::util::AtomicFloat;
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{
    AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler,
    MidiMessageLike, VecAudioBuffer,
};

use crate::atomic_enum::AtomicEnum;
pub use crate::handle::LooperProcessorHandle;
use crate::handle::ProcessParameters;
use crate::midi_map::{Action, MidiSpec};
use crate::RecordingState::Recording;

mod atomic_enum;
mod handle;
mod midi_map;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
enum RecordingState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
}

#[cfg(test)]
mod test_recording_state {
    use num::FromPrimitive;

    use super::*;

    #[test]
    fn test_from_atomic() {
        let state = 0;
        let state = RecordingState::from_usize(state).unwrap();
        assert_eq!(state, RecordingState::Empty);
    }
}

struct LoopState {
    recording_state: AtomicEnum<RecordingState>,
    start: AtomicUsize,
    end: AtomicUsize,
}

pub struct LooperProcessorState {
    loop_state: LoopState,
    looper_cursor: AtomicFloat,
    looper_increment: AtomicFloat,
    num_channels: AtomicUsize,
    looped_clip: Shared<SharedCell<VecAudioBuffer<AtomicF32>>>,
}

impl LooperProcessorState {
    pub fn new() -> Self {
        LooperProcessorState {
            num_channels: AtomicUsize::new(2usize),
            looper_cursor: AtomicFloat::new(0.0),
            looper_increment: AtomicFloat::new(1.0),
            loop_state: LoopState {
                recording_state: AtomicEnum::new(RecordingState::Empty),
                start: AtomicUsize::new(0),
                end: AtomicUsize::new(0),
            },
            looped_clip: make_shared(make_shared_cell(VecAudioBuffer::new())),
        }
    }

    pub fn increment_cursor(&self) {
        let mut looper_cursor = self.looper_cursor.get();
        looper_cursor += self.looper_increment.get();

        // This is a slowdown to stop feature that needs to be properly exposed
        // if self.loop_state.recording_state.get() == RecordingState::Playing {
        //     self.looper_increment
        //         .set((self.looper_increment.get() - 0.00001134).max(0.0));
        // }

        let num_samples = self.looped_clip.get().num_samples() as f32;
        let recording_state = self.loop_state.recording_state.get();
        if recording_state == RecordingState::Playing {
            let start = self.loop_state.start.load(Ordering::Relaxed) as f32;
            let end = self.loop_state.end.load(Ordering::Relaxed) as f32;

            if end > start {
                if looper_cursor >= end {
                    looper_cursor = start;
                }
            } else {
                // End point is before start
                let loop_length = num_samples - start + end;
                if looper_cursor >= start {
                    let cursor_relative_to_start = looper_cursor - start;
                    if cursor_relative_to_start >= loop_length {
                        looper_cursor = start;
                    }
                } else {
                    let cursor_relative_to_start = looper_cursor - end + num_samples - start;
                    if cursor_relative_to_start >= loop_length {
                        looper_cursor = start;
                    }
                }
            }
        } else {
            looper_cursor %= num_samples;
        }

        self.looper_cursor.set(looper_cursor as f32);
    }

    fn clear(&self) {
        self.loop_state.recording_state.set(RecordingState::Empty);
        self.looper_cursor.set(0.0);
        self.loop_state.start.store(0, Ordering::Relaxed);
        self.loop_state.end.store(0, Ordering::Relaxed);
        for sample in self.looped_clip.get().slice() {
            sample.set(0.0);
        }
    }

    fn on_tick(&self, is_recording: bool, looper_cursor: usize) {
        match self.loop_state.recording_state.get() {
            // Loop has ended
            RecordingState::Recording if !is_recording => {
                self.loop_state.recording_state.set(RecordingState::Playing);
                self.loop_state.end.store(looper_cursor, Ordering::Relaxed);
            }
            // Loop has started
            RecordingState::Empty if is_recording => {
                self.loop_state
                    .recording_state
                    .set(RecordingState::Recording);
                self.loop_state
                    .start
                    .store(looper_cursor, Ordering::Relaxed);
            }
            _ => {}
        }

        self.increment_cursor();
    }

    /// Returns the size of the current loop
    pub fn num_samples(&self) -> usize {
        let recording_state = self.loop_state.recording_state.get();

        if recording_state == RecordingState::Empty {
            return 0;
        }

        let clip = self.looped_clip.get();
        let start = self.loop_state.start.load(Ordering::Relaxed);
        let end = self.end_cursor();

        if end >= start {
            end - start
        } else {
            clip.num_samples() - start + end
        }
    }

    /// Either the looper cursor or the end
    fn end_cursor(&self) -> usize {
        let recording_state = self.loop_state.recording_state.get();
        if recording_state == RecordingState::Recording {
            self.looper_cursor.get() as usize
        } else {
            self.loop_state.end.load(Ordering::Relaxed)
        }
    }

    pub fn loop_iterator(&self) -> impl Iterator<Item = f32> {
        let start = self.loop_state.start.load(Ordering::Relaxed);
        let clip = self.looped_clip.get();

        (0..self.num_samples()).map(move |index| {
            let idx = (start + index) % clip.num_samples();
            let mut s = 0.0;
            for channel in 0..clip.num_channels() {
                s += unsafe { clip.get_unchecked(channel, idx).get() };
            }
            s
        })
    }

    pub fn looped_clip(&self) -> &Shared<SharedCell<VecAudioBuffer<AtomicF32>>> {
        &self.looped_clip
    }
}

/// A single stereo looper
pub struct LooperProcessor {
    id: String,
    handle: Shared<LooperProcessorHandle>,
}

impl LooperProcessor {
    pub fn new(handle: &Handle) -> Self {
        let state = LooperProcessorState::new();
        LooperProcessor {
            id: uuid::Uuid::new_v4().to_string(),
            handle: Shared::new(handle, LooperProcessorHandle::new(handle, state)),
        }
    }

    pub fn handle(&self) -> Shared<LooperProcessorHandle> {
        self.handle.clone()
    }

    fn force_stop(&mut self, looper_cursor: usize) -> bool {
        // STOP looper if going above max duration
        if self.handle.is_recording()
            && self.handle.state.num_samples()
                >= self.handle.state.looped_clip.get().num_samples() - 1
        {
            self.handle
                .state
                .loop_state
                .recording_state
                .set(RecordingState::Playing);
            self.handle
                .state
                .looper_cursor
                .set(self.handle.state.loop_state.start.load(Ordering::Relaxed) as f32);
            self.handle
                .state
                .loop_state
                .end
                .store(looper_cursor - 1, Ordering::Relaxed);
            self.handle.stop_recording();
            false
        } else {
            true
        }
    }
}

impl AudioProcessor for LooperProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        log::info!("Prepare looper {}", self.id);
        if settings.output_channels() != settings.input_channels() {
            log::error!("Prepare failed. Output/input channels mismatch");
            return;
        }

        let num_channels = settings.input_channels();
        self.handle
            .state
            .num_channels
            .store(num_channels, Ordering::Relaxed);
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(
            num_channels,
            (settings.sample_rate() * 10.0) as usize,
            AtomicF32::new(0.0),
        );
        self.handle.state.looped_clip.set(make_shared(buffer));
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample_index in 0..data.num_samples() {
            let parameters = self.handle.parameters();
            if parameters.should_clear {
                self.handle.set_should_clear(false);
                self.handle.state.clear();
            }

            let looper_cursor = self.handle.state.looper_cursor.get() as usize;

            for channel_num in 0..data.num_channels() {
                self.process_sample(&parameters, data, sample_index, channel_num)
            }

            if self.handle.is_playing_back() || self.handle.is_recording() {
                if self.force_stop(looper_cursor) {
                    self.handle
                        .state
                        .on_tick(parameters.is_recording, looper_cursor);
                }
            }
        }
    }
}

impl LooperProcessor {
    fn process_sample<BufferType: AudioBuffer<SampleType = f32>>(
        &mut self,
        parameters: &ProcessParameters<f32>,
        data: &mut BufferType,
        sample_index: usize,
        channel_num: usize,
    ) {
        let flooper_cursor: f32 = self.handle.state.looper_cursor.get();
        let looper_cursor: usize = flooper_cursor as usize;

        let ProcessParameters {
            playback_input,
            is_playing_back,
            is_recording,
            loop_volume,
            dry_volume,
            ..
        } = *parameters;

        // INPUT SECTION:
        let input = *data.get(channel_num, sample_index);
        let dry_output = if !playback_input { 0.0 } else { input };

        // PLAYBACK SECTION:
        let looped_clip = self.handle.state.looped_clip.get();
        let looper_output = if is_playing_back {
            let looper_output1 = looped_clip.get(channel_num, looper_cursor).get();
            let looper_output2 = looped_clip
                .get(channel_num, (looper_cursor + 1) % looped_clip.num_samples())
                .get();

            let offset = flooper_cursor - looper_cursor as f32;

            looper_output1 * (1.0 - offset) + looper_output2 * offset
        } else {
            0.0
        };

        // RECORDING SECTION:
        if is_recording {
            // When recording starts we'll store samples in the looper buffer
            let sample = looped_clip.get(channel_num, looper_cursor);
            sample.set(*data.get(channel_num, sample_index) + sample.get());
        }

        // OUTPUT SECTION:
        let mixed_output = loop_volume * looper_output + dry_volume * dry_output;
        data.set(channel_num, sample_index, mixed_output);
    }
}

impl MidiEventHandler for LooperProcessor {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        for message in midi_messages {
            let status = message
                .bytes()
                .map(|bytes| rimd::Status::from_u8(bytes[0]).map(|status| (status, bytes)))
                .flatten();
            if let Some((_status, bytes)) = status {
                if let Some(action) = self
                    .handle
                    .midi_map()
                    .get(&MidiSpec::new(bytes[0], bytes[1]))
                {
                    match action {
                        Action::SetRecording(value) => {
                            if value {
                                self.handle.start_recording()
                            } else {
                                self.handle.stop_recording()
                            }
                        }
                        Action::SetPlayback(value) => {
                            if value {
                                self.handle.play()
                            } else {
                                self.handle.stop()
                            }
                        }
                        Action::Clear => {
                            self.handle.clear();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::rms_level;
    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_testing_helpers::test_level_equivalence;

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

        looper.prepare(settings);

        let mut silence_buffer = Vec::new();
        // Produce 0.1 second empty buffer
        silence_buffer.resize((0.1 * settings.sample_rate()) as usize, 0.0);

        let mut audio_buffer = silence_buffer.clone();
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.process(&mut audio_buffer);
        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }

    #[test]
    fn test_looper_plays_its_input_back() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings);

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
        looper.prepare(settings);

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        assert_ne!(sine_buffer.len(), 0);
        println!("Sine samples: {:?}", sine_buffer);
        println!("Sine RMS: {}", rms_level(&sine_buffer));
        assert_ne!(rms_level(&sine_buffer), 0.0);

        let mut audio_buffer = sine_buffer;
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.handle().store_playback_input(false);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }
}
