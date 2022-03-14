pub struct TimeSignature {
    beats_per_bar: usize,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self { beats_per_bar: 4 }
    }
}

pub fn estimate_tempo(
    time_signature: TimeSignature,
    sample_rate: f32,
    length_samples: usize,
) -> f32 {
    let beats_per_bar = time_signature.beats_per_bar as f32;
    let length_secs = (length_samples as f32) / sample_rate;

    let mut tempo_candidate = 0.0;
    for num_bars in 1..100 {
        let num_bars = 100 - num_bars;
        let num_bars = num_bars as f32;
        let secs_per_bar = length_secs / num_bars;
        let secs_per_beat = secs_per_bar / beats_per_bar;

        tempo_candidate = (1.0 / secs_per_beat) * 60.0;
        if tempo_candidate >= 80.0 && tempo_candidate <= 160.0 {
            return tempo_candidate;
        }
    }

    tempo_candidate
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tempo_estimation() {
        let result = estimate_tempo(
            Default::default(),
            44100.0,
            (44100.0 * 1.0 / (120.0 / 60.0) * 4.0) as usize,
        );
        assert_eq!(result, 120.0)
    }
}
