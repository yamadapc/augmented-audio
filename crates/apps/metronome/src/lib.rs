use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings};
use augmented_adsr_envelope::Envelope;
use augmented_oscillator::Oscillator;
use augmented_playhead::{PlayHead, PlayHeadOptions};
pub use bridge_generated::*;

mod api;
mod bridge_generated;

pub struct MetronomeProcessorHandle {
    is_playing: AtomicBool,
    tempo: AtomicF32,
    volume: AtomicF32,
    position_beats: AtomicF32,
    beats_per_bar: AtomicI32,
}

struct MetronomeProcessorState {
    playhead: PlayHead,
    oscillator: Oscillator<f32>,
    playing: bool,
    envelope: Envelope,
    last_position: f32,
}

pub struct MetronomeProcessor {
    state: MetronomeProcessorState,
    handle: Shared<MetronomeProcessorHandle>,
}

impl Default for MetronomeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl MetronomeProcessor {
    pub fn new() -> Self {
        let mut envelope = Envelope::new();
        envelope.set_attack(Duration::from_millis(3));
        envelope.set_decay(Duration::from_millis(10));
        envelope.set_sustain(0.0);
        envelope.set_release(Duration::from_millis(10));

        MetronomeProcessor {
            handle: make_shared(MetronomeProcessorHandle {
                is_playing: AtomicBool::new(true),
                tempo: AtomicF32::new(120.0),
                volume: AtomicF32::new(1.0),
                position_beats: AtomicF32::new(0.0),
                beats_per_bar: AtomicI32::new(4),
            }),
            state: MetronomeProcessorState {
                last_position: 0.0,
                playhead: PlayHead::new(PlayHeadOptions::new(Some(44100.0), Some(16), Some(120))),
                oscillator: Oscillator::new_with_sample_rate(
                    44100.0,
                    augmented_oscillator::generators::sine_generator,
                ),
                playing: false,
                envelope,
            },
        }
    }

    fn process_frame(&mut self, frame: &mut [f32]) {
        self.state.playhead.accept_samples(1);
        self.state.envelope.tick();
        self.state.oscillator.tick();

        let position = self.state.playhead.position_beats() as f32;
        self.handle.position_beats.set(position);

        if !self.state.playing {
            let beats_per_bar = self.handle.beats_per_bar.load(Ordering::Relaxed);
            let f_beats_per_bar = beats_per_bar as f32;

            if beats_per_bar != 1 && position % f_beats_per_bar < 1.0 {
                self.state.oscillator.set_frequency(880.0);
            } else {
                self.state.oscillator.set_frequency(440.0);
            }
        }

        if !self.state.playing && position.floor() != self.state.last_position.floor() {
            self.state.playing = true;
            self.state.envelope.note_on();
        } else {
            self.state.playing = false;
        }

        let out =
            self.state.oscillator.get() * self.handle.volume.get() * self.state.envelope.volume();

        for sample in frame {
            *sample = out;
        }

        self.state.last_position = position;
    }
}

impl AudioProcessor for MetronomeProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.state.playhead = PlayHead::new(PlayHeadOptions::new(
            Some(settings.sample_rate()),
            Some(self.handle.tempo.get() as u32),
            self.state.playhead.options().ticks_per_quarter_note(),
        ));
        self.state.envelope.set_sample_rate(settings.sample_rate());
        self.state
            .oscillator
            .set_sample_rate(settings.sample_rate())
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if !self.handle.is_playing.load(Ordering::Relaxed) {
            self.state.playhead.set_position_seconds(0.0);
            self.handle.position_beats.set(0.0);

            for sample in data.slice_mut() {
                *sample = 0.0;
            }
            return;
        }

        let tempo = self.handle.tempo.get() as u32;
        if Some(tempo) != self.state.playhead.options().tempo() {
            self.state.playhead.set_tempo(tempo);
        }

        for frame in data.frames_mut() {
            self.process_frame(frame);
        }
    }
}
