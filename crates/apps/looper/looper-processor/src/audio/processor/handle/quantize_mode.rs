use std::sync::atomic::AtomicUsize;

use num::FromPrimitive;
use num::ToPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};

use audio_processor_traits::AtomicF32;
use augmented_atomics::{AtomicEnum, AtomicValue};

use crate::audio::loop_quantization::LoopQuantizerMode;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum QuantizeMode {
    None = 0,
    SnapNext = 1,
    SnapClosest = 2,
}

impl QuantizeMode {
    pub fn cycle(&self) -> QuantizeMode {
        QuantizeMode::from_u32((self.to_u32().unwrap() + 1) % 3).unwrap()
    }
}

pub struct QuantizeOptions {
    mode: AtomicEnum<QuantizeMode>,
    beats: AtomicUsize,
    threshold_ms: AtomicF32,
}

impl Default for QuantizeOptions {
    fn default() -> Self {
        Self {
            mode: AtomicEnum::new(QuantizeMode::None),
            beats: AtomicUsize::new(4),
            threshold_ms: AtomicF32::new(50.0),
        }
    }
}

impl QuantizeOptions {
    pub fn inner(&self) -> LoopQuantizerMode {
        match self.mode.get() {
            QuantizeMode::None => LoopQuantizerMode::None,
            QuantizeMode::SnapNext => LoopQuantizerMode::SnapNext {
                beats: self.beats.get(),
            },
            QuantizeMode::SnapClosest => LoopQuantizerMode::SnapClosest {
                beats: self.beats.get(),
                threshold_ms: self.threshold_ms.get(),
            },
        }
    }

    pub fn mode(&self) -> QuantizeMode {
        self.mode.get()
    }

    pub fn beats(&self) -> usize {
        self.beats.get()
    }

    pub fn threshold_ms(&self) -> f32 {
        self.threshold_ms.get()
    }

    pub fn set_mode(&self, mode: QuantizeMode) {
        self.mode.set(mode);
    }

    pub fn set_beats(&self, beats: usize) {
        self.beats.set(beats);
    }

    pub fn set_threshold_ms(&self, threshold_ms: f32) {
        self.threshold_ms.set(threshold_ms);
    }
}
