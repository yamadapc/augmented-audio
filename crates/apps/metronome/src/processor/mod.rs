use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings};
use augmented_adsr_envelope::Envelope;
use augmented_oscillator::Oscillator;
use augmented_playhead::{PlayHead, PlayHeadOptions};

const DEFAULT_CLICK_ATTACK_MS: u64 = 3;
const DEFAULT_CLICK_DECAY_RELEASE_MS: u64 = 10;
const DEFAULT_SAMPLE_RATE: f32 = 44100.0;
const DEFAULT_TEMPO: u32 = 120;

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

    pub fn set_is_playing(&self, value: bool) {
        self.is_playing.store(value, Ordering::Relaxed);
    }

    pub fn set_volume(&self, value: f32) {
        self.volume.set(value);
    }

    pub fn position_beats(&self) -> f32 {
        self.position_beats.get()
    }

    pub fn set_beats_per_bar(&self, value: i32) {
        self.beats_per_bar.store(value, Ordering::Relaxed);
    }
}

/// Holds mutable state for the metronome
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

/// Public methods
impl MetronomeProcessor {
    pub fn new() -> Self {
        let envelope = Self::build_envelope();

        let sample_rate = DEFAULT_SAMPLE_RATE;
        let tempo = DEFAULT_TEMPO;

        MetronomeProcessor {
            handle: make_shared(MetronomeProcessorHandle {
                is_playing: AtomicBool::new(true),
                tempo: AtomicF32::new(tempo as f32),
                volume: AtomicF32::new(1.0),
                position_beats: AtomicF32::new(0.0),
                beats_per_bar: AtomicI32::new(4),
            }),
            state: MetronomeProcessorState {
                last_position: 0.0,
                playhead: PlayHead::new(PlayHeadOptions::new(
                    Some(sample_rate),
                    Some(tempo),
                    Some(16),
                )),
                oscillator: Oscillator::sine(sample_rate),
                playing: false,
                envelope,
            },
        }
    }

    pub fn handle(&self) -> &Shared<MetronomeProcessorHandle> {
        &self.handle
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

/// Private methods
impl MetronomeProcessor {
    fn build_envelope() -> Envelope {
        let mut envelope = Envelope::new();
        envelope.set_attack(Duration::from_millis(DEFAULT_CLICK_ATTACK_MS));
        envelope.set_decay(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
        envelope.set_sustain(0.0);
        envelope.set_release(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
        envelope
    }

    fn process_frame(&mut self, frame: &mut [f32]) {
        self.state.playhead.accept_samples(1);
        self.state.envelope.tick();
        self.state.oscillator.tick();

        let position = self.state.playhead.position_beats() as f32;
        self.handle.position_beats.set(position);

        self.trigger_click(position);

        let out =
            self.state.oscillator.get() * self.handle.volume.get() * self.state.envelope.volume();

        for sample in frame {
            *sample = out;
        }

        self.state.last_position = position;
    }

    /// Triggers the envelope when beat changes and sets the oscillator frequency when on the
    /// accented beat.
    fn trigger_click(&mut self, position: f32) {
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
    }
}
