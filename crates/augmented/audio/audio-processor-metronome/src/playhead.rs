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

use crate::constants::{DEFAULT_SAMPLE_RATE, DEFAULT_TEMPO};

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

impl DefaultMetronomePlayhead {
    fn playhead(&self) -> &PlayHead {
        &self.playhead
    }
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

#[cfg(test)]
mod test {
    use super::{DefaultMetronomePlayhead, MetronomePlayhead};
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::AudioProcessorSettings;

    #[test]
    fn test_default_metronome_playhead_has_expected_values() {
        let playhead = DefaultMetronomePlayhead::default();
        assert_f_eq!(playhead.position_beats(), 0.0);
        assert_eq!(playhead.tempo(), Some(120.0));
    }

    #[test]
    fn test_reset_sets_position_to_zero() {
        let mut playhead = DefaultMetronomePlayhead::default();
        playhead.accept_samples(100);
        playhead.reset();
        assert_f_eq!(playhead.position_beats(), 0.0);
    }

    #[test]
    fn test_set_tempo_changes_tempo() {
        let mut playhead = DefaultMetronomePlayhead::default();
        playhead.set_tempo(80.0);
        assert_eq!(playhead.tempo(), Some(80.0));
    }

    #[test]
    fn test_prepare_sets_sample_rate_and_tempo() {
        let mut playhead = DefaultMetronomePlayhead::default();
        let settings = AudioProcessorSettings::new(22050.0, 2, 2, 512);
        playhead.prepare(&settings, 100.0);

        assert_eq!(playhead.tempo(), Some(100.0));
        let sample_rate = playhead.playhead().options().sample_rate().unwrap();
        assert_f_eq!(sample_rate, 22050.0);
    }

    #[test]
    fn test_accept_samples_updates_position() {
        let mut playhead = DefaultMetronomePlayhead::default();
        let settings = AudioProcessorSettings::new(44100.0, 2, 2, 512);
        playhead.prepare(&settings, 120.0);
        playhead.accept_samples(4410);
        assert_f_eq!(playhead.position_beats(), 0.2);
    }
}
