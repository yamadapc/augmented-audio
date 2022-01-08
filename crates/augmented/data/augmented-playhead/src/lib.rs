pub struct PlayHeadOptions {
    pub sample_rate: Option<f32>,
    pub tempo: Option<u32>,
    pub ticks_per_quarter_note: Option<u32>,
}

pub struct PlayHead {
    options: PlayHeadOptions,
    // micro-second position ; might be better to use double.
    position_us: u64,
    position_samples: u32,
    position_ticks: u32,
}

impl PlayHead {
    pub fn new(options: PlayHeadOptions) -> Self {
        Self {
            options,
            position_samples: 0,
            position_ticks: 0,
            position_us: 0,
        }
    }

    pub fn accept_samples(&mut self, num_samples: u32) {
        self.position_samples += num_samples;
        if let Some(sample_rate) = self.options.sample_rate {
            let elapsed_us = (1.0 / sample_rate) * (num_samples as f32) * 1_000_000.0;
            self.position_us += elapsed_us as u64;

            if let Some((tempo, ticks_per_quarter_note)) =
                self.options.tempo.zip(self.options.ticks_per_quarter_note)
            {
                let secs_per_beat = 1.0 / ((tempo as f32) / 60.0);
                let elapsed_beats = (elapsed_us / 1_000_000.0) / secs_per_beat;
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
            self.position_us += (elapsed_seconds * 1_000_000.0) as u64;

            if let Some(sample_rate) = self.options.sample_rate {
                let elapsed_samples = sample_rate * elapsed_seconds;
                self.position_samples += elapsed_samples as u32;
            }
        }
    }

    pub fn set_position_seconds(&mut self, seconds: f32) {
        self.position_us = (seconds * 1_000_000.0) as u64;
        self.accept_samples(0);
    }

    pub fn set_tempo(&mut self, tempo: u32) {
        self.options.tempo = Some(tempo);
    }

    pub fn options(&self) -> &PlayHeadOptions {
        &self.options
    }

    pub fn position_seconds(&self) -> f32 {
        self.position_us as f32 / 1_000_000.0
    }

    pub fn position_beats(&self) -> f32 {
        self.options
            .tempo
            .map(|tempo| {
                let secs = self.position_seconds();
                let beats_per_second = tempo as f32 / 60.0;
                beats_per_second * secs
            })
            .unwrap_or(0.0)
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
    use audio_processor_testing_helpers::assert_f_eq;

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
        assert!((play_head.position_seconds() - 0.0).abs() < f32::EPSILON);

        play_head.accept_samples(512);
        assert_eq!(play_head.position_samples, 512);
        // At 44100Hz each block should be roughly 1ms
        assert!((play_head.position_seconds() - 0.01160998).abs() < (1.0 / 1_000_000.0));
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
        assert!((play_head.position_seconds() - 0.0).abs() < f32::EPSILON);

        // At 44100Hz, 22050 samples equals to 500ms, which is 1 beat at 120bpm
        play_head.accept_samples(22050);
        assert!((play_head.position_seconds() - 0.5).abs() < f32::EPSILON);
        assert_eq!(play_head.position_ticks, 32);
    }

    #[test]
    fn test_accept_many_samples() {
        let sample_count = 5644800;
        let options = PlayHeadOptions {
            sample_rate: Some(44100.0),
            tempo: Some(120),
            ticks_per_quarter_note: Some(32),
        };
        let mut play_head = PlayHead::new(options);
        play_head.accept_samples(sample_count);
        assert!((play_head.position_seconds() - 128.0).abs() < f32::EPSILON);
        assert!((play_head.position_beats() - 256.0).abs() < f32::EPSILON);
        play_head.accept_samples(sample_count / 2);
        assert!((play_head.position_seconds() - 192.0).abs() < f32::EPSILON);
        assert!((play_head.position_beats() - 384.0).abs() < f32::EPSILON);
    }

    // #[test]
    // fn test_accept_samples_loop() {
    //     let inverse_sample_rate = 1.0 / 44100.0;
    //     let options = PlayHeadOptions {
    //         sample_rate: Some(44100.0),
    //         tempo: Some(120),
    //         ticks_per_quarter_note: Some(32),
    //     };
    //     let mut play_head = PlayHead::new(options);
    //
    //     for i in 0..100_000_000 {
    //         let expected = (i as f32) * inverse_sample_rate;
    //         assert!(
    //             play_head.position_seconds() - expected < 0.02,
    //             "(i = {}) {} != {}",
    //             i,
    //             play_head.position_seconds(),
    //             expected
    //         );
    //         play_head.accept_samples(1);
    //     }
    // }
}
