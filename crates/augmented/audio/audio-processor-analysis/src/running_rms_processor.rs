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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use audio_garbage_collector::{Handle, Shared, SharedCell};
use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
use audio_processor_traits::{
    AtomicF32, AudioBuffer, AudioProcessorSettings, SimpleAudioProcessor,
};

/// A shared "processor handle" to `RunningRMSProcessor`
pub struct RunningRMSProcessorHandle {
    window: SharedCell<VecAudioBuffer<AtomicF32>>,
    running_sums: SharedCell<Vec<AtomicF32>>,
    cursor: AtomicUsize,
    duration_micros: AtomicUsize,
}

impl RunningRMSProcessorHandle {
    /// Create a new handle with empty buffers
    fn new(gc_handle: &Handle) -> Self {
        RunningRMSProcessorHandle {
            window: SharedCell::new(Shared::new(gc_handle, VecAudioBuffer::new())),
            running_sums: SharedCell::new(Shared::new(gc_handle, Vec::new())),
            cursor: AtomicUsize::new(0),
            duration_micros: AtomicUsize::new(0),
        }
    }

    /// Create a new RMS window with size & replace the old one with it.
    fn resize(&self, gc_handle: &Handle, num_channels: usize, num_samples: usize) {
        self.cursor.store(0, Ordering::Relaxed);

        let mut window = VecAudioBuffer::new();
        window.resize(num_channels, num_samples, AtomicF32::from(0.0));
        self.window.replace(Shared::new(gc_handle, window));

        let mut running_sums = Vec::new();
        running_sums.resize(num_channels, AtomicF32::from(0.0));
        self.running_sums
            .replace(Shared::new(gc_handle, running_sums));
    }

    /// Calculate the RMS of the current window based on its running sum and size
    pub fn calculate_rms(&self, channel: usize) -> f32 {
        let running_sums = self.running_sums.get();
        if channel >= running_sums.len() {
            return 0.0;
        }

        let sum = running_sums[channel].get().max(0.0);
        (sum / self.window.get().num_samples() as f32).sqrt()
    }
}

/// An `AudioProcessor` which slides a window & calculates a running Squared sum of the input.
///
/// It exposes a `RunningRMSProcessorHandle` which may be called from any thread to get the current
/// RMS in real-time.
///
/// When the internal window's buffer needs to be resized, it's replaced via an atomic pointer swap.
pub struct RunningRMSProcessor {
    handle: Shared<RunningRMSProcessorHandle>,
    duration_samples: usize,
    duration: Duration,
    gc_handle: Handle,
}

impl RunningRMSProcessor {
    /// Create a `RunningRMSProcessor` which will calculate RMS based on a certain `duration` of
    /// samples.
    pub fn new_with_duration(gc_handle: &Handle, duration: Duration) -> Self {
        let handle = Shared::new(gc_handle, RunningRMSProcessorHandle::new(gc_handle));
        handle
            .duration_micros
            .store(duration.as_micros() as usize, Ordering::Relaxed);

        RunningRMSProcessor {
            handle,
            duration_samples: 0,
            duration,
            gc_handle: gc_handle.clone(),
        }
    }

    pub fn from_handle(handle: Shared<RunningRMSProcessorHandle>) -> Self {
        let duration = Duration::from_micros(handle.duration_micros.load(Ordering::Relaxed) as u64);
        Self {
            gc_handle: audio_garbage_collector::handle().clone(),
            handle,
            duration_samples: 0,
            duration: duration,
        }
    }

    pub fn handle(&self) -> &Shared<RunningRMSProcessorHandle> {
        &self.handle
    }
}

impl SimpleAudioProcessor for RunningRMSProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        self.duration_samples = (settings.sample_rate() * self.duration.as_secs_f32()) as usize;
        self.handle.resize(
            &self.gc_handle,
            settings.output_channels(),
            self.duration_samples,
        );
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        if self.duration_samples == 0 {
            return;
        }

        let running_sums = self.handle.running_sums.get();
        let window = self.handle.window.get();
        let mut cursor = self.handle.cursor.load(Ordering::Relaxed);
        for (channel_index, sample) in frame.iter().enumerate() {
            let value_slot = window.get(channel_index, cursor);
            let previous_value = value_slot.get();

            let new_value = *sample * *sample; // using square rather than abs is around 1% faster
            value_slot.set(new_value);

            let running_sum_slot = &running_sums[channel_index];
            let running_sum = running_sum_slot.get() + new_value - previous_value;
            running_sum_slot.set(running_sum);
        }

        cursor += 1;
        if cursor >= self.duration_samples {
            cursor = 0;
        }
        self.handle.cursor.store(cursor, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::GarbageCollector;
    use audio_processor_traits::simple_processor::process_buffer;

    use super::*;

    #[test]
    fn test_audio_process_running() {
        let gc = GarbageCollector::default();
        let mut processor =
            RunningRMSProcessor::new_with_duration(gc.handle(), Duration::from_millis(10));
        let mut test_buffer = VecAudioBuffer::new();

        test_buffer.resize(2, 1000, 1.0);
        processor.s_prepare(AudioProcessorSettings::default());
        process_buffer(&mut processor, &mut test_buffer);
        let rms = processor.handle.calculate_rms(0);
        assert!(rms > 0.0);
    }
}
