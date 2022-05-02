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

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_convert_stats() {
        let rust_stats = AudioProcessorMetricsStats {
            average_cpu: 0.3,
            max_cpu: 0.5,
            average_nanos: 10.0,
            max_nanos: 20.0,
        };
        let c_stats: CAudioProcessorMetricsStats = rust_stats.into();
        assert_f_eq!(c_stats.average_nanos, 10.0);
        assert_f_eq!(c_stats.average_cpu, 0.3);
        assert_f_eq!(c_stats.max_nanos, 20.0);
        assert_f_eq!(c_stats.max_cpu, 0.5);
    }

    #[test]
    fn test_get_stats() {
        let engine = LooperEngine::default();
        let engine_ptr = Box::into_raw(Box::new(engine));

        let stats = unsafe { looper_engine__get_stats(engine_ptr) };
        assert!(stats.average_nanos.abs() < f32::EPSILON);
        assert!(stats.max_cpu.abs() < f32::EPSILON);
        assert!(stats.average_nanos.abs() < f32::EPSILON);
        assert!(stats.average_cpu.abs() < f32::EPSILON);

        unsafe {
            let _ = Box::from_raw(engine_ptr);
        }
    }
}
