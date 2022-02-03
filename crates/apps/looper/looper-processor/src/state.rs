use std::sync::atomic::{AtomicUsize, Ordering};

use basedrop::{Shared, SharedCell};
use num_derive::{FromPrimitive, ToPrimitive};

use audio_garbage_collector::{make_shared, make_shared_cell};
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AtomicF32, AudioBuffer, VecAudioBuffer};

use crate::{AtomicEnum, AtomicFloat};

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub(crate) enum RecordingState {
    Empty = 0,
    Recording = 1,
    Playing = 2,
}

pub(crate) struct LoopState {
    pub(crate) recording_state: AtomicEnum<RecordingState>,
    pub(crate) start: AtomicUsize,
    pub(crate) end: AtomicUsize,
}

pub struct LooperProcessorState {
    pub(crate) loop_state: LoopState,
    pub(crate) looper_cursor: AtomicFloat,
    pub(crate) looper_increment: AtomicFloat,
    pub(crate) num_channels: AtomicUsize,
    pub(crate) looped_clip: Shared<SharedCell<VecAudioBuffer<AtomicF32>>>,
}

impl LooperProcessorState {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn on_tick(&self, is_recording: bool, looper_cursor: usize) {
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

    use super::*;

    #[test]
    fn test_from_atomic() {
        let state = 0;
        let state = RecordingState::from_usize(state).unwrap();
        assert_eq!(state, RecordingState::Empty);
    }
}
