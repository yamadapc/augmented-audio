use std::sync::atomic::{AtomicBool, Ordering};

use audio_garbage_collector::Handle;
use audio_processor_traits::AtomicF32;

use crate::midi_map::MidiMap;

const QUEUE_CAPACITY: usize = 2048;

/// Public API types, which should be thread-safe
pub struct LooperProcessorHandle<SampleType> {
    is_recording: AtomicBool,
    is_playing_back: AtomicBool,
    playback_input: AtomicBool,
    should_clear: AtomicBool,
    dry_volume: AtomicF32,
    loop_volume: AtomicF32,
    midi_map: MidiMap,
    pub queue: atomic_queue::Queue<SampleType>,
}

impl<SampleType> LooperProcessorHandle<SampleType> {
    pub fn new(handle: &Handle) -> Self {
        LooperProcessorHandle {
            is_recording: AtomicBool::new(false),
            is_playing_back: AtomicBool::new(false),
            playback_input: AtomicBool::new(true),
            should_clear: AtomicBool::new(false),
            dry_volume: AtomicF32::new(1.0),
            loop_volume: AtomicF32::new(1.0),
            midi_map: MidiMap::new_with_handle(handle),
            queue: atomic_queue::Queue::new(QUEUE_CAPACITY),
        }
    }

    pub fn store_playback_input(&self, value: bool) {
        self.playback_input.store(value, Ordering::Relaxed);
    }

    pub fn start_recording(&self) {
        self.is_recording.store(true, Ordering::Relaxed);
    }

    pub fn clear(&self) {
        self.stop();
        self.should_clear.store(true, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
        self.is_playing_back.store(false, Ordering::Relaxed);
    }

    pub fn play(&self) {
        self.is_playing_back.store(true, Ordering::Relaxed);
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

    pub(crate) fn set_should_clear(&self, _value: bool) {
        self.should_clear.store(false, Ordering::Relaxed);
    }

    pub(crate) fn midi_map(&self) -> &MidiMap {
        &self.midi_map
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

impl<F: num::Float> LooperProcessorHandle<F> {
    pub fn parameters(&self) -> ProcessParameters<F> {
        ProcessParameters {
            should_clear: self.should_clear.load(Ordering::Relaxed),
            playback_input: self.playback_input.load(Ordering::Relaxed),
            is_playing_back: self.is_playing_back.load(Ordering::Relaxed),
            is_recording: self.is_recording.load(Ordering::Relaxed),
            loop_volume: F::from(self.loop_volume.get()).unwrap(),
            dry_volume: F::from(self.dry_volume.get()).unwrap(),
        }
    }
}
