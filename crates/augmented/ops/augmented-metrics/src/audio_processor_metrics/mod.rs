//! Tracks CPU usage of audio processing code as a factor of the available time for the sample-rate
//! and buffer-size.
//!
//! For example, if 44.1kHz is the sample-rate and 512 is the block size, we've 11ms to process. If
//! the processor takes 5.5ms this should report 50% usage.
//!
//! There are two sides for the usage measurement:
//!
//! * `AudioProcessorMetrics` should be added to the audio-processor code
//! * `AudioProcessorMetricsActor` is a polling aggregation background worker that can aggregate the
//!   last few measurements
//!
//! # Example audio-thread integration
//! `AudioProcessorMetrics` is a real-time safe object you can use in audio-processing code. The
//! `on_process_start`/`on_process_end` methods will time the processing and store this in a shared
//! handle.
//!
//! ```rust
//! use audio_processor_traits::{AudioProcessor, AudioBuffer, AudioProcessorSettings};
//! use augmented_audio_metrics::{AudioProcessorMetrics, AudioProcessorMetricsHandle};
//!
//! struct TrackedProcessor {
//!     metrics: AudioProcessorMetrics
//! }
//!
//! impl AudioProcessor for TrackedProcessor {
//!     type SampleType = f32;
//!
//!     fn prepare(&mut self, settings: AudioProcessorSettings) {
//!         self.metrics.prepare(settings);
//!     }
//!
//!     fn process<BufferType: AudioBuffer<SampleType=Self::SampleType>>(
//!         &mut self,
//!         data: &mut BufferType
//!     ) {
//!         self.metrics.on_process_start();
//!
//!         // do audio-processing work
//!
//!         self.metrics.on_process_end();
//!     }
//! }
//! ```
//!
//! # Example metrics polling
//! The metrics then need to be polled out of the `AudioProcessorMetricsHandle`.
//! `AudioProcessorMetricsActor` implements basic average/max aggregation by polling from this
//! handle. It exposes a single `poll` method that should be called at any poll interval.
//!
//! In this example, you can see how it could be called every second from a background-thread to log
//! metrics.
//!
//! ```rust
//! use std::time::Duration;
//! use audio_garbage_collector::Shared;
//! use augmented_audio_metrics::{AudioProcessorMetricsHandle, AudioProcessorMetrics, AudioProcessorMetricsActor, AudioProcessorMetricsStats};
//!
//! let metrics = AudioProcessorMetrics::default(); // this would be created in some processor
//!
//! // Shared ref. counted reference to the handle
//! let metrics_handle: Shared<AudioProcessorMetricsHandle> = metrics.handle().clone();
//! std::thread::spawn(move || {
//!     let mut metrics_actor = AudioProcessorMetricsActor::new(metrics_handle);
//!     for _i in 0..3 {
//!         let AudioProcessorMetricsStats {
//!             average_cpu,
//!             max_cpu,
//!             average_nanos,
//!             max_nanos
//!         } = metrics_actor.poll();
//!         log::info!("average_cpu={}", average_cpu);
//!         log::info!("max_cpu={}", max_cpu);
//!         log::info!("average_nanos={}", average_nanos);
//!         log::info!("max_nanos={}", max_nanos);
//!         std::thread::sleep(Duration::from_secs(1));
//!     }
//! });
//! ```
use audio_garbage_collector::Shared;
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
