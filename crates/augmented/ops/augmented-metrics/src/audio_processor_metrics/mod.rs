// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//! Tracks CPU usage of audio processing code as a factor of the available time for the sample-rate
//! and buffer-size.
//!
//! For example, if 44.1kHz is the sample-rate and 512 is the block size, we've 11ms to process. If
//! the processor takes 5.5ms this should report 50% usage.
//!
//! There are two sides for the usage measurement:
//!
//! * [`AudioProcessorMetrics`] should be added to the audio-processor code
//!   - Its [`AudioProcessorMetricsHandle`] reference will be shared with measurement state
//! * [`AudioProcessorMetricsActor`] is a polling aggregation background worker that can aggregate
//!   the last few measurements
//!   - It'll return [`AudioProcessorMetricsStats`]
//!
//! # Example audio-thread integration
//! `AudioProcessorMetrics` is a real-time safe object you can use in audio-processing code. The
//! `on_process_start`/`on_process_end` methods will time the processing and store this in a shared
//! handle.
//!
//! ```rust
//! use audio_processor_traits::{AudioProcessor, AudioBuffer, AudioProcessorSettings, AudioContext};
//! use augmented_audio_metrics::audio_processor_metrics::{AudioProcessorMetrics, AudioProcessorMetricsHandle};
//!
//! struct TrackedProcessor {
//!     metrics: AudioProcessorMetrics
//! }
//!
//! impl AudioProcessor for TrackedProcessor {
//!     type SampleType = f32;
//!
//!     fn prepare(
//!         &mut self,
//!         context: &mut AudioContext
//!     ) {
//!         self.metrics.prepare(context.settings);
//!     }
//!
//!     fn process(&mut self, _context: &mut AudioContext, _data: &mut AudioBuffer<Self::SampleType>) {
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
//! use augmented_audio_metrics::audio_processor_metrics::{AudioProcessorMetricsHandle, AudioProcessorMetrics, AudioProcessorMetricsActor, AudioProcessorMetricsStats};
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
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::time::Duration;

use audio_garbage_collector::Shared;
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
        self.push_measurement(duration, cpu_percent);

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
            average_cpu: fix_nan(average_cpu),
            max_cpu: fix_nan(max_cpu),
            average_nanos: fix_nan(average_nanos),
            max_nanos: fix_nan(max_nanos),
        }
    }

    fn push_measurement(&mut self, duration: Duration, cpu_percent: f32) {
        self.last_measurements.push_front((cpu_percent, duration));
        self.last_measurements.truncate(MAX_FRAMES);
    }
}

fn fix_nan(value: f32) -> f32 {
    if value.is_nan() {
        0.0
    } else {
        value
    }
}

#[derive(Default)]
pub struct AudioProcessorMetricsStats {
    pub average_cpu: f32,
    pub max_cpu: f32,
    pub average_nanos: f32,
    pub max_nanos: f32,
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::make_shared;
    use audio_processor_traits::AudioProcessorSettings;

    use super::*;

    #[test]
    fn test_actor_poll() {
        let handle = AudioProcessorMetricsHandle::default();
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 1000.0;
        settings.block_size = 100;
        handle.prepare(settings);

        let handle = make_shared(handle);
        let mut actor = AudioProcessorMetricsActor::new(handle.clone());

        handle.set_duration(Duration::from_millis(50));
        let stats = actor.poll();
        assert!((stats.average_cpu - 0.5).abs() < f32::EPSILON);
        assert!((stats.max_cpu - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_push_measurement_pushes_measurement_to_the_start_of_the_deque() {
        let handle = make_shared(AudioProcessorMetricsHandle::default());
        let mut actor = AudioProcessorMetricsActor::new(handle);
        assert!(actor.last_measurements.is_empty());
        actor.push_measurement(Duration::from_millis(50), 0.5);
        assert!(!actor.last_measurements.is_empty());
        assert_eq!(actor.last_measurements.len(), 1);
        assert!((actor.last_measurements[0].0 - 0.5).abs() < f32::EPSILON);
        assert_eq!(actor.last_measurements[0].1, Duration::from_millis(50));
    }

    #[test]
    fn test_push_measurement_removes_the_first_measurements_when_at_max_frames() {
        let handle = make_shared(AudioProcessorMetricsHandle::default());
        let mut actor = AudioProcessorMetricsActor::new(handle);
        assert!(actor.last_measurements.is_empty());

        actor.push_measurement(Duration::from_millis(0), 0.1);
        for _i in 0..MAX_FRAMES {
            actor.push_measurement(Duration::from_millis(5), 0.5);
        }
        assert!(!actor.last_measurements.is_empty());
        assert_eq!(actor.last_measurements.len(), MAX_FRAMES);

        for (cpu_percent, duration) in actor.last_measurements.iter() {
            assert!((cpu_percent - 0.5).abs() < f32::EPSILON);
            assert_eq!(*duration, Duration::from_millis(5));
        }
    }

    #[test]
    fn test_fix_nan() {
        let nan = f32::NAN;
        assert!((fix_nan(nan) - 0.0).abs() < f32::EPSILON);
        let v = 20.0;
        assert!((fix_nan(v) - 20.0).abs() < f32::EPSILON);
    }
}
