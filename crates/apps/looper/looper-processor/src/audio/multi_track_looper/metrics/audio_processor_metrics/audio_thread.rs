use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::time::{Duration, Instant};

use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_traits::AudioProcessorSettings;
use augmented_atomics::{AtomicF32, AtomicValue};

#[derive(Default)]
pub struct AudioProcessorMetricsHandle {
    /// Current processing time as a factor of the available time for the buffer size / sample rate
    /// configured
    duration_micros: AtomicU64,
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
}

impl AudioProcessorMetricsHandle {
    pub fn duration(&self) -> Duration {
        let duration_micros = self.duration_micros.get();
        Duration::from_micros(duration_micros)
    }

    pub fn cpu_percent(&self) -> f32 {
        let time_per_sample = 1.0 / self.sample_rate.get();
        let time_per_block = time_per_sample * self.buffer_size.get() as f32;
        let duration = self.duration();
        duration.as_secs_f32() / time_per_block
    }

    pub fn prepare(&self, settings: AudioProcessorSettings) {
        self.sample_rate.set(settings.sample_rate());
        self.buffer_size.set(settings.block_size());
    }
}

pub struct AudioProcessorMetrics {
    last_start_time: Instant,
    handle: Shared<AudioProcessorMetricsHandle>,
}

impl Default for AudioProcessorMetrics {
    fn default() -> Self {
        Self {
            last_start_time: Instant::now(),
            handle: make_shared(Default::default()),
        }
    }
}

impl AudioProcessorMetrics {
    pub fn handle(&self) -> Shared<AudioProcessorMetricsHandle> {
        self.handle.clone()
    }

    pub fn on_process_start(&mut self) {
        self.last_start_time = Instant::now();
    }

    pub fn on_process_end(&self) {
        let duration = self.last_start_time.elapsed();
        let duration_micros = duration.as_micros() as u64;
        self.handle.duration_micros.set(duration_micros)
    }
}
