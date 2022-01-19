use std::sync::atomic::{AtomicBool, Ordering};

use audio_garbage_collector::Handle;
use audio_processor_traits::{AtomicF32, AudioBuffer};

use crate::midi_map::MidiMap;
use crate::RecordingState;
use state::LooperProcessorState;

pub mod state;

/// Public API types, which should be thread-safe
pub struct LooperProcessorHandle {
    is_recording: AtomicBool,
    is_playing_back: AtomicBool,
    playback_input: AtomicBool,
    should_clear: AtomicBool,
    dry_volume: AtomicF32,
    loop_volume: AtomicF32,
    midi_map: MidiMap,
    pub(crate) state: LooperProcessorState,
}

impl LooperProcessorHandle {
    pub(crate) fn new(handle: &Handle, state: LooperProcessorState) -> Self {
        LooperProcessorHandle {
            is_recording: AtomicBool::new(false),
            is_playing_back: AtomicBool::new(false),
            playback_input: AtomicBool::new(true),
            should_clear: AtomicBool::new(false),
            // TODO: 0 is the default for testing
            dry_volume: AtomicF32::new(0.0),
            loop_volume: AtomicF32::new(1.0),
            midi_map: MidiMap::new_with_handle(handle),
            state,
        }
    }

    pub fn num_samples(&self) -> usize {
        self.state.num_samples()
    }

    pub fn loop_iterator(&self) -> impl Iterator<Item = f32> {
        self.state.loop_iterator()
    }

    pub(crate) fn state(&self) -> &LooperProcessorState {
        &self.state
    }

    pub fn store_playback_input(&self, value: bool) {
        self.playback_input.store(value, Ordering::Relaxed);
    }

    pub fn start_recording(&self) {
        self.is_recording.store(true, Ordering::Relaxed);
        self.state.looper_increment.set(1.0);
    }

    pub fn clear(&self) {
        self.stop();
        self.should_clear.store(true, Ordering::Relaxed);
        self.state.looper_increment.set(1.0);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.state.looper_increment.set(1.0);
        self.is_playing_back.store(false, Ordering::Relaxed);
    }

    pub fn play(&self) {
        self.is_playing_back.store(true, Ordering::Relaxed);
        self.state.looper_increment.set(1.0);
    }

    pub fn toggle_playback(&self) {
        let is_playing_back = self.is_playing_back.load(Ordering::Relaxed);
        if is_playing_back {
            self.stop();
        } else {
            self.play();
        }
    }

    pub fn toggle_recording(&self) {
        let is_recording = self.is_recording.load(Ordering::Relaxed);
        if is_recording {
            self.stop_recording();
        } else {
            self.start_recording();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }

    pub fn is_playing_back(&self) -> bool {
        self.is_playing_back.load(Ordering::Relaxed)
    }

    pub fn stop_recording(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.is_playing_back.store(true, Ordering::Relaxed);
    }

    pub fn set_dry_volume(&self, value: f32) {
        self.dry_volume.set(value);
    }

    pub fn set_loop_volume(&self, value: f32) {
        self.loop_volume.set(value);
    }

    pub fn playhead(&self) -> usize {
        let start = self.state.loop_state.start.load(Ordering::Relaxed);
        let end = self.state.loop_state.end.load(Ordering::Relaxed);
        let cursor = self.state.looper_cursor.get() as usize;
        let clip = self.state.looped_clip.get();
        if end > start {
            cursor - start
        } else {
            cursor + (clip.num_samples() - start)
        }
    }

    pub(crate) fn set_should_clear(&self, _value: bool) {
        self.should_clear.store(false, Ordering::Relaxed);
    }

    pub(crate) fn midi_map(&self) -> &MidiMap {
        &self.midi_map
    }

    pub fn debug(&self) -> String {
        format!(
            "cursor={}/{} start={} end={} state={:?} length={}",
            self.state.looper_cursor.get(),
            self.state.looped_clip.get().num_samples(),
            self.state.loop_state.start.load(Ordering::Relaxed),
            self.state.loop_state.end.load(Ordering::Relaxed),
            self.state.loop_state.recording_state.get(),
            self.state.num_samples()
        )
    }

    pub(crate) fn force_stop_if_overflowing(&self, looper_cursor: usize) -> bool {
        // STOP looper if going above max duration
        if self.is_recording()
            && self.state.num_samples() >= self.state.looped_clip.get().num_samples() - 1
        {
            self.state
                .loop_state
                .recording_state
                .set(RecordingState::Playing);
            self.state
                .looper_cursor
                .set(self.state.loop_state.start.load(Ordering::Relaxed) as f32);
            self.state
                .loop_state
                .end
                .store(looper_cursor - 1, Ordering::Relaxed);
            self.stop_recording();
            false
        } else {
            true
        }
    }
}

/// State read from LooperProcessorHandle on each sample
pub struct ProcessParameters<F: num::Float> {
    /// The loop should be cleared in this tick
    pub should_clear: bool,
    /// Input to the processor should be forwarded
    pub playback_input: bool,
    /// The looper is playing
    pub is_playing_back: bool,
    /// The looper is recording
    pub is_recording: bool,
    /// The volume of the looper
    pub loop_volume: F,
    /// The volume of the dry signal
    pub dry_volume: F,
}

impl LooperProcessorHandle {
    pub fn parameters(&self) -> ProcessParameters<f32> {
        ProcessParameters {
            should_clear: self.should_clear.load(Ordering::Relaxed),
            playback_input: self.playback_input.load(Ordering::Relaxed),
            is_playing_back: self.is_playing_back.load(Ordering::Relaxed),
            is_recording: self.is_recording.load(Ordering::Relaxed),
            loop_volume: self.loop_volume.get(),
            dry_volume: self.dry_volume.get(),
        }
    }
}
