use std::sync::atomic::{AtomicBool, Ordering};

use derive_builder::Builder;
use mockall::automock;

#[cfg(not(target_os = "ios"))]
pub use audio_processor_standalone::standalone_vst::vst::plugin::HostCallback;
#[cfg(not(target_os = "ios"))]
use audio_processor_standalone::{standalone_vst::vst, standalone_vst::vst::host::Host};
use augmented_playhead::{PlayHead, PlayHeadOptions};

#[cfg(target_os = "ios")]
pub type HostCallback = ();

#[derive(Builder)]
pub struct TimeInfo {
    tempo: Option<f64>,
    position_samples: f64,
    position_beats: Option<f64>,
}

impl TimeInfo {
    pub fn tempo(&self) -> Option<f64> {
        self.tempo
    }

    pub fn position_samples(&self) -> f64 {
        self.position_samples
    }

    pub fn position_beats(&self) -> Option<f64> {
        self.position_beats
    }
}

#[automock]
pub trait TimeInfoProvider {
    fn get_time_info(&self) -> TimeInfo;
    fn tick(&self);
    // Transport control; this should only be enabled when the looper is the master tempo
    fn play(&self);
    fn stop(&self);
    fn pause(&self);
}

pub struct TimeInfoProviderImpl {
    host_callback: Option<HostCallback>,
    playhead: PlayHead,
    is_playing: AtomicBool,
}

impl TimeInfoProvider for TimeInfoProviderImpl {
    #[cfg(target_os = "ios")]
    fn get_time_info(&self) -> TimeInfo {
        self.playhead_timeinfo()
    }

    #[cfg(not(target_os = "ios"))]
    fn get_time_info(&self) -> TimeInfo {
        let host_time_info = self
            .host_callback
            .and_then(|cb| {
                cb.get_time_info(
                    (vst::api::TimeInfoFlags::TEMPO_VALID | vst::api::TimeInfoFlags::PPQ_POS_VALID)
                        .bits(),
                )
            })
            .map(|vst_time_info| TimeInfo {
                tempo: Some(vst_time_info.tempo),
                position_samples: vst_time_info.sample_pos,
                position_beats: Some(vst_time_info.ppq_pos),
            });

        host_time_info.unwrap_or_else(|| self.playhead_timeinfo())
    }

    fn tick(&self) {
        if self.is_playing.load(Ordering::Relaxed) {
            self.playhead.accept_samples(1);
        }
    }

    fn play(&self) {
        self.is_playing.store(true, Ordering::Relaxed);
    }

    fn stop(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
        self.playhead.set_position_seconds(0.0);
    }

    fn pause(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }
}

impl TimeInfoProviderImpl {
    pub fn new(host_callback: Option<HostCallback>) -> Self {
        TimeInfoProviderImpl {
            host_callback,
            playhead: PlayHead::new(PlayHeadOptions::new(None, None, None)),
            is_playing: AtomicBool::new(false),
        }
    }

    pub fn playhead(&self) -> &PlayHead {
        &self.playhead
    }

    pub fn set_tempo(&self, tempo: f32) {
        self.playhead.set_tempo(tempo);
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.playhead.set_sample_rate(sample_rate);
    }

    fn playhead_timeinfo(&self) -> TimeInfo {
        TimeInfo {
            tempo: self.playhead.options().tempo().map(|t| t as f64),
            position_samples: self.playhead.position_samples() as f64,
            position_beats: self
                .playhead
                .options()
                .tempo()
                .map(|_| self.playhead.position_beats()),
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_time_info_provider_without_tempo_doesnt_move() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.tick();
        time_info_provider.tick();
        time_info_provider.tick();
        time_info_provider.tick();
        assert!(time_info_provider
            .get_time_info()
            .position_beats()
            .is_none());
        assert_f_eq!(time_info_provider.get_time_info().position_samples(), 4.0);
    }

    #[test]
    fn test_time_info_provider_with_tempo_keep_track_of_beats() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(100.0);
        time_info_provider.set_tempo(60.0);

        let time_info = time_info_provider.get_time_info();
        assert!(time_info.position_beats().is_some());
        assert_eq!(time_info.position_beats(), Some(0.0));
        assert_eq!(time_info.position_samples(), 0.0);

        for _i in 0..100 {
            time_info_provider.tick();
        }
        let time_info = time_info_provider.get_time_info();
        assert!(time_info.position_beats().is_some());
        assert_f_eq!(time_info.position_beats().unwrap(), 1.0);
        assert_f_eq!(time_info.position_samples(), 100.0);
    }
}
