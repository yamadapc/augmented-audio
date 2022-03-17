use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use augmented_atomics::{AtomicF32, AtomicF64, AtomicOption, AtomicValue};

pub struct PlayHeadOptions {
    sample_rate: AtomicOption<AtomicF32>,
    tempo: AtomicOption<AtomicF32>,
    ticks_per_quarter_note: AtomicOption<AtomicU32>,
}

impl PlayHeadOptions {
    pub fn new(
        sample_rate: Option<f32>,
        tempo: Option<f32>,
        ticks_per_quarter_note: Option<u32>,
    ) -> Self {
        PlayHeadOptions {
            sample_rate: sample_rate.into(),
            tempo: tempo.into(),
            ticks_per_quarter_note: ticks_per_quarter_note.into(),
        }
    }

    pub fn sample_rate(&self) -> Option<f32> {
        self.sample_rate.inner()
    }

    pub fn tempo(&self) -> Option<f32> {
        self.tempo.inner()
    }

    pub fn ticks_per_quarter_note(&self) -> Option<u32> {
        self.ticks_per_quarter_note.inner()
    }
}

pub struct PlayHead {
    options: PlayHeadOptions,
    // micro-second position ; might be better to use double.
    position_us: AtomicU64,
    position_samples: AtomicU32,
    position_ticks: AtomicU32,
    position_beats: AtomicF64,
}

impl PlayHead {
    pub fn new(options: PlayHeadOptions) -> Self {
        Self {
            options,
            position_beats: AtomicF64::from(0.0),
            position_samples: AtomicU32::from(0),
            position_ticks: AtomicU32::from(0),
            position_us: AtomicU64::from(0),
        }
    }

    pub fn accept_samples(&self, num_samples: u32) {
        self.position_samples
            .fetch_add(num_samples, Ordering::Relaxed);
        if let Some(sample_rate) = self.options.sample_rate.inner() {
            let elapsed_secs = (1.0 / sample_rate) * (num_samples as f32);
            let elapsed_us = elapsed_secs * 1_000_000.0;
            self.position_us
                .fetch_add(elapsed_us as u64, Ordering::Relaxed);
            self.update_position_beats(elapsed_secs as f64);

            if let Some((tempo, ticks_per_quarter_note)) = self
                .options
                .tempo
                .inner()
                .zip(self.options.ticks_per_quarter_note.inner())
            {
                let secs_per_beat = 1.0 / ((tempo as f32) / 60.0);
                let elapsed_beats = (elapsed_us / 1_000_000.0) / secs_per_beat;
                self.position_ticks.store(
                    (ticks_per_quarter_note as f32 * elapsed_beats) as u32,
                    Ordering::Relaxed,
                );
            }
        }
    }

    pub fn accept_ticks(&self, num_ticks: u32) {
        self.position_ticks.fetch_add(num_ticks, Ordering::Relaxed);

        if let Some((tempo, ticks_per_quarter_note)) = self
            .options
            .tempo
            .inner()
            .zip(self.options.ticks_per_quarter_note.inner())
        {
            let elapsed_beats = num_ticks as f32 / ticks_per_quarter_note as f32;
            let secs_per_beat = 1.0 / ((tempo as f32) / 60.0);
            let elapsed_seconds = elapsed_beats * secs_per_beat;
            let elapsed_us = (elapsed_seconds * 1_000_000.0) as u64;
            self.position_us.fetch_add(elapsed_us, Ordering::Relaxed);
            self.update_position_beats(elapsed_seconds as f64);

            if let Some(sample_rate) = self.options.sample_rate.inner() {
                let elapsed_samples = sample_rate * elapsed_seconds;
                self.position_samples
                    .fetch_add(elapsed_samples as u32, Ordering::Relaxed);
            }
        }
    }

    pub fn set_position_seconds(&self, seconds: f32) {
        self.position_us
            .store((seconds * 1_000_000.0) as u64, Ordering::Relaxed);
        self.position_beats.store(
            self.options
                .tempo
                .inner()
                .map(|tempo| {
                    let beats_per_second = tempo as f64 / 60.0;
                    beats_per_second * seconds as f64
                })
                .unwrap_or(0.0),
            Ordering::Relaxed,
        );
        self.accept_samples(0);
    }

    pub fn set_tempo(&self, tempo: f32) {
        self.options.tempo.set(Some(tempo));
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.options.sample_rate.set(Some(sample_rate));
    }

    pub fn options(&self) -> &PlayHeadOptions {
        &self.options
    }

    pub fn position_seconds(&self) -> f32 {
        self.position_us.get() as f32 / 1_000_000.0
    }

    pub fn position_beats(&self) -> f64 {
        self.position_beats.get()
    }

    pub fn position_samples(&self) -> u32 {
        self.position_samples.get()
    }

    pub fn position_ticks(&self) -> u32 {
        self.position_ticks.get()
    }

    fn update_position_beats(&self, elapsed_secs: f64) {
        let position_beats = self.position_beats.get();
        let position_beats = position_beats
            + self
                .options
                .tempo
                .inner()
                .map(|tempo| {
                    let beats_per_second = tempo as f64 / 60.0;
                    beats_per_second * elapsed_secs
                })
                .unwrap_or(0.0);
        self.position_beats.set(position_beats);
    }
}

#[cfg(test)]
mod test {
    use crate::{PlayHead, PlayHeadOptions};

    #[test]
    fn test_accept_samples() {
        let options = PlayHeadOptions::new(Some(44100.0), Some(120.0), Some(32));
        let play_head = PlayHead::new(options);
        assert_eq!(play_head.position_samples(), 0);
        assert_eq!(play_head.position_ticks(), 0);
        assert!((play_head.position_seconds() - 0.0).abs() < f32::EPSILON);

        play_head.accept_samples(512);
        assert_eq!(play_head.position_samples(), 512);
        // At 44100Hz each block should be roughly 1ms
        assert!((play_head.position_seconds() - 0.01160998).abs() < (1.0 / 1_000_000.0));
    }

    #[test]
    fn test_accept_samples_ticks_conversion() {
        let options = PlayHeadOptions::new(Some(44100.0), Some(120.0), Some(32));
        let play_head = PlayHead::new(options);
        assert_eq!(play_head.position_samples(), 0);
        assert_eq!(play_head.position_ticks(), 0);
        assert!((play_head.position_seconds() - 0.0).abs() < f32::EPSILON);

        // At 44100Hz, 22050 samples equals to 500ms, which is 1 beat at 120bpm
        play_head.accept_samples(22050);
        assert!((play_head.position_seconds() - 0.5).abs() < f32::EPSILON);
        assert_eq!(play_head.position_ticks(), 32);
    }

    #[test]
    fn test_accept_many_samples() {
        let sample_count = 5644800;
        let options = PlayHeadOptions::new(Some(44100.0), Some(120.0), Some(32));
        let play_head = PlayHead::new(options);
        play_head.accept_samples(sample_count);
        assert!((play_head.position_seconds() - 128.0).abs() < f32::EPSILON);
        assert!((play_head.position_beats() - 256.0).abs() < f64::EPSILON);
        play_head.accept_samples(sample_count / 2);
        assert!((play_head.position_seconds() - 192.0).abs() < f32::EPSILON);
        assert!((play_head.position_beats() - 384.0).abs() < f64::EPSILON);
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
