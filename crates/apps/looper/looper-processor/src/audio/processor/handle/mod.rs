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
//! Some functions in this module are annotated with `#[inline]`. This gives us between 1-2%
//! performance improvement.
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use atomic_refcell::AtomicRefCell;
use basedrop::{Shared, SharedCell};
use num_derive::{FromPrimitive, ToPrimitive};

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::{AtomicEnum, AtomicValue};
pub use quantize_mode::{QuantizeMode, QuantizeOptions};
use utils::CopyLoopClipParams;

use crate::audio::multi_track_looper::metrics::audio_thread_logger::AudioThreadLogger;
use crate::audio::processor::handle::scratch_pad::ScratchPad;
use crate::audio::{
    loop_quantization::{LoopQuantizer, QuantizeInput},
    time_info_provider::{HostCallback, TimeInfoProvider, TimeInfoProviderImpl},
};

mod quantize_mode;
mod scratch_pad;
mod utils;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// `LooperState` represents the current state the looper is in.
///
/// The following is a simplified diagram of states and possible transitions:
///
/// ```mermaid
/// stateDiagram-v2
///     [*] --> Empty
///     Empty --> Recording
///     RecordingScheduled: Recording scheduled
///     Empty --> RecordingScheduled
///     RecordingScheduled --> Recording
///     Recording --> Playing
///     Recording --> Paused
///     Playing --> Paused
///     Paused --> Playing
///     PlayingScheduled: Playing scheduled
///     Paused --> PlayingScheduled
///     PlayingScheduled --> Playing            
/// ```
///
/// This is simplified because there's an "overdubbing" state, which is separate from recording
/// and because pause/stop can be called at any point & currently is not well defined (it's on the
/// caller to decide and implement if pausing the looper mid recording should capture the partial
/// recording or do something else).
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum LooperState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
    Paused = 3,
    Overdubbing = 4,
    RecordingScheduled = 5,
    PlayingScheduled = 6,
}

#[derive(Clone)]
pub struct LooperOptions {
    pub max_loop_length: Duration,
    pub host_callback: Option<HostCallback>,
}

pub type LooperClip = SharedCell<AtomicRefCell<VecAudioBuffer<AtomicF32>>>;
pub type LooperClipRef = Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>;

pub fn looper_clip_copy_to_vec_buffer(buffer: &LooperClipRef) -> VecAudioBuffer<f32> {
    let buffer = buffer.borrow();
    let buffer = buffer.deref();

    let mut work_buffer = VecAudioBuffer::new();
    work_buffer.resize(buffer.num_channels(), buffer.num_samples(), 0.0);
    for (loop_frame, work_frame) in buffer.frames().zip(work_buffer.frames_mut()) {
        for (loop_sample, work_sample) in loop_frame.iter().zip(work_frame.iter_mut()) {
            *work_sample = loop_sample.get()
        }
    }
    work_buffer
}

impl Default for LooperOptions {
    fn default() -> Self {
        Self {
            max_loop_length: Duration::from_secs(crate::MAX_LOOP_LENGTH_SECS as u64),
            host_callback: None,
        }
    }
}

pub struct LooperHandle {
    // Public params
    /// Volume of loop playback
    dry_volume: AtomicF32,
    /// Passthrough volume
    wet_volume: AtomicF32,

    /// A number between 0 and 1 representing an offset from loop to start
    start_offset: AtomicF32,
    /// A number between 0 and 1 representing an offset from loop to end
    end_offset: AtomicF32,
    /// The loop will linearly fade in from position 0 to this position
    fade_start: AtomicF32,
    /// The loop will linearly fade out from this position until the end
    fade_end: AtomicF32,
    /// Playback speed, 1 means 1x playback, 0 means no playback, -1 means reverse playback
    /// and so on
    speed: AtomicF32,
    /// Defaults to true, if false the loop will not repeat until triggered.
    loop_enabled: AtomicBool,

    /// This looper's playback state
    state: AtomicEnum<LooperState>,
    start_cursor: AtomicUsize,
    length: AtomicUsize,
    scheduled_playback: AtomicUsize,
    /// Circular buffer that is always recording
    scratch_pad: AtomicRefCell<scratch_pad::ScratchPad>,
    /// The current clip being played back
    looper_clip: LooperClip,
    /// Temporary swap buffer
    looper_clip1: LooperClip,
    /// Where playback is within the looped clip buffer
    cursor: AtomicF32,
    /// Provides time information
    time_info_provider: Shared<TimeInfoProviderImpl>,
    tick_time: AtomicBool,

    options: LooperOptions,

    settings: SharedCell<AudioProcessorSettings>,
    quantize_options: QuantizeOptions,
}

impl Default for LooperHandle {
    fn default() -> Self {
        Self::from_options(Default::default())
    }
}

pub enum ToggleRecordingResult {
    StartedRecording,
    StoppedRecording,
    StopRecordingScheduled,
}

pub enum LooperHandleThread {
    AudioThread,
    OtherThread,
}

// MARK: Constructors
impl LooperHandle {
    pub fn new(options: LooperOptions, time_info_provider: Shared<TimeInfoProviderImpl>) -> Self {
        Self {
            dry_volume: AtomicF32::new(0.0),
            wet_volume: AtomicF32::new(1.0),

            start_offset: AtomicF32::new(0.0),
            end_offset: AtomicF32::new(1.0),
            fade_start: AtomicF32::new(0.0),
            fade_end: AtomicF32::new(0.0),
            speed: AtomicF32::new(1.0),
            loop_enabled: AtomicBool::new(true),

            state: AtomicEnum::new(LooperState::Empty),
            start_cursor: AtomicUsize::new(0),
            length: AtomicUsize::new(0),
            scratch_pad: AtomicRefCell::new(scratch_pad::ScratchPad::new(VecAudioBuffer::new())),
            looper_clip: make_shared_cell(AtomicRefCell::new(VecAudioBuffer::new())),
            looper_clip1: make_shared_cell(AtomicRefCell::new(VecAudioBuffer::new())),
            scheduled_playback: AtomicUsize::new(0),
            cursor: AtomicF32::new(0.0),
            time_info_provider,
            tick_time: AtomicBool::new(true),
            options,
            settings: make_shared_cell(Default::default()),
            quantize_options: QuantizeOptions::default(),
        }
    }

    pub fn from_options(options: LooperOptions) -> Self {
        let time_info_provider = make_shared(TimeInfoProviderImpl::new(options.host_callback));
        Self::new(options, time_info_provider)
    }
}

// MARK: Public getters/setters
impl LooperHandle {
    /// Pass-through volume
    pub fn dry_volume(&self) -> f32 {
        self.dry_volume.get()
    }

    /// Volume of the looper output
    pub fn wet_volume(&self) -> f32 {
        self.wet_volume.get()
    }

    /// Playback speed, may be a negative number
    pub fn speed(&self) -> f32 {
        self.speed.get()
    }

    /// Set pass-through volume
    pub fn set_dry_volume(&self, value: f32) {
        self.dry_volume.set(value);
    }

    /// Set looper volume
    pub fn set_wet_volume(&self, value: f32) {
        self.wet_volume.set(value);
    }

    /// Set a start offset as a percentage of the looper length (0-1)
    ///
    /// Whenever the looper repeats or is triggered it'll start from the sample matching this
    /// offset.
    pub fn set_start_offset(&self, value: f32) {
        self.start_offset.set(value);
    }

    /// Set an end offset as a percentage of the looper length (0-1)
    pub fn set_end_offset(&self, value: f32) {
        self.end_offset.set(value);
    }

    pub fn set_fade_start(&self, value: f32) {
        self.fade_start.set(value);
    }

    pub fn set_fade_end(&self, value: f32) {
        self.fade_end.set(value);
    }

    pub fn set_speed(&self, value: f32) {
        self.speed.set(value);
    }

    pub fn set_loop_enabled(&self, value: bool) {
        self.loop_enabled.store(value, Ordering::Relaxed);
    }

    pub fn set_tick_time(&self, value: bool) {
        self.tick_time.set(value)
    }

    pub fn trigger(&self) {
        self.cursor.set(self.get_start_samples());
    }

    pub fn quantize_options(&self) -> &QuantizeOptions {
        &self.quantize_options
    }

    pub fn time_info_provider(&self) -> &TimeInfoProviderImpl {
        &self.time_info_provider
    }

    pub fn set_tempo(&self, tempo: f32) {
        self.time_info_provider.set_tempo(tempo);
        self.time_info_provider.play();
    }

    pub fn looper_clip(&self) -> Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>> {
        self.looper_clip.get()
    }

    pub fn num_samples(&self) -> usize {
        self.length.load(Ordering::Relaxed)
    }

    pub fn playhead(&self) -> usize {
        self.cursor.get() as usize
    }
}

// MARK: Computed properties
impl LooperHandle {
    pub fn is_empty(&self) -> bool {
        self.num_samples() == 0 || self.state.get() == LooperState::Empty
    }

    pub fn is_recording(&self) -> bool {
        let state = self.state.get();
        state == LooperState::Recording || state == LooperState::Overdubbing
    }

    pub fn is_playing_back(&self) -> bool {
        let state = self.state.get();
        state == LooperState::Playing
            || state == LooperState::Recording
            || state == LooperState::Overdubbing
    }
}

/// MARK: Actions to change the looper state:
///
/// * Start recording
/// * Stop recording
/// * Start playback
/// * Stop playback
/// * Clear
///
impl LooperHandle {
    /// Return the real start cursor based on start offset & length
    fn get_start_samples(&self) -> f32 {
        self.start_offset.get() * self.length.get() as f32
    }

    fn get_end_samples(&self) -> f32 {
        self.end_offset.get() * self.length.get() as f32
    }

    pub fn toggle_recording(&self, thread: LooperHandleThread) -> ToggleRecordingResult {
        let old_state = self.state.get();
        if old_state == LooperState::Recording || old_state == LooperState::Overdubbing {
            if let LooperHandleThread::OtherThread = thread {
                self.stop_recording_allocating_loop();
            } else {
                self.stop_recording_audio_thread_only();
            }
            ToggleRecordingResult::StoppedRecording
        } else {
            self.start_recording();
            ToggleRecordingResult::StartedRecording
        }
    }

    /// Toggle playback. Return true if the looper is playing after this.
    pub fn toggle_playback(&self) -> bool {
        let old_state = self.state.get();
        if old_state == LooperState::Playing
            || old_state == LooperState::Recording
            || old_state == LooperState::Overdubbing
        {
            self.stop_recording_allocating_loop();
            self.pause();
            false
        } else {
            self.play();
            true
        }
    }

    fn get_quantized_offset(&self) -> Option<i32> {
        let time_info = self.time_info_provider.get_time_info();
        let beat_info = time_info.tempo().zip(time_info.position_beats());
        beat_info.map(|(tempo, position_beats)| {
            let quantizer = LoopQuantizer::new(self.quantize_options.inner());
            quantizer.quantize(QuantizeInput {
                tempo: tempo as f32,
                sample_rate: self.settings.get().sample_rate(),
                position_beats: position_beats as f32,
                position_samples: 0,
            })
        })
    }

    pub fn start_recording(&self) -> LooperState {
        // Initial recording
        let old_state = self.state.get();
        if old_state == LooperState::Empty {
            let scratch_pad = self.scratch_pad.borrow();
            let cursor = scratch_pad.cursor() as i32;
            let quantized_offset = self.get_quantized_offset();
            let is_recording_scheduled = quantized_offset.map(|offset| offset > 0).unwrap_or(false);
            let cursor = quantized_offset
                .map(|offset| (cursor + offset))
                .unwrap_or(cursor);

            self.start_cursor.store(cursor as usize, Ordering::Relaxed);
            self.state.set(if is_recording_scheduled {
                LooperState::RecordingScheduled
            } else {
                LooperState::Recording
            });
            self.length.store(0, Ordering::Relaxed);
            // Start overdub
        } else if old_state == LooperState::Paused || old_state == LooperState::Playing {
            if self.tick_time.get() {
                self.time_info_provider.play();
            }
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
        self.length.store(0, Ordering::Relaxed);
    }

    pub fn play(&self) {
        if self.tick_time.get() {
            self.time_info_provider.play();
        }
        self.state.set(LooperState::Playing);
        // TODO - This should be a parameter, or there should be a "Stopped" state
        self.cursor.set(self.get_start_samples());
    }

    pub fn stop(&self) {
        self.pause();
        self.cursor.set(self.get_start_samples());
    }

    pub fn pause(&self) {
        if self.tick_time.get() {
            self.time_info_provider.pause();
        }
        if self.state.get() != LooperState::Empty {
            self.state.set(LooperState::Paused);
        }
    }

    pub fn stop_recording(&self, thread: LooperHandleThread) {
        match thread {
            LooperHandleThread::OtherThread => self.stop_recording_allocating_loop(),
            LooperHandleThread::AudioThread => self.stop_recording_audio_thread_only(),
        }
    }

    /// Stops recording by copying the current scratch buffer onto a new looper buffer and swapping
    /// the old one with the new
    fn stop_recording_allocating_loop(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording || old_state == LooperState::PlayingScheduled {
            let cursor = self.scratch_pad.borrow().cursor();
            let quantized_offset = self.get_quantized_offset();
            let is_stop_scheduled = quantized_offset.map(|offset| offset > 0).unwrap_or(false);

            if is_stop_scheduled {
                log::info!("Stop is scheduled in {:?} samples", quantized_offset);
                let final_cursor = ((cursor as i32) + quantized_offset.unwrap_or(0)) as usize;
                self.scheduled_playback.set(final_cursor);
                self.state.set(LooperState::PlayingScheduled);
            } else {
                let scratch_pad = self.scratch_pad.borrow();

                let mut new_buffer = VecAudioBuffer::new();
                utils::copy_looped_clip(
                    CopyLoopClipParams {
                        scratch_pad: &scratch_pad,
                        start_cursor: self.start_cursor.load(Ordering::Relaxed),
                        length: (self.length.load(Ordering::Relaxed) as i32
                            + quantized_offset.unwrap_or(0))
                            as usize,
                    },
                    &mut new_buffer,
                );
                self.looper_clip
                    .set(make_shared(AtomicRefCell::new(new_buffer)));
                if self.tick_time.get() {
                    self.time_info_provider.play();
                }
                self.state.set(LooperState::Playing);
                self.cursor.set(0.0);
            }
        } else if old_state == LooperState::Overdubbing {
            self.state.set(LooperState::Playing);
        }
    }

    /// Stop recording by copying the scratch buffer in-place into the current looper buffer
    ///
    /// This currently is only good for the audio-thread, because it takes a mutable reference on the
    /// looper clip.
    ///
    /// Note:
    ///
    /// * The mutable reference is needed to resize the buffer
    /// * The clip is guaranteed to never resize in this function, because it can only of a certain
    ///   pre-allocated maximum length
    ///
    /// Since another reader might have a reference to the buffer, in this scheme, we need to swap
    /// buffers here. A temporary buffer is used. Samples are copied onto it and then that buffer
    /// is swapped with the current playback buffer.
    fn stop_recording_audio_thread_only(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording || old_state == LooperState::PlayingScheduled {
            let scratch_pad = self.scratch_pad.borrow();

            let shared_buffer = self.looper_clip1.get();
            if let Some(mut buffer) = shared_buffer.deref().try_borrow_mut().ok() {
                utils::copy_looped_clip(
                    CopyLoopClipParams {
                        scratch_pad: &scratch_pad,
                        start_cursor: self.start_cursor.load(Ordering::Relaxed),
                        length: self.length.load(Ordering::Relaxed),
                    },
                    &mut *buffer,
                );
                if self.tick_time.get() {
                    self.time_info_provider.play();
                }
                self.cursor.set(0.0);
                self.state.set(LooperState::Playing);
            }
            self.looper_clip.set(shared_buffer);
        } else if old_state == LooperState::Overdubbing {
            self.state.set(LooperState::Playing);
        }
    }
}

/// MARK: Buffer override
impl LooperHandle {
    /// Override the looper memory buffer.
    /// Not real-time safe, must be called out of the audio-thread.
    pub fn set_looper_buffer(&self, buffer: &impl AudioBuffer<SampleType = f32>) {
        let mut new_buffer: VecAudioBuffer<AtomicF32> = VecAudioBuffer::new();
        new_buffer.resize(
            buffer.num_channels(),
            buffer.num_samples(),
            AtomicF32::new(0.0),
        );
        for (source_frame, dest_frame) in buffer.frames().zip(new_buffer.frames_mut()) {
            for (source_sample, dest_sample) in source_frame.iter().zip(dest_frame) {
                dest_sample.set(*source_sample);
            }
        }

        let new_length = new_buffer.num_samples();
        self.looper_clip.set(make_shared(new_buffer.into()));
        self.length.set(new_length);
        self.state.set(LooperState::Paused);
        self.cursor.set(self.get_start_samples());
    }
}

/// MARK: Package private methods
impl LooperHandle {
    /// # Safety
    ///
    /// Attempting to call `LooperHandle::prepare` while the looper is running on another thread
    /// might panic, because this method will try to get a mutable reference to internal buffers.
    pub(crate) fn prepare(&self, settings: AudioProcessorSettings) {
        let max_loop_length_secs = self.options.max_loop_length.as_secs_f32();
        let max_loop_length_samples =
            (settings.sample_rate() * max_loop_length_secs).ceil() as usize;
        let num_channels = settings.output_channels();

        // Pre-allocate scratch-pad
        let scratch_pad = scratch_pad::ScratchPad::new(utils::empty_buffer(
            num_channels,
            max_loop_length_samples,
        ));
        *self.scratch_pad.borrow_mut() = scratch_pad;

        // Pre-allocate looper clip
        if self.state.get() == LooperState::Empty {
            self.looper_clip
                .set(make_shared(AtomicRefCell::new(utils::empty_buffer(
                    num_channels,
                    max_loop_length_samples,
                ))));
            self.looper_clip1
                .set(make_shared(AtomicRefCell::new(utils::empty_buffer(
                    num_channels,
                    max_loop_length_samples,
                ))));
        }

        self.time_info_provider
            .set_sample_rate(settings.sample_rate());

        self.settings.set(make_shared(settings));
    }

    pub fn state(&self) -> LooperState {
        self.state.get()
    }

    #[inline]
    pub(crate) fn process(&self, channel: usize, sample: f32) -> f32 {
        let scratch_pad = self.scratch_pad.borrow();
        scratch_pad.set(channel, sample);

        let state = self.state.get();
        let out = match state {
            LooperState::Playing => {
                let clip = self.looper_clip.get();
                let clip = clip.deref().borrow();

                // interpolation with next sample
                let cursor = self.cursor.get();
                let delta_next_sample = cursor - cursor.floor();
                let cursor = cursor as usize;
                let clip_out1 = clip.get(channel, cursor % clip.num_samples()).get();
                let clip_out2 = clip.get(channel, (cursor + 1) % clip.num_samples()).get();

                self.apply_wet_volume(clip_out1 + delta_next_sample * (clip_out2 - clip_out1))
            }
            LooperState::Overdubbing => {
                let clip = self.looper_clip.get();
                let clip = clip.deref().borrow();
                // TODO - There should be separate read/write cursors (?)
                let cursor = self.cursor.get() as usize;
                let clip_sample = clip.get(channel, cursor % clip.num_samples());
                let clip_out = clip_sample.get();

                clip_sample.set(clip_out + sample);
                self.apply_wet_volume(clip_out)
            }
            _ => 0.0,
        };

        self.dry_volume.get() * sample + out
    }

    #[inline]
    fn apply_wet_volume(&self, out: f32) -> f32 {
        let fade_in_volume = self.get_fade_in_volume(self.cursor.get());
        let fade_out_volume = self.get_fade_out_volume(self.cursor.get());
        self.wet_volume.get() * out * fade_in_volume * fade_out_volume
    }

    fn get_fade_out_volume(&self, cursor: f32) -> f32 {
        let fade_perc = self.fade_end.get();
        let length = self.length.get() as f32;
        let fade_samples = fade_perc * length;
        let distance_end = length - cursor - 1.0;
        (distance_end / fade_samples).min(1.0).max(0.0)
    }

    fn get_fade_in_volume(&self, cursor: f32) -> f32 {
        calculate_fade_volume(self.fade_start.get(), self.length.get(), cursor)
    }

    #[inline]
    pub(crate) fn after_process(&self) {
        let scratch_pad = self.scratch_pad.borrow();
        scratch_pad.after_process();
        if self.tick_time.load(Ordering::Relaxed) {
            self.time_info_provider.tick();
        }

        let state = self.state.get();
        match state {
            // Scheduled states, waiting for scheduled cursor
            LooperState::PlayingScheduled => self.after_process_playing_scheduled(&scratch_pad),
            LooperState::RecordingScheduled => self.after_process_recording_scheduled(&scratch_pad),
            // Recording, incrementing the length
            LooperState::Recording => self.after_process_recording(&scratch_pad),
            // Playing states, moving/resetting the cursor
            LooperState::Playing => self.after_process_playing(),
            LooperState::Overdubbing => self.after_process_playing(),
            _ => {}
        }
    }

    /// If we're in `PlayingScheduled` state, wait until the scheduled playback cursor is reached
    /// then stop recording
    ///
    /// Until then, increment the length counter
    fn after_process_playing_scheduled(&self, scratch_pad: &ScratchPad) {
        let current_scratch_cursor = scratch_pad.cursor();
        let scheduled_playback = self.scheduled_playback.get();
        // TODO - this is broken when the cursor wraps around its maximum
        AudioThreadLogger::handle().info("After process playing scheduled");
        if current_scratch_cursor >= scheduled_playback {
            AudioThreadLogger::handle().info("Stopping recording");
            self.stop_recording_audio_thread_only();
        } else {
            self.length.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// If we're in `RecordingScheduled` state, wait until the scheduled record start is reached
    /// then start recording
    fn after_process_recording_scheduled(&self, scratch_pad: &ScratchPad) {
        let current_scratch_cursor = scratch_pad.cursor();
        let scheduled_start = self.start_cursor.load(Ordering::Relaxed);

        // TODO - this is broken when the cursor wraps around its maximum
        if current_scratch_cursor >= scheduled_start {
            self.state.set(LooperState::Recording);
        }
    }

    /// If we're in `Recording` state, increment the length cursor on each tick.
    /// When the recording buffer is exhausted, quit recording.
    fn after_process_recording(&self, scratch_pad: &ScratchPad) {
        let len = self.length.load(Ordering::Relaxed) + 1;
        if len > scratch_pad.max_len() {
            self.stop_recording_audio_thread_only();
        } else {
            self.length.store(len, Ordering::Relaxed);
        }
    }

    // If we're in `Playing` or `Overdubbing` states (the looper is reproducing audio).
    //
    // * Increment the `cursor` by `speed`
    // * If "loop mode" is enabled, reset the cursor to the start of the playback on each tick
    //   - Start of playback depends on the direction the loop is moving to
    //     * If the loop is playing forwards, we'll go to the start offset parameter position
    //     * If the loop is playing backwards, we'll go to the end offset parameter position
    fn after_process_playing(&self) {
        let end_samples = self.get_end_samples();
        if end_samples <= 0.0 {
            return;
        }

        let cursor = self.cursor.get();
        let length = self.length.get() as f32;
        let speed = self.speed.get();

        if !self.loop_enabled.load(Ordering::Relaxed) && (cursor + speed) >= end_samples {
            return;
        }

        let mut cursor = (cursor + speed) % end_samples % length;

        let start_samples = self.get_start_samples();
        let loop_has_finished = cursor < start_samples;
        if loop_has_finished {
            cursor = if speed > 0.0 {
                start_samples
            } else {
                end_samples
            };
        }

        self.cursor.set(cursor);
    }
}

/// Given a fade setting as a percentage of the loop length, return the current volume
fn calculate_fade_volume(fade_perc: f32, length: usize, cursor: f32) -> f32 {
    let fade_samples = fade_perc * length as f32;
    // The volume is the current cursor position within the fade
    // This is linear fade from 0 to 1.0 for the duration of the fade setting.
    (cursor / fade_samples).min(1.0).max(0.0)
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;
    use audio_processor_testing_helpers::assert_f_eq;

    use audio_processor_traits::AudioProcessor;

    use crate::LooperProcessor;

    use super::*;

    mod transitions {
        use audio_processor_testing_helpers::assert_f_eq;

        use audio_garbage_collector::make_shared;
        use audio_processor_traits::{AudioProcessorSettings, VecAudioBuffer};
        use augmented_atomics::AtomicValue;

        use crate::audio::processor::handle::LooperState;
        use crate::QuantizeMode::SnapNext;
        use crate::{
            LooperHandleThread, LooperOptions, LooperProcessorHandle, QuantizeMode,
            TimeInfoProvider, TimeInfoProviderImpl,
        };

        #[test]
        fn test_start_recording_when_empty() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());

            // Move the looper 3 samples forward
            looper.process(0, 0.0);
            looper.after_process();
            looper.process(0, 0.0);
            looper.after_process();
            looper.process(0, 0.0);
            looper.after_process();

            let state = looper.start_recording();
            assert_eq!(state, LooperState::Recording);
            assert_eq!(looper.playhead(), 0, "Playhead is not moving yet");
            assert_eq!(
                looper.start_cursor.get(),
                3,
                "Start cursor is set to current scratch index"
            );
            assert_eq!(looper.length.get(), 0);
        }

        #[test]
        fn test_start_recording_when_empty_with_quantization() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());
            // 1 beat per sec
            looper.set_tempo(60.0);
            looper.quantize_options().set_mode(SnapNext);

            // Move the looper 3 samples forward, we're now past beat 0 and
            // the looper should wait until sample 400 to start recording
            looper.process(0, 0.0);
            looper.after_process();
            looper.process(0, 0.0);
            looper.after_process();
            looper.process(0, 0.0);
            looper.after_process();

            looper.start_recording();
            assert_eq!(looper.state(), LooperState::RecordingScheduled);
            assert_eq!(looper.playhead(), 0, "Playhead is not moving yet");
            assert_eq!(
                looper.start_cursor.get(),
                400,
                "Start cursor is set to sample 400 which isn't set yet"
            );
            assert_eq!(looper.length.get(), 0);
        }

        #[test]
        fn test_start_recording_when_paused() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());
            looper.set_looper_buffer(&VecAudioBuffer::empty_with(1, 100, 0.0));
            assert_eq!(looper.state(), LooperState::Paused);

            looper.start_recording();
            assert_eq!(looper.state(), LooperState::Overdubbing);
        }

        #[test]
        fn test_start_recording_when_playing() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());
            looper.set_looper_buffer(&VecAudioBuffer::empty_with(1, 100, 0.0));
            assert_eq!(looper.state(), LooperState::Paused);
            looper.play();
            assert_eq!(looper.state(), LooperState::Playing);

            looper.start_recording();
            assert_eq!(looper.state(), LooperState::Overdubbing);
        }

        #[test]
        fn test_play_when_empty() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());
            assert_eq!(looper.state(), LooperState::Empty);
            looper.play();
            assert_eq!(looper.state(), LooperState::Playing);
        }

        #[test]
        fn test_pause_when_empty() {
            let looper = LooperProcessorHandle::default();
            looper.prepare(test_settings());
            assert_eq!(looper.state(), LooperState::Empty);
            looper.pause();
            assert_eq!(looper.state(), LooperState::Empty);
        }

        #[test]
        fn test_empty_into_recording() {
            let time_info_provider = make_shared(TimeInfoProviderImpl::new(None));
            let looper =
                LooperProcessorHandle::new(LooperOptions::default(), time_info_provider.clone());

            // 1 beat per second
            time_info_provider.set_tempo(60.0);
            time_info_provider.play();
            looper.prepare(test_settings());
            // Use snap next mode
            looper.quantize_options().set_mode(QuantizeMode::SnapNext);

            let process_beats = |num_beats| {
                for i in 0..(100 * num_beats) {
                    let r = looper.process(0, i as f32);
                    assert_f_eq!(r, 0.0); // Dry is 0 by default
                    looper.after_process();
                }
            };

            assert_eq!(looper.state(), LooperState::Empty);
            // Process 4 beats into the looper
            process_beats(4);

            // Start recording
            looper.start_recording();
            assert_eq!(looper.state(), LooperState::Recording);
            // Process 3 beats into the looper
            process_beats(3);
            looper.stop_recording(LooperHandleThread::OtherThread);
            assert_eq!(looper.state(), LooperState::PlayingScheduled);
            // Process 1 beats into the looper
            process_beats(1);
            assert_eq!(looper.state(), LooperState::Playing);
        }

        fn test_settings() -> AudioProcessorSettings {
            AudioProcessorSettings {
                // 100 samples per second
                sample_rate: 100.0,
                input_channels: 1,
                output_channels: 1,
                ..AudioProcessorSettings::default()
            }
        }
    }

    #[test]
    fn test_get_fade_volumes_when_buffer_is_empty() {
        let handle = LooperHandle::default();
        assert_f_eq!(handle.get_fade_in_volume(0.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(0.0), 0.0);
        assert_f_eq!(handle.get_fade_in_volume(2.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(2.0), 0.0);
    }

    #[test]
    fn test_get_fade_volumes_when_theres_no_fade_set_return_1() {
        let handle = LooperHandle::default();
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        assert_f_eq!(handle.get_fade_in_volume(0.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(0.0), 1.0);
        assert_f_eq!(handle.get_fade_in_volume(2.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(2.0), 1.0);
    }

    #[test]
    fn test_set_buffer_sets_internal_buffer() {
        let handle = LooperHandle::default();
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        let looper_clip_ref = handle.looper_clip();
        let buffer = looper_clip_ref.borrow();
        assert_f_eq!(buffer.get(0, 0).get(), 1.0);
        assert_f_eq!(buffer.get(0, 1).get(), 2.0);
        assert_f_eq!(buffer.get(0, 2).get(), 3.0);
        assert_f_eq!(buffer.get(0, 3).get(), 4.0);
    }

    #[test]
    fn test_set_buffer_sets_internal_buffer_when_handle_is_indirect() {
        let handle = make_shared(LooperHandle::default());
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        let looper_clip_ref = handle.looper_clip();
        let buffer = looper_clip_ref.borrow();
        assert_f_eq!(buffer.get(0, 0).get(), 1.0);
        assert_f_eq!(buffer.get(0, 1).get(), 2.0);
        assert_f_eq!(buffer.get(0, 2).get(), 3.0);
        assert_f_eq!(buffer.get(0, 3).get(), 4.0);
    }

    #[test]
    fn test_play_after_set_buffer() {
        let mut processor = LooperProcessor::default();
        let handle = processor.handle().clone();
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        handle.play();

        let mut buffer = VecAudioBuffer::empty_with(1, 4, 0.0);
        processor.prepare(AudioProcessorSettings {
            output_channels: 1,
            block_size: 4,
            ..Default::default()
        });

        processor.process(&mut buffer);
        assert_eq!(buffer.slice(), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_get_fade_in_volume_when_theres_fade_in() {
        let handle = LooperHandle::default();
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        handle.set_fade_start(0.5);
        assert_f_eq!(handle.get_fade_in_volume(0.0), 0.0);
        assert_f_eq!(handle.get_fade_in_volume(1.0), 0.5);
        assert_f_eq!(handle.get_fade_in_volume(2.0), 1.0);
        assert_f_eq!(handle.get_fade_in_volume(3.0), 1.0);
    }

    #[test]
    fn test_get_fade_out_volume_when_theres_fade_out() {
        let handle = LooperHandle::default();
        let mut buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        handle.set_looper_buffer(&buffer.interleaved());
        handle.set_fade_end(0.5);
        assert_f_eq!(handle.get_fade_out_volume(0.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(1.0), 1.0);
        assert_f_eq!(handle.get_fade_out_volume(2.0), 0.5);
        assert_f_eq!(handle.get_fade_out_volume(3.0), 0.0);
    }

    #[test]
    fn test_stop_recording_does_not_allocate() {
        let handle = LooperHandle::default();
        handle.prepare(AudioProcessorSettings::default());
        handle.start_recording();
        let buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        for sample in buffer.slice() {
            handle.process(0, *sample);
            handle.after_process();
        }
        assert_eq!(handle.state.get(), LooperState::Recording);
        assert_no_alloc(|| {
            handle.stop_recording_audio_thread_only();
        });
    }

    mod get_offset {
        use super::*;

        #[test]
        fn test_get_offset_cursor_without_tempo() {
            let handle = LooperHandle::default();
            handle.prepare(AudioProcessorSettings::default());
            let quantize_options = handle.quantize_options();
            quantize_options.set_mode(quantize_mode::QuantizeMode::SnapNext);

            let offset = handle.get_quantized_offset();
            assert!(offset.is_none());
        }

        #[test]
        fn test_get_offset_cursor_with_tempo_but_disabled_quantize() {
            let handle = LooperHandle::default();
            handle.prepare(AudioProcessorSettings::new(100.0, 1, 1, 512));
            handle.set_tempo(60.0);

            // At the start, offset is 0
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(0));
            handle.process(0, 0.0); // <- we tick one sample
            handle.after_process();
            assert_f_eq!(
                handle.time_info_provider.get_time_info().position_samples(),
                1.0
            );
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(0));
        }

        #[test]
        fn test_get_offset_cursor_with_tempo_snap_next() {
            let handle = LooperHandle::default();
            handle.prepare(AudioProcessorSettings::new(100.0, 1, 1, 512));
            handle.set_tempo(60.0);
            let quantize_options = handle.quantize_options();
            quantize_options.set_mode(quantize_mode::QuantizeMode::SnapNext);

            // At the start, offset is 0
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(0));
            handle.process(0, 0.0); // <- we tick one sample
            handle.after_process();
            assert_f_eq!(
                handle.time_info_provider.get_time_info().position_samples(),
                1.0
            );

            // Now we should snap to the next beat (which is 399 samples ahead)
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(399));
        }

        #[test]
        fn test_get_offset_cursor_with_tempo_snap_closest() {
            let handle = LooperHandle::default();
            handle.prepare(AudioProcessorSettings::new(100.0, 1, 1, 512));
            handle.set_tempo(60.0);
            let quantize_options = handle.quantize_options();
            quantize_options.set_mode(quantize_mode::QuantizeMode::SnapClosest);

            // At the start, offset is 0
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(0));
            handle.process(0, 0.0); // <- we tick one sample
            handle.after_process();
            assert_f_eq!(
                handle.time_info_provider.get_time_info().position_samples(),
                1.0
            );

            // Now we should snap to the closest beat (which is one sample behind)
            let offset = handle.get_quantized_offset();
            assert_eq!(offset, Some(-1));
        }
    }
}
