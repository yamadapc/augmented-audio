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
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::time::{Duration, Instant};

use audio_garbage_collector::{make_shared, Shared};
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

    pub fn prepare(&self, settings: AudioProcessorSettings) {
        self.handle.prepare(settings);
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
