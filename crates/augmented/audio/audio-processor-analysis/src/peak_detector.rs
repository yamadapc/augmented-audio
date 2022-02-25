type FloatT = f32;

pub fn calculate_multiplier(sample_rate: FloatT, duration_ms: FloatT) -> FloatT {
    let attack_secs = duration_ms * 0.001;
    let attack_samples = sample_rate * attack_secs;
    FloatT::exp2(-1.0 / attack_samples)
}

pub struct PeakDetector {
    value: FloatT,
}

impl Default for PeakDetector {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl PeakDetector {
    pub fn value(&self) -> FloatT {
        self.value
    }

    pub fn accept_frame(&mut self, attack_mult: FloatT, release_mult: FloatT, frame: &[FloatT]) {
        let frame_len = frame.len() as FloatT;
        let new: FloatT = frame.iter().map(|f| FloatT::abs(*f)).sum::<FloatT>() / frame_len;
        let curr_slope = if self.value > new {
            release_mult
        } else {
            attack_mult
        };
        self.value = (self.value * curr_slope) + ((1.0 - curr_slope) * new);
    }
}
