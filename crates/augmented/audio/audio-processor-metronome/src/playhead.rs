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

use audio_processor_traits::AudioProcessorSettings;
use augmented_playhead::{PlayHead, PlayHeadOptions};

use crate::{DEFAULT_SAMPLE_RATE, DEFAULT_TEMPO};

/// This is so that consumers can control the play-head and metronome just follow
///
/// There are two types of methods here:
///
/// * Mutation methods: reset, set_tempo, prepare
///   - These should be used by the metronome app only and noop if there's another master
/// * Getter methods
///   - These must be implemented
pub trait MetronomePlayhead {
    fn reset(&mut self) {}
    fn set_tempo(&mut self, _tempo: f32) {}
    fn prepare(&mut self, _settings: &AudioProcessorSettings, _tempo: f32) {}
    fn accept_samples(&mut self, _samples: u32) {}
    fn tempo(&self) -> Option<f32> {
        None
    }

    fn position_beats(&self) -> f64;
}

pub struct DefaultMetronomePlayhead {
    playhead: PlayHead,
}

impl Default for DefaultMetronomePlayhead {
    fn default() -> Self {
        let sample_rate = DEFAULT_SAMPLE_RATE;
        let tempo = DEFAULT_TEMPO;
        let playhead = PlayHead::new(PlayHeadOptions::new(
            Some(sample_rate),
            Some(tempo),
            Some(16),
        ));
        Self { playhead }
    }
}

impl MetronomePlayhead for DefaultMetronomePlayhead {
    fn reset(&mut self) {
        self.playhead.set_position_seconds(0.0);
    }

    fn set_tempo(&mut self, tempo: f32) {
        self.playhead.set_tempo(tempo);
    }

    fn prepare(&mut self, settings: &AudioProcessorSettings, tempo: f32) {
        self.playhead = PlayHead::new(PlayHeadOptions::new(
            Some(settings.sample_rate()),
            Some(tempo),
            self.playhead.options().ticks_per_quarter_note(),
        ));
    }

    fn accept_samples(&mut self, samples: u32) {
        self.playhead.accept_samples(samples)
    }

    fn tempo(&self) -> Option<f32> {
        self.playhead.options().tempo()
    }

    fn position_beats(&self) -> f64 {
        self.playhead.position_beats()
    }
}
