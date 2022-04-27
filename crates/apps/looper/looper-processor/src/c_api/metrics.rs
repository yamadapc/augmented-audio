use augmented_audio_metrics::audio_processor_metrics::AudioProcessorMetricsStats;

use crate::engine::LooperEngine;

#[repr(C)]
pub struct CAudioProcessorMetricsStats {
    pub average_cpu: f32,
    pub max_cpu: f32,
    pub average_nanos: f32,
    pub max_nanos: f32,
}

impl From<AudioProcessorMetricsStats> for CAudioProcessorMetricsStats {
    fn from(stats: AudioProcessorMetricsStats) -> Self {
        let AudioProcessorMetricsStats {
            average_cpu,
            max_cpu,
            average_nanos,
            max_nanos,
        } = stats;
        CAudioProcessorMetricsStats {
            average_cpu,
            max_cpu,
            average_nanos,
            max_nanos,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_stats(
    engine: *mut LooperEngine,
) -> CAudioProcessorMetricsStats {
    let metrics_actor = &(*engine).metrics_actor();
    if let Ok(mut metrics_actor) = metrics_actor.lock() {
        metrics_actor.poll().into()
    } else {
        AudioProcessorMetricsStats::default().into()
    }
}
