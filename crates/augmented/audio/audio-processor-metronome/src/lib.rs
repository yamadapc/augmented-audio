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
const DEFAULT_TEMPO: f32 = 120.0;

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

/// This is so that consumers can control the playhead and metronome just follow
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

/// Holds mutable state for the metronome
struct MetronomeProcessorState {
    // playhead: PlayHead,
    oscillator: Oscillator<f32>,
    is_beeping: bool,
    envelope: Envelope,
    last_position: f32,
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
                oscillator: Oscillator::sine(sample_rate),
                is_beeping: false,
                envelope,
            },
            playhead,
        }
    }

    pub fn handle(&self) -> &Shared<MetronomeProcessorHandle> {
        &self.handle
    }
}

impl<P: MetronomePlayhead> AudioProcessor for MetronomeProcessor<P> {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.playhead.prepare(&settings, self.handle.tempo.get());
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
    fn build_envelope() -> Envelope {
        let envelope = Envelope::new();
        envelope.set_attack(Duration::from_millis(DEFAULT_CLICK_ATTACK_MS));
        envelope.set_decay(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
        envelope.set_sustain(0.0);
        envelope.set_release(Duration::from_millis(DEFAULT_CLICK_DECAY_RELEASE_MS));
        envelope
    }

    fn process_frame(&mut self, frame: &mut [f32]) {
        self.playhead.accept_samples(1);
        self.state.envelope.tick();
        self.state.oscillator.tick();

        let position = self.playhead.position_beats() as f32;
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
        if !self.state.is_beeping {
            let beats_per_bar = self.handle.beats_per_bar.load(Ordering::Relaxed);
            let f_beats_per_bar = beats_per_bar as f32;

            if beats_per_bar != 1 && position % f_beats_per_bar < 1.0 {
                self.state.oscillator.set_frequency(880.0);
            } else {
                self.state.oscillator.set_frequency(440.0);
            }
        }

        if !self.state.is_beeping && position.floor() != self.state.last_position.floor() {
            self.state.is_beeping = true;
            self.state.envelope.note_on();
        } else {
            self.state.is_beeping = false;
        }
    }
}
