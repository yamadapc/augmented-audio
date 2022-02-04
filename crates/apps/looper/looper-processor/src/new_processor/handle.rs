use atomic_refcell::AtomicRefCell;
use num_derive::{FromPrimitive, ToPrimitive};
use std::borrow::{Borrow, BorrowMut};

use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use audio_garbage_collector::{make_shared, make_shared_cell};
use basedrop::{Shared, SharedCell};

use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessorSettings, VecAudioBuffer};

use crate::AtomicEnum;

use super::scratch_pad;

struct CopyLoopClipParams<'a> {
    scratch_pad: &'a scratch_pad::ScratchPad,
    start_cursor: usize,
    length: usize,
}

fn copy_looped_clip(params: CopyLoopClipParams, result_buffer: &mut VecAudioBuffer<AtomicF32>) {
    let buffer = params.scratch_pad.buffer();

    result_buffer.resize(buffer.num_channels(), params.length, AtomicF32::new(0.0));

    for channel in 0..buffer.num_channels() {
        for i in 0..params.length {
            let index = (i + params.start_cursor) % buffer.num_samples();
            let sample = buffer.get(channel, index).clone();
            result_buffer.set(channel, i, sample);
        }
    }
}

fn empty_buffer(channels: usize, samples: usize) -> VecAudioBuffer<AtomicF32> {
    let mut b = VecAudioBuffer::new();
    b.resize(channels, samples, AtomicF32::new(0.0));
    b
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum LooperState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
    Paused = 3,
    Overdubbing = 4,
}

pub struct LooperHandle {
    /// Public params
    dry_volume: AtomicF32,
    wet_volume: AtomicF32,

    /// This looper's playback state
    state: AtomicEnum<LooperState>,
    start_cursor: AtomicUsize,
    length: AtomicUsize,
    /// Circular buffer that is always recording
    scratch_pad: SharedCell<scratch_pad::ScratchPad>,
    /// The current clip being played back
    looper_clip: SharedCell<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    /// Where playback is within the looped clip buffer
    cursor: AtomicUsize,
    options: LooperHandleOptions,
}

pub struct LooperHandleOptions {
    pub max_loop_length: Duration,
}

impl Default for LooperHandleOptions {
    fn default() -> Self {
        Self {
            max_loop_length: Duration::from_secs(10),
        }
    }
}

impl LooperHandle {
    pub fn new(options: LooperHandleOptions) -> Self {
        Self {
            dry_volume: AtomicF32::new(0.0),
            wet_volume: AtomicF32::new(1.0),

            state: AtomicEnum::new(LooperState::Empty),
            start_cursor: AtomicUsize::new(0),
            length: AtomicUsize::new(0),
            scratch_pad: make_shared_cell(scratch_pad::ScratchPad::new(VecAudioBuffer::new())),
            looper_clip: make_shared_cell(AtomicRefCell::new(VecAudioBuffer::new())),
            cursor: AtomicUsize::new(0),
            options,
        }
    }

    pub fn dry_volume(&self) -> f32 {
        self.dry_volume.get()
    }

    pub fn wet_volume(&self) -> f32 {
        self.wet_volume.get()
    }

    pub fn set_dry_volume(&self, value: f32) {
        self.dry_volume.set(value);
    }

    pub fn set_wet_volume(&self, value: f32) {
        self.wet_volume.set(value);
    }

    /// UI thread only
    pub fn toggle_recording(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording || old_state == LooperState::Overdubbing {
            self.stop_recording_allocating_loop();
        } else {
            self.start_recording();
        }
    }

    pub fn toggle_playback(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Playing
            || old_state == LooperState::Recording
            || old_state == LooperState::Overdubbing
        {
            self.stop_recording_allocating_loop();
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn start_recording(&self) -> LooperState {
        // Initial recording
        let old_state = self.state.get();
        if old_state == LooperState::Empty {
            let cursor = self.scratch_pad.get().cursor();
            self.start_cursor.store(cursor, Ordering::Relaxed);
            self.state.set(LooperState::Recording);
            self.length.store(0, Ordering::Relaxed);
            // Start overdub
        } else if old_state == LooperState::Paused || old_state == LooperState::Playing {
            self.state.set(LooperState::Overdubbing);
        }

        self.state.get()
    }

    pub fn clear(&self) {
        self.state.set(LooperState::Empty);
        // Clear the looper clip in case playback re-starts
        let clip = self.looper_clip.get();
        let clip = clip.deref().borrow();
        for sample in clip.slice() {
            sample.set(0.0);
        }
    }

    pub fn play(&self) {
        self.state.set(LooperState::Playing);
    }

    pub fn pause(&self) {
        self.state.set(LooperState::Paused);
    }

    pub fn stop_recording_allocating_loop(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording {
            let scratch_pad = self.scratch_pad.get();

            let mut result_buffer = self.looper_clip.get();
            let mut new_buffer = VecAudioBuffer::new();
            copy_looped_clip(
                CopyLoopClipParams {
                    scratch_pad: &scratch_pad,
                    start_cursor: self.start_cursor.load(Ordering::Relaxed),
                    length: self.length.load(Ordering::Relaxed),
                },
                &mut new_buffer,
            );
            self.looper_clip
                .set(make_shared(AtomicRefCell::new(new_buffer)));
            self.state.set(LooperState::Playing);
            self.cursor.store(0, Ordering::Relaxed);
        } else if old_state == LooperState::Overdubbing {
            self.state.set(LooperState::Playing);
        }
    }

    pub fn stop_recording_audio_thread_only(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording {
            let scratch_pad = self.scratch_pad.get();

            let result_buffer = self.looper_clip.get();
            let result_buffer = result_buffer.deref().try_borrow_mut().ok();
            if let Some(mut result_buffer) = result_buffer {
                copy_looped_clip(
                    CopyLoopClipParams {
                        scratch_pad: &scratch_pad,
                        start_cursor: self.start_cursor.load(Ordering::Relaxed),
                        length: self.length.load(Ordering::Relaxed),
                    },
                    &mut *result_buffer,
                );
                self.state.set(LooperState::Playing);
                self.cursor.store(0, Ordering::Relaxed);
            }
        } else if old_state == LooperState::Overdubbing {
            self.state.set(LooperState::Playing);
        }
    }

    pub fn looper_clip(&self) -> Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>> {
        self.looper_clip.get()
    }

    pub fn num_samples(&self) -> usize {
        self.length.load(Ordering::Relaxed)
    }

    pub fn is_recording(&self) -> bool {
        let state = self.state.get();
        state == LooperState::Recording || state == LooperState::Overdubbing
    }

    pub fn playhead(&self) -> usize {
        self.cursor.load(Ordering::Relaxed)
    }

    pub fn is_playing_back(&self) -> bool {
        let state = self.state.get();
        state == LooperState::Playing
            || state == LooperState::Recording
            || state == LooperState::Overdubbing
    }
}

/// MARK: Package private methods
impl LooperHandle {
    pub(crate) fn prepare(&self, settings: AudioProcessorSettings) {
        let max_loop_length_secs = self.options.max_loop_length.as_secs_f32();
        let max_loop_length_samples =
            (settings.sample_rate() * max_loop_length_secs).ceil() as usize;
        let num_channels = settings.input_channels();

        // Pre-allocate scratch-pad
        let scratch_pad =
            scratch_pad::ScratchPad::new(empty_buffer(num_channels, max_loop_length_samples));
        self.scratch_pad.set(make_shared(scratch_pad));

        // Pre-allocate looper clip
        let looper_clip = empty_buffer(num_channels, max_loop_length_samples);
        self.looper_clip
            .set(make_shared(AtomicRefCell::new(looper_clip)));
    }

    #[inline]
    pub(crate) fn process(&self, channel: usize, sample: f32) -> f32 {
        let scratch_pad = self.scratch_pad.get();
        scratch_pad.set(channel, sample);

        let out = match self.state.get() {
            LooperState::Playing => {
                let clip = self.looper_clip.get();
                let clip = clip.deref().borrow();
                let cursor = self.cursor.load(Ordering::Relaxed);
                let clip_out = clip.get(channel, cursor % clip.num_samples()).get();
                clip_out
            }
            LooperState::Overdubbing => {
                let clip = self.looper_clip.get();
                let clip = clip.deref().borrow();
                let cursor = self.cursor.load(Ordering::Relaxed);
                let clip_sample = clip.get(channel, cursor);
                let clip_out = clip_sample.get();
                clip_sample.set(clip_out + sample);
                clip_out
            }
            _ => 0.0,
        };

        self.dry_volume.get() * sample + self.wet_volume.get() * out
    }

    #[inline]
    pub(crate) fn after_process(&self) {
        let scratch_pad = self.scratch_pad.get();
        scratch_pad.after_process();

        let state = self.state.get();
        if state == LooperState::Recording {
            let len = self.length.load(Ordering::Relaxed) + 1;
            if len > scratch_pad.max_len() {
                self.stop_recording_audio_thread_only();
            } else {
                self.length.store(len, Ordering::Relaxed);
            }
        } else if state == LooperState::Playing || state == LooperState::Recording {
            let cursor = self.cursor.load(Ordering::Relaxed);
            let cursor = (cursor + 1) % self.length.load(Ordering::Relaxed);
            self.cursor.store(cursor, Ordering::Relaxed);
        }
    }

    pub(crate) fn debug(&self) {
        let shared = self.scratch_pad.get();
        let buffer = shared.buffer();
        // println!("scratch_buffer={:?}", buffer);
    }
}
