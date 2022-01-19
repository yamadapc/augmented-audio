use audio_processor_standalone::standalone_vst::vst;
use audio_processor_standalone::standalone_vst::vst::host::Host;
use audio_processor_standalone::standalone_vst::vst::plugin::HostCallback;
use augmented_playhead::{PlayHead, PlayHeadOptions};
use mockall::automock;

pub struct TimeInfo {
    pub(crate) tempo: Option<f64>,
    pub(crate) position_samples: f64,
    pub(crate) position_beats: Option<f64>,
}

#[automock]
pub trait TimeInfoProvider {
    fn get_time_info(&self) -> TimeInfo;
    fn tick(&mut self);
}

pub struct TimeInfoProviderImpl {
    host_callback: Option<HostCallback>,
    playhead: PlayHead,
}

impl TimeInfoProvider for TimeInfoProviderImpl {
    fn get_time_info(&self) -> TimeInfo {
        let host_time_info = self
            .host_callback
            .map(|cb| {
                cb.get_time_info(
                    (vst::api::TimeInfoFlags::TEMPO_VALID | vst::api::TimeInfoFlags::PPQ_POS_VALID)
                        .bits(),
                )
            })
            .flatten()
            .map(|vst_time_info| TimeInfo {
                tempo: Some(vst_time_info.tempo),
                position_samples: vst_time_info.sample_pos,
                position_beats: Some(vst_time_info.ppq_pos),
            });

        host_time_info.unwrap_or_else(|| TimeInfo {
            tempo: self.playhead.options().tempo.map(|t| t as f64),
            position_samples: self.playhead.position_samples() as f64,
            position_beats: self
                .playhead
                .options()
                .tempo
                .map(|_| self.playhead.position_beats()),
        })
    }

    fn tick(&mut self) {
        self.playhead.accept_samples(1);
    }
}

impl TimeInfoProviderImpl {
    pub fn new(host_callback: Option<HostCallback>) -> Self {
        TimeInfoProviderImpl {
            host_callback,
            playhead: PlayHead::new(PlayHeadOptions {
                sample_rate: None,
                tempo: None,
                ticks_per_quarter_note: None,
            }),
        }
    }

    pub fn set_tempo(&mut self, tempo: u32) {
        self.playhead.set_tempo(tempo);
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.playhead.set_sample_rate(sample_rate);
    }
}
