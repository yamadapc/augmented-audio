use std::sync::atomic::{AtomicUsize, Ordering};

use basedrop::{Shared, SharedCell};
use num_derive::{FromPrimitive, ToPrimitive};

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_traits::{
    AtomicF32, AudioBuffer, AudioProcessorSettings, OwnedAudioBuffer, VecAudioBuffer,
};

use crate::loop_quantization::{LoopQuantizer, LoopQuantizerMode, QuantizeInput};
use crate::time_info_provider::TimeInfoProvider;
use crate::util::atomic_enum::AtomicEnum;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum RecordingState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
}

impl RecordingState {
    pub fn is_empty(&self) -> bool {
        matches!(self, RecordingState::Empty)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum LooperQuantizationModeType {
    None = 0,
    SnapNext = 1,
    SnapClosest = 2,
}

pub(crate) struct LoopState {
    pub(crate) recording_state: AtomicEnum<RecordingState>,
    pub(crate) start: AtomicUsize,
    pub(crate) end: AtomicUsize,
}

pub(crate) struct LooperProcessorState {
    pub(crate) loop_state: LoopState,
    pub(crate) looper_cursor: AtomicF32,
    pub(crate) looper_increment: AtomicF32,
    pub(crate) num_channels: AtomicUsize,
    pub(crate) looped_clip: Shared<SharedCell<VecAudioBuffer<AtomicF32>>>,
    pub(crate) quantization_mode: AtomicEnum<LooperQuantizationModeType>,
}

impl LooperProcessorState {
    pub(crate) fn new() -> Self {
        LooperProcessorState {
            num_channels: AtomicUsize::new(2usize),
            looper_cursor: AtomicF32::new(0.0),
            looper_increment: AtomicF32::new(1.0),
            loop_state: LoopState {
                recording_state: AtomicEnum::new(RecordingState::Empty),
                start: AtomicUsize::new(0),
                end: AtomicUsize::new(0),
            },
            looped_clip: make_shared(make_shared_cell(VecAudioBuffer::new())),
            quantization_mode: AtomicEnum::new(LooperQuantizationModeType::SnapClosest),
        }
    }

    pub(crate) fn increment_cursor(&self) {
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

    pub(crate) fn clear(&self) {
        self.loop_state.recording_state.set(RecordingState::Empty);
        self.looper_cursor.set(0.0);
        self.loop_state.start.store(0, Ordering::Relaxed);
        self.loop_state.end.store(0, Ordering::Relaxed);
        for sample in self.looped_clip.get().slice() {
            sample.set(0.0);
        }
    }

    pub(crate) fn on_tick(
        &self,
        settings: &AudioProcessorSettings,
        time_info_provider: &impl TimeInfoProvider,
        is_recording: bool,
        looper_cursor: usize,
    ) {
        match self.loop_state.recording_state.get() {
            // Loop has ended
            RecordingState::Recording if !is_recording => {
                let quantized_cursor =
                    self.get_quantized_cursor(settings, time_info_provider, looper_cursor);
                if looper_cursor >= quantized_cursor {
                    self.loop_state.recording_state.set(RecordingState::Playing);
                }
                self.loop_state
                    .end
                    .store(quantized_cursor, Ordering::Relaxed);
            }
            // Loop has started
            RecordingState::Empty if is_recording => {
                let quantized_cursor =
                    self.get_quantized_cursor(settings, time_info_provider, looper_cursor);
                if looper_cursor >= quantized_cursor {
                    self.loop_state
                        .recording_state
                        .set(RecordingState::Recording);
                }
                self.loop_state
                    .start
                    .store(quantized_cursor, Ordering::Relaxed);
            }
            _ => {}
        }

        self.increment_cursor();
    }

    fn get_quantized_cursor(
        &self,
        settings: &AudioProcessorSettings,
        time_info_provider: &impl TimeInfoProvider,
        looper_cursor: usize,
    ) -> usize {
        let num_samples = self.looped_clip.get().num_samples();

        Self::build_quantized_cursor(
            self.quantization_mode.get(),
            settings,
            time_info_provider,
            num_samples,
            looper_cursor,
        )
    }

    fn build_quantized_cursor(
        quantization_mode: LooperQuantizationModeType,
        settings: &AudioProcessorSettings,
        time_info_provider: &impl TimeInfoProvider,
        num_samples: usize,
        looper_cursor: usize,
    ) -> usize {
        let quantizer = LoopQuantizer::new(match quantization_mode {
            LooperQuantizationModeType::None => LoopQuantizerMode::None,
            LooperQuantizationModeType::SnapNext => LoopQuantizerMode::SnapNext { beats: 4 },
            LooperQuantizationModeType::SnapClosest => LoopQuantizerMode::SnapClosest {
                beats: 4,
                threshold_ms: 100.0,
            },
        });
        let time_info = time_info_provider.get_time_info();

        if let Some((tempo, position_beats)) = time_info.tempo.zip(time_info.position_beats) {
            let result = quantizer.quantize(QuantizeInput {
                tempo: tempo as f32,
                sample_rate: settings.sample_rate,
                position_beats: position_beats as f32,
                position_samples: looper_cursor,
            });

            // TODO: Clean-up this mess
            if result < 0 {
                (num_samples + result.abs() as usize) % num_samples
            } else {
                (result % num_samples as i32) as usize
            }
        } else {
            looper_cursor
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
}

// Public API
impl LooperProcessorState {
    /// Returns the size of the current loop
    pub fn num_samples(&self) -> usize {
        let recording_state = self.loop_state.recording_state.get();

        if recording_state == RecordingState::Empty {
            return 0;
        }

        let clip = self.looped_clip.get();
        let start = self.loop_state.start.load(Ordering::Relaxed);

        // let cursor = self.looper_cursor.get();
        // if cursor < start as f32 {
        //     return 0;
        // }

        let end = self.end_cursor();

        if end >= start {
            end - start
        } else {
            clip.num_samples() - start + end
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
}

#[cfg(test)]
mod test {
    use num::FromPrimitive;

    use crate::time_info_provider::{MockTimeInfoProvider, TimeInfo};

    use super::*;

    #[test]
    fn test_get_quantized_cursor_zero() {
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 1000.0;
        let mut time_info_provider = MockTimeInfoProvider::new();
        time_info_provider
            .expect_get_time_info()
            .returning(|| TimeInfo {
                tempo: Some(60.0),     // 1 beat per second, 1000 samples per beat
                position_samples: 0.0, // we're at beat 0
                position_beats: Some(0.0),
            });

        let loop_len = 10_000;
        let looper_cursor = 0;

        let result = LooperProcessorState::build_quantized_cursor(
            LooperQuantizationModeType::SnapClosest,
            &settings,
            &time_info_provider,
            loop_len,
            looper_cursor,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_quantized_cursor_next_bar() {
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 1000.0;
        let mut time_info_provider = MockTimeInfoProvider::new();
        time_info_provider
            .expect_get_time_info()
            .returning(|| TimeInfo {
                tempo: Some(60.0),        // 1 beat per second, 1000 samples per beat
                position_samples: 1000.0, // we're at beat 1
                position_beats: Some(1.0),
            });

        let loop_len = 10_000;
        let looper_cursor = 1000;

        let result = LooperProcessorState::build_quantized_cursor(
            LooperQuantizationModeType::SnapClosest,
            &settings,
            &time_info_provider,
            loop_len,
            looper_cursor,
        );
        assert_eq!(result, 4000);
    }

    #[test]
    fn test_get_quantized_cursor_overflowing() {
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 1000.0;
        let mut time_info_provider = MockTimeInfoProvider::new();
        time_info_provider
            .expect_get_time_info()
            .returning(|| TimeInfo {
                tempo: Some(60.0),        // 1 beat per second, 1000 samples per beat
                position_samples: 9000.0, // we're at beat 1
                position_beats: Some(1.0),
            });

        let loop_len = 10_000;
        let looper_cursor = 9000;

        let result = LooperProcessorState::build_quantized_cursor(
            LooperQuantizationModeType::SnapClosest,
            &settings,
            &time_info_provider,
            loop_len,
            looper_cursor,
        );
        assert_eq!(result, 2000);
    }

    #[test]
    fn test_from_atomic() {
        let state = 0;
        let state = RecordingState::from_usize(state).unwrap();
        assert_eq!(state, RecordingState::Empty);
    }
}
