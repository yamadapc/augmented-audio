pub struct PlayHeadOptions {
    sample_rate: Option<f32>,
    tempo: Option<u32>,
    ticks_per_quarter_note: Option<u32>,
}

pub struct PlayHead {
    options: PlayHeadOptions,
    position_seconds: f32,
    position_samples: u32,
    position_ticks: u32,
}

impl PlayHead {
    pub fn new(options: PlayHeadOptions) -> Self {
        Self {
            options,
            position_samples: 0,
            position_ticks: 0,
            position_seconds: 0.0,
        }
    }

    pub fn accept_samples(&mut self, num_samples: u32) {
        self.position_samples += num_samples;
        if let Some(sample_rate) = self.options.sample_rate {
            let elapsed_secs = (1.0 / sample_rate) * (num_samples as f32);
            self.position_seconds += elapsed_secs;

            if let Some((tempo, ticks_per_quarter_note)) =
                self.options.tempo.zip(self.options.ticks_per_quarter_note)
            {
                let secs_per_beat = 1.0 / ((tempo as f32) / 60.0);
                let elapsed_beats = elapsed_secs / secs_per_beat;
                self.position_ticks = (ticks_per_quarter_note as f32 * elapsed_beats) as u32;
            }
        }
    }

    pub fn accept_ticks(&mut self, num_ticks: u32) {
        self.position_ticks += num_ticks;

        if let Some((tempo, ticks_per_quarter_note)) =
            self.options.tempo.zip(self.options.ticks_per_quarter_note)
        {
            let elapsed_beats = num_ticks as f32 / ticks_per_quarter_note as f32;
            let secs_per_beat = 1.0 / ((tempo as f32) / 60.0);
            let elapsed_seconds = elapsed_beats * secs_per_beat;
            self.position_seconds += elapsed_seconds;

            if let Some(sample_rate) = self.options.sample_rate {
                let elapsed_samples = sample_rate * elapsed_seconds;
                self.position_samples += elapsed_samples as u32;
            }
        }
    }

    pub fn options(&self) -> &PlayHeadOptions {
        &self.options
    }

    pub fn position_seconds(&self) -> f32 {
        self.position_seconds
    }

    pub fn position_samples(&self) -> u32 {
        self.position_samples
    }

    pub fn position_ticks(&self) -> u32 {
        self.position_ticks
    }
}

#[cfg(test)]
mod test {
    use crate::{PlayHead, PlayHeadOptions};

    #[test]
    fn test_accept_samples() {
        let options = PlayHeadOptions {
            sample_rate: Some(44100.0),
            tempo: Some(120),
            ticks_per_quarter_note: Some(32),
        };
        let mut play_head = PlayHead::new(options);
        assert_eq!(play_head.position_samples, 0);
        assert_eq!(play_head.position_ticks, 0);
        assert!((play_head.position_seconds - 0.0).abs() < f32::EPSILON);

        play_head.accept_samples(512);
        assert_eq!(play_head.position_samples, 512);
        // At 44100Hz each block should be roughly 1ms
        assert!((play_head.position_seconds - 0.01160998).abs() < f32::EPSILON);
    }

    #[test]
    fn test_accept_samples_ticks_conversion() {
        let options = PlayHeadOptions {
            sample_rate: Some(44100.0),
            tempo: Some(120),
            ticks_per_quarter_note: Some(32),
        };
        let mut play_head = PlayHead::new(options);
        assert_eq!(play_head.position_samples, 0);
        assert_eq!(play_head.position_ticks, 0);
        assert!((play_head.position_seconds - 0.0).abs() < f32::EPSILON);

        // At 44100Hz, 22050 samples equals to 500ms, which is 1 beat at 120bpm
        play_head.accept_samples(22050);
        assert!((play_head.position_seconds - 0.5).abs() < f32::EPSILON);
        assert_eq!(play_head.position_ticks, 32);
    }
}
