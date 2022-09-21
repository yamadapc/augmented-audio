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

//! Implements a simple metronome [`audio_processor_traits::AudioProcessor`].
//!
//! This is the metronome sound in:
//!
//! * [Simple Metronome](https://beijaflor.io/blog/01-2022/rust-audio-experiments-3/)
//! * [Continuous Looper](https://beijaflor.io/blog/04-2022/rust-audio-experiments-5/)

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings};

use self::constants::{build_envelope, DEFAULT_SAMPLE_RATE, DEFAULT_TEMPO};
pub use self::playhead::{DefaultMetronomePlayhead, MetronomePlayhead};
use self::sound::MetronomeSoundSine;
pub use self::sound::{MetronomeSound, MetronomeSoundAny, MetronomeSoundType};

mod constants;
mod playhead;
mod sound;

/// Public metronome API
pub struct MetronomeProcessorHandle {
    is_playing: AtomicBool,
    tempo: AtomicF32,
    volume: AtomicF32,
    position_beats: AtomicF32,
    beats_per_bar: AtomicI32,
}

impl MetronomeProcessorHandle {
    pub fn set_tempo(&self, value: f32) {
        self.tempo.set(value);
    }

    pub fn tempo(&self) -> f32 {
        self.tempo.get()
    }

    pub fn set_is_playing(&self, value: bool) {
        self.is_playing.store(value, Ordering::Relaxed);
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn set_volume(&self, value: f32) {
        self.volume.set(value);
    }

    pub fn volume(&self) -> f32 {
        self.volume.get()
    }

    pub fn position_beats(&self) -> f32 {
        self.position_beats.get()
    }

    pub fn beats_per_bar(&self) -> i32 {
        self.beats_per_bar.load(Ordering::Relaxed)
    }

    pub fn set_beats_per_bar(&self, value: i32) {
        self.beats_per_bar.store(value, Ordering::Relaxed);
    }
}

/// Holds mutable state for the metronome
struct MetronomeProcessorState {
    is_beeping: bool,
    metronome_sound: MetronomeSoundType,
    last_position: f32,
}

impl Default for MetronomeProcessorState {
    fn default() -> Self {
        Self {
            last_position: -1.0,
            is_beeping: false,
            metronome_sound: MetronomeSoundType::Sine(MetronomeSoundSine::new(
                DEFAULT_SAMPLE_RATE,
                build_envelope(),
            )),
        }
    }
}

pub struct MetronomeProcessor<P: MetronomePlayhead> {
    state: MetronomeProcessorState,
    handle: Shared<MetronomeProcessorHandle>,
    playhead: P,
}

impl Default for MetronomeProcessor<DefaultMetronomePlayhead> {
    fn default() -> Self {
        Self::new(DefaultMetronomePlayhead::default())
    }
}

/// Public methods
impl<P: MetronomePlayhead> MetronomeProcessor<P> {
    pub fn new(playhead: P) -> Self {
        MetronomeProcessor {
            handle: make_shared(MetronomeProcessorHandle {
                is_playing: AtomicBool::new(true),
                tempo: AtomicF32::new(DEFAULT_TEMPO as f32),
                volume: AtomicF32::new(1.0),
                position_beats: AtomicF32::new(0.0),
                beats_per_bar: AtomicI32::new(4),
            }),
            state: MetronomeProcessorState::default(),
            playhead,
        }
    }

    pub fn from_handle(playhead: P, handle: Shared<MetronomeProcessorHandle>) -> Self {
        Self {
            handle,
            state: MetronomeProcessorState::default(),
            playhead,
        }
    }

    pub fn set_sound(&mut self, sound: MetronomeSoundType) {
        self.state.metronome_sound = sound;
    }

    pub fn handle(&self) -> &Shared<MetronomeProcessorHandle> {
        &self.handle
    }
}

impl<P: MetronomePlayhead> AudioProcessor for MetronomeProcessor<P> {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.playhead.prepare(&settings, self.handle.tempo.get());
        self.state.metronome_sound.prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if !self.handle.is_playing.load(Ordering::Relaxed) {
            self.playhead.reset();
            self.handle.position_beats.set(0.0);

            for sample in data.slice_mut() {
                *sample = 0.0;
            }
            return;
        }

        self.playhead.set_tempo(self.handle.tempo.get());

        for frame in data.frames_mut() {
            self.process_frame(frame);
        }
    }
}

/// Private methods
impl<P: MetronomePlayhead> MetronomeProcessor<P> {
    fn process_frame(&mut self, frame: &mut [f32]) {
        self.playhead.accept_samples(1);

        let position = self.playhead.position_beats() as f32;
        self.handle.position_beats.set(position);

        self.trigger_click(position);

        let out = self.state.metronome_sound.process() * self.handle.volume.get();

        for sample in frame {
            *sample = out;
        }

        self.state.last_position = position;
    }

    /// Triggers the envelope when beat changes and sets the oscillator frequency when on the
    /// accented beat.
    fn trigger_click(&mut self, position: f32) {
        if !self.state.is_beeping {
            let beats_per_bar = self.handle.beats_per_bar.load(Ordering::Relaxed);
            let f_beats_per_bar = beats_per_bar as f32;

            let is_beat_1 = beats_per_bar != 1 && position % f_beats_per_bar < 1.0;
            self.state.metronome_sound.set_accent_beat(is_beat_1);
        }

        if !self.state.is_beeping && position.floor() != self.state.last_position.floor() {
            self.state.is_beeping = true;
            self.state.metronome_sound.trigger();
        } else {
            self.state.is_beeping = false;
        }
    }
}
