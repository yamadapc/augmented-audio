use augmented_atomics::{AtomicEnum, AtomicF32};
use num_derive::{FromPrimitive, ToPrimitive};
use std::thread::current;
use std::time::Duration;

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

struct StageConfig {
    samples: AtomicF32,
    duration_secs: AtomicF32,
}

impl Default for StageConfig {
    fn default() -> Self {
        StageConfig::new(0.0, Duration::from_secs_f32(0.0))
    }
}

impl StageConfig {
    fn new(samples: f32, duration: Duration) -> Self {
        StageConfig {
            samples: samples.into(),
            duration_secs: duration.as_secs_f32().into(),
        }
    }

    fn set_sample_rate(&self, sample_rate: f32) {
        self.samples
            .set(samples_for_duration(sample_rate, self.duration_secs.get()));
    }

    fn set_duration(&self, sample_rate: f32, duration: Duration) {
        self.duration_secs.set(duration.as_secs_f32());
        self.samples
            .set(samples_for_duration(sample_rate, self.duration_secs.get()));
    }
}

struct EnvelopeConfig {
    attack: StageConfig,
    attack_level: AtomicF32,
    decay: StageConfig,
    sustain: AtomicF32,
    release: StageConfig,
    sample_rate: AtomicF32,
    is_exp: bool,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        EnvelopeConfig {
            attack: StageConfig::new(0.0, Duration::from_secs_f32(0.2)),
            attack_level: 1.0.into(),
            decay: StageConfig::new(0.0, Duration::from_secs_f32(0.3)),
            sustain: 0.8.into(),
            release: StageConfig::new(0.0, Duration::from_secs_f32(0.1)),
            sample_rate: 0.0.into(),
            is_exp: false,
        }
    }
}

impl EnvelopeConfig {
    fn exp() -> Self {
        Self {
            is_exp: true,
            ..Self::default()
        }
    }
}

struct EnvelopeState {
    current_samples: AtomicF32,
    stage_start_volume: AtomicF32,
    current_volume: AtomicF32,
}

impl Default for EnvelopeState {
    fn default() -> Self {
        EnvelopeState {
            current_samples: 0.0.into(),
            stage_start_volume: 0.0.into(),
            current_volume: 0.0.into(),
        }
    }
}

/// An ADSR envelope implementation
pub struct Envelope {
    stage: AtomicEnum<EnvelopeStage>,
    state: EnvelopeState,
    config: EnvelopeConfig,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl Envelope {
    /// Create a linear envelope with default configuration
    pub fn new() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle.into(),
            state: EnvelopeState::default(),
            config: EnvelopeConfig::default(),
        }
    }

    /// Create an exponential envelope with default configuration
    pub fn exp() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle.into(),
            state: EnvelopeState::default(),
            config: EnvelopeConfig::exp(),
        }
    }

    /// Set the envelope sample rate, required before playback
    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.config.sample_rate.set(sample_rate);
        self.config.attack.set_sample_rate(sample_rate);
        self.config.decay.set_sample_rate(sample_rate);
        self.config.release.set_sample_rate(sample_rate);
    }

    /// Set the envelope sample rate, required before playback
    pub fn set_attack(&self, duration: Duration) {
        self.config
            .attack
            .set_duration(self.config.sample_rate.get(), duration);
    }

    /// Set the envelope decay time
    pub fn set_decay(&self, duration: Duration) {
        self.config
            .decay
            .set_duration(self.config.sample_rate.get(), duration);
    }

    /// Set the envelope sustain time
    pub fn set_sustain(&self, sustain: f32) {
        self.config.sustain.set(sustain);
    }

    /// Set the envelope release time
    pub fn set_release(&self, duration: Duration) {
        self.config
            .release
            .set_duration(self.config.sample_rate.get(), duration);
    }

    /// Get the current volume multiplier
    pub fn volume(&self) -> f32 {
        self.update_stage(self.state.current_samples.get(), true);
        self.state.current_volume.get()
    }

    /// Update the envelope, pushing its state forwards by 1 sample
    pub fn tick(&self) {
        let current_samples = self.state.current_samples.get() + 1.0;
        self.state.current_samples.set(current_samples);
        self.update_stage(current_samples, false);
    }

    fn update_stage(&self, current_samples: f32, recurse: bool) {
        // println!("update_stage(current_samples={})", current_samples);
        let maybe_stage_config =
            match self.stage.get() {
                EnvelopeStage::Idle => None,
                EnvelopeStage::Attack => {
                    self.state.current_volume.set(self.calculate_volume(
                        self.config.attack_level.get(),
                        self.config.attack.samples.get(),
                    ));
                    Some(&self.config.attack)
                }
                EnvelopeStage::Decay => {
                    self.state.current_volume.set(self.calculate_volume(
                        self.config.sustain.get(),
                        self.config.decay.samples.get(),
                    ));
                    Some(&self.config.decay)
                }
                EnvelopeStage::Sustain => {
                    self.state.current_volume.set(self.config.sustain.get());
                    None
                }
                EnvelopeStage::Release => {
                    self.state
                        .current_volume
                        .set(self.calculate_volume(0.0, self.config.release.samples.get()));
                    Some(&self.config.release)
                }
            };

        if let Some(stage_config) = maybe_stage_config {
            if current_samples >= stage_config.samples.get() {
                self.next_stage();

                // Purpose is to handle 0 value envelopes
                if recurse {
                    self.update_stage(current_samples, true);
                }
            }
        }
    }

    /// Trigger the envelope by setting its stage to the Attack phase. Does not change the current
    /// volume, only the stage.
    pub fn note_on(&self) {
        self.set_stage(EnvelopeStage::Attack);
    }

    /// Set the envelope stage to release.
    pub fn note_off(&self) {
        self.set_stage(EnvelopeStage::Release);
    }

    fn next_stage(&self) {
        match self.stage.get() {
            EnvelopeStage::Attack => {
                self.state
                    .current_volume
                    .set(self.config.attack_level.get());
                self.set_stage(EnvelopeStage::Decay);
            }
            EnvelopeStage::Decay => {
                self.set_stage(EnvelopeStage::Sustain);
            }
            EnvelopeStage::Sustain => {
                self.set_stage(EnvelopeStage::Release);
            }
            EnvelopeStage::Release => {
                self.set_stage(EnvelopeStage::Idle);
            }
            EnvelopeStage::Idle => {}
        }
    }

    fn set_stage(&self, stage: EnvelopeStage) {
        self.state
            .stage_start_volume
            .set(self.state.current_volume.get());
        self.state.current_samples.set(0.0);
        self.stage.set(stage);
    }

    fn calculate_volume(&self, target: f32, duration_samples: f32) -> f32 {
        let start = self.state.stage_start_volume.get();
        let current_samples = self.state.current_samples.get();

        if self.config.is_exp {
            let current_volume = self.state.current_volume.get();
            let a = std::f32::consts::E.powf(-1.0 / (duration_samples.max(f32::EPSILON) * 0.3));
            return a * current_volume + (1.0 - a) * target;
        }

        let perc = current_samples / duration_samples.max(f32::EPSILON);
        let diff = target - start;
        start + perc * diff
    }
}

fn samples_for_duration(sample_rate: f32, duration_secs: f32) -> f32 {
    sample_rate * duration_secs
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use plotters::prelude::*;

    use super::*;

    #[test]
    fn test_0_attack_envelope_with_decay() {
        let mut envelope = Envelope::default();
        envelope.set_sample_rate(44100.0);

        envelope.set_attack(Duration::from_secs_f32(0.0));
        envelope.set_decay(Duration::from_secs_f32(0.2));

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, 2.0) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, 1.0) as i32) {
            envelope_buffer.push((start + i, envelope.volume()));
            envelope.tick();
        }

        generate_plot(envelope_buffer, "zero-attack-envelope-with-decay")
    }

    #[test]
    fn test_0_attack_envelope() {
        let mut envelope = Envelope::default();
        envelope.set_sample_rate(44100.0);

        envelope.set_attack(Duration::from_secs_f32(0.0));
        envelope.set_decay(Duration::from_secs_f32(0.0));

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, 2.0) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, 1.0) as i32) {
            envelope_buffer.push((start + i, envelope.volume()));
            envelope.tick();
        }

        generate_plot(envelope_buffer, "zero-attack-envelope")
    }

    #[test]
    fn test_adsr_default_envelope() {
        let mut envelope = Envelope::default();
        envelope.set_sample_rate(44100.0);
        envelope.set_attack(Duration::from_secs_f32(0.3));
        envelope.set_release(Duration::from_secs_f32(0.3));
        envelope.set_decay(Duration::from_secs_f32(0.3));

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, 2.0) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, 1.0) as i32) {
            envelope_buffer.push((start + i, envelope.volume()));
            envelope.tick();
        }

        generate_plot(envelope_buffer, "default-envelope")
    }

    #[test]
    fn test_adsr_exp_envelope() {
        let mut envelope = Envelope::exp();
        envelope.set_sample_rate(44100.0);
        envelope.set_attack(Duration::from_secs_f32(0.3));
        envelope.set_release(Duration::from_secs_f32(0.3));
        envelope.set_decay(Duration::from_secs_f32(0.3));

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, 2.0) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, 1.0) as i32) {
            envelope_buffer.push((start + i, envelope.volume()));
            envelope.tick();
        }

        generate_plot(envelope_buffer, "exp-envelope")
    }

    fn generate_plot(output: Vec<(i32, f32)>, plot_name: &str) {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let filename = root_dir.join(format!("src/__plots__/{}.png", plot_name));
        let plot_filename = Path::new(&filename);

        let backend = BitMapBackend::new(plot_filename, (1000, 1000));
        let drawing_area = backend.into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(plot_name, ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0.0..output.len() as f64, 0.0..1.2)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                output.iter().map(|(x, y)| (*x as f64, *y as f64)),
                &RED,
            ))
            .unwrap();
        drawing_area.present().unwrap();
    }
}
