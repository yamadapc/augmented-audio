use basedrop::Shared;
use std::cmp::Ordering;
use std::collections::VecDeque;

use std::time::Duration;

pub use audio_thread::*;

mod audio_thread;

const MAX_FRAMES: usize = 100;

/// This is a stateful struct that should poll the metrics handle.
///
/// It'll build statistics on audio-thread performance over time.
pub struct AudioProcessorMetricsActor {
    last_measurements: VecDeque<(f32, Duration)>,
    handle: Shared<AudioProcessorMetricsHandle>,
}

impl AudioProcessorMetricsActor {
    pub fn new(handle: Shared<AudioProcessorMetricsHandle>) -> Self {
        Self {
            last_measurements: VecDeque::new(),
            handle,
        }
    }

    pub fn poll(&mut self) -> AudioProcessorMetricsStats {
        let duration = self.handle.duration();
        let cpu_percent = self.handle.cpu_percent();
        self.last_measurements.push_front((cpu_percent, duration));
        self.last_measurements.truncate(MAX_FRAMES);

        let durations_nanos: Vec<f32> = self
            .last_measurements
            .iter()
            .map(|(_, duration)| duration.as_nanos() as f32)
            .collect();
        let durations_cpu: Vec<f32> = self.last_measurements.iter().map(|(cpu, _)| *cpu).collect();
        let average_cpu = durations_cpu.iter().sum::<f32>() / self.last_measurements.len() as f32;
        let max_cpu = durations_cpu
            .iter()
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap_or(Ordering::Equal))
            .cloned()
            .unwrap_or(0.0);
        let average_nanos =
            durations_nanos.iter().sum::<f32>() / self.last_measurements.len() as f32;
        let max_nanos = durations_nanos
            .iter()
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap_or(Ordering::Equal))
            .cloned()
            .unwrap_or(0.0);

        AudioProcessorMetricsStats {
            average_cpu,
            max_cpu,
            average_nanos,
            max_nanos,
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct AudioProcessorMetricsStats {
    pub average_cpu: f32,
    pub max_cpu: f32,
    pub average_nanos: f32,
    pub max_nanos: f32,
}
