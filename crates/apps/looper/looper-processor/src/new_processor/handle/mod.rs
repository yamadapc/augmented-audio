use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use atomic_refcell::AtomicRefCell;
use basedrop::{Shared, SharedCell};
use num_derive::{FromPrimitive, ToPrimitive};

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_standalone::standalone_vst::vst::plugin::HostCallback;
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::AtomicEnum;
pub use quantize_mode::{QuantizeMode, QuantizeOptions};
use utils::CopyLoopClipParams;

use crate::loop_quantization::{LoopQuantizer, QuantizeInput};
use crate::time_info_provider::{TimeInfoProvider, TimeInfoProviderImpl};

mod quantize_mode;
mod scratch_pad;
mod utils;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum LooperState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
    Paused = 3,
    Overdubbing = 4,

    RecordingScheduled = 5,
}

pub struct LooperOptions {
    pub max_loop_length: Duration,
    pub host_callback: Option<HostCallback>,
}

impl Default for LooperOptions {
    fn default() -> Self {
        Self {
            max_loop_length: Duration::from_secs(10),
            host_callback: None,
        }
    }
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
    /// Provides time information
    time_info_provider: TimeInfoProviderImpl,
    options: LooperOptions,

    settings: SharedCell<AudioProcessorSettings>,
    quantize_options: QuantizeOptions,
}

impl Default for LooperHandle {
    fn default() -> Self {
        Self::from_options(Default::default())
    }
}

impl LooperHandle {
    pub fn from_options(options: LooperOptions) -> Self {
        let time_info_provider = TimeInfoProviderImpl::new(options.host_callback);
        Self {
            dry_volume: AtomicF32::new(0.0),
            wet_volume: AtomicF32::new(1.0),

            state: AtomicEnum::new(LooperState::Empty),
            start_cursor: AtomicUsize::new(0),
            length: AtomicUsize::new(0),
            scratch_pad: make_shared_cell(scratch_pad::ScratchPad::new(VecAudioBuffer::new())),
            looper_clip: make_shared_cell(AtomicRefCell::new(VecAudioBuffer::new())),
            cursor: AtomicUsize::new(0),
            time_info_provider,
            options,
            settings: make_shared_cell(Default::default()),
            quantize_options: QuantizeOptions::default(),
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
            let scratch_pad = self.scratch_pad.get();
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
            self.state.set(LooperState::Overdubbing);
        }

        self.state.get()
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
        self.state.set(LooperState::Playing);
    }

    pub fn pause(&self) {
        self.state.set(LooperState::Paused);
    }

    pub fn stop_recording_allocating_loop(&self) {
        let old_state = self.state.get();
        if old_state == LooperState::Recording {
            let scratch_pad = self.scratch_pad.get();

            let _result_buffer = self.looper_clip.get();
            let mut new_buffer = VecAudioBuffer::new();
            utils::copy_looped_clip(
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
                utils::copy_looped_clip(
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

    pub fn quantize_options(&self) -> &QuantizeOptions {
        &self.quantize_options
    }

    pub fn time_info_provider(&self) -> &TimeInfoProviderImpl {
        &self.time_info_provider
    }

    pub fn set_tempo(&self, tempo: u32) {
        self.time_info_provider.set_tempo(tempo);
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
        let scratch_pad = scratch_pad::ScratchPad::new(utils::empty_buffer(
            num_channels,
            max_loop_length_samples,
        ));
        self.scratch_pad.set(make_shared(scratch_pad));

        // Pre-allocate looper clip
        let looper_clip = utils::empty_buffer(num_channels, max_loop_length_samples);
        self.looper_clip
            .set(make_shared(AtomicRefCell::new(looper_clip)));

        self.time_info_provider
            .set_sample_rate(settings.sample_rate());

        self.settings.set(make_shared(settings));
    }

    pub(crate) fn state(&self) -> LooperState {
        self.state.get()
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
        self.time_info_provider.tick();

        let state = self.state.get();
        if state == LooperState::RecordingScheduled {
            let current_scratch_cursor = scratch_pad.cursor();
            let scheduled_start = self.start_cursor.load(Ordering::Relaxed);
            if current_scratch_cursor >= scheduled_start {
                self.state.set(LooperState::Recording);
            }
        } else if state == LooperState::Recording {
            let len = self.length.load(Ordering::Relaxed) + 1;
            if len > scratch_pad.max_len() {
                self.stop_recording_audio_thread_only();
            } else {
                self.length.store(len, Ordering::Relaxed);
            }
        } else if state == LooperState::Playing || state == LooperState::Overdubbing {
            let cursor = self.cursor.load(Ordering::Relaxed);
            let cursor = (cursor + 1) % self.length.load(Ordering::Relaxed);
            self.cursor.store(cursor, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

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
            handle.set_tempo(60);

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
            handle.set_tempo(60);
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
            handle.set_tempo(60);
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
