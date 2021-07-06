use std::time::Duration;

enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

struct StageConfig {
    samples: f32,
    duration: Duration,
}

impl Default for StageConfig {
    fn default() -> Self {
        StageConfig::new(0.0, Duration::from_secs_f32(0.0))
    }
}

impl StageConfig {
    fn new(samples: f32, duration: Duration) -> Self {
        StageConfig { samples, duration }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.samples = samples_for_duration(sample_rate, &self.duration);
    }

    fn set_duration(&mut self, sample_rate: f32, duration: Duration) {
        self.duration = duration;
        self.samples = samples_for_duration(sample_rate, &self.duration);
    }
}

struct EnvelopeConfig {
    attack: StageConfig,
    attack_level: f32,
    decay: StageConfig,
    sustain: f32,
    release: StageConfig,
    sample_rate: f32,
    is_exp: bool,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        EnvelopeConfig {
            attack: StageConfig::new(0.0, Duration::from_secs_f32(0.2)),
            attack_level: 1.0,
            decay: StageConfig::new(0.0, Duration::from_secs_f32(0.3)),
            sustain: 0.8,
            release: StageConfig::new(0.0, Duration::from_secs_f32(0.1)),
            sample_rate: 0.0,
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
    current_samples: f32,
    stage_start_volume: f32,
    current_volume: f32,
}

impl Default for EnvelopeState {
    fn default() -> Self {
        EnvelopeState {
            current_samples: 0.0,
            stage_start_volume: 0.0,
            current_volume: 0.0,
        }
    }
}

pub struct Envelope {
    stage: EnvelopeStage,
    state: EnvelopeState,
    config: EnvelopeConfig,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle,
            state: EnvelopeState::default(),
            config: EnvelopeConfig::default(),
        }
    }

    pub fn exp() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle,
            state: EnvelopeState::default(),
            config: EnvelopeConfig::exp(),
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.config.sample_rate = sample_rate;
        self.config.attack.set_sample_rate(sample_rate);
        self.config.decay.set_sample_rate(sample_rate);
        self.config.release.set_sample_rate(sample_rate);
    }

    pub fn set_attack(&mut self, duration: Duration) {
        self.config
            .attack
            .set_duration(self.config.sample_rate, duration);
    }

    pub fn set_decay(&mut self, duration: Duration) {
        self.config
            .decay
            .set_duration(self.config.sample_rate, duration);
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.config.sustain = sustain;
    }

    pub fn set_release(&mut self, duration: Duration) {
        self.config
            .release
            .set_duration(self.config.sample_rate, duration);
    }

    pub fn volume(&self) -> f32 {
        self.state.current_volume
    }

    pub fn tick(&mut self) {
        self.state.current_samples += 1.0;
        let current_samples = self.state.current_samples;
        let maybe_stage_config = match self.stage {
            EnvelopeStage::Idle => None,
            EnvelopeStage::Attack => {
                self.state.current_volume =
                    self.calculate_volume(self.config.attack_level, self.config.attack.samples);
                Some(&self.config.attack)
            }
            EnvelopeStage::Decay => {
                self.state.current_volume =
                    self.calculate_volume(self.config.sustain, self.config.decay.samples);
                Some(&self.config.decay)
            }
            EnvelopeStage::Sustain => {
                self.state.current_volume = self.config.sustain;
                None
            }
            EnvelopeStage::Release => {
                self.state.current_volume = self.calculate_volume(0.0, self.config.release.samples);
                Some(&self.config.release)
            }
        };

        if let Some(stage_config) = maybe_stage_config {
            if current_samples >= stage_config.samples {
                self.next_stage();
            }
        }
    }

    pub fn note_on(&mut self) {
        self.set_stage(EnvelopeStage::Attack);
    }

    pub fn note_off(&mut self) {
        self.set_stage(EnvelopeStage::Release);
    }

    fn next_stage(&mut self) {
        match self.stage {
            EnvelopeStage::Attack => {
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

    fn set_stage(&mut self, stage: EnvelopeStage) {
        self.state.stage_start_volume = self.state.current_volume;
        self.state.current_samples = 0.0;
        self.stage = stage;
    }

    fn calculate_volume(&self, target: f32, duration_samples: f32) -> f32 {
        let start = self.state.stage_start_volume;
        let current_samples = self.state.current_samples;

        if self.config.is_exp {
            let start = self.state.stage_start_volume;
            let target = target;
            let diff = target - start;
            let perc = (current_samples / duration_samples).powf(if diff >= 0.0 {
                2.0
            } else {
                1.0 / 2.0
            });
            return start + perc * diff;
        }

        let perc = current_samples / duration_samples;
        let diff = target - start;
        start + perc * diff
    }
}

fn samples_for_duration(sample_rate: f32, duration: &Duration) -> f32 {
    sample_rate * duration.as_secs_f32()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use plotters::prelude::*;

    use super::*;

    #[test]
    fn test_adsr_default_envelope() {
        let mut envelope = Envelope::default();
        envelope.set_sample_rate(44100.0);

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, &Duration::from_secs_f32(2.0)) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, &Duration::from_secs_f32(1.0)) as i32) {
            envelope_buffer.push((start + i, envelope.volume()));
            envelope.tick();
        }

        generate_plot(envelope_buffer, "default-envelope")
    }

    #[test]
    fn test_adsr_exp_envelope() {
        let mut envelope = Envelope::exp();
        envelope.set_sample_rate(44100.0);

        let mut envelope_buffer = Vec::new();
        envelope.note_on();
        for i in 0..(samples_for_duration(44100.0, &Duration::from_secs_f32(2.0)) as i32) {
            envelope_buffer.push((i, envelope.volume()));
            envelope.tick();
        }
        envelope.note_off();
        let start = envelope_buffer.len() as i32;
        for i in 0..(samples_for_duration(44100.0, &Duration::from_secs_f32(1.0)) as i32) {
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
