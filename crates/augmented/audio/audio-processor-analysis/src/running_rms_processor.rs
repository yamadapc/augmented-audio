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
use audio_processor_traits::atomic_float::{AtomicFloatRepresentable, AtomicValue};
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor, Float};

pub type RunningRMSProcessorHandle = RunningRMSProcessorHandleImpl<f32>;

/// A shared "processor handle" to `RunningRMSProcessor`
pub struct RunningRMSProcessorHandleImpl<ST: AtomicFloatRepresentable> {
    window: SharedCell<AudioBuffer<ST::AtomicType>>,
    running_sums: SharedCell<Vec<ST::AtomicType>>,
    cursor: AtomicUsize,
    duration_micros: AtomicUsize,
}

impl<ST> RunningRMSProcessorHandleImpl<ST>
where
    ST: AtomicFloatRepresentable + Float,
    ST::AtomicType: Send + Sync + Clone + 'static,
{
    /// Create a new handle with empty buffers
    fn new(gc_handle: &Handle) -> Self {
        RunningRMSProcessorHandleImpl {
            window: SharedCell::new(Shared::new(gc_handle, AudioBuffer::empty())),
            running_sums: SharedCell::new(Shared::new(gc_handle, Vec::new())),
            cursor: AtomicUsize::new(0),
            duration_micros: AtomicUsize::new(0),
        }
    }

    fn cursor(&self) -> usize {
        self.cursor.load(Ordering::Relaxed)
    }

    /// Create a new RMS window with size & replace the old one with it.
    #[numeric_literals::replace_float_literals(ST::from(literal).unwrap())]
    fn resize(&self, gc_handle: &Handle, num_channels: usize, num_samples: usize) {
        self.cursor.store(0, Ordering::Relaxed);

        let mut window = AudioBuffer::empty();
        window.resize_with(num_channels, num_samples, || ST::AtomicType::from(0.0));
        self.window.replace(Shared::new(gc_handle, window));

        let mut running_sums = Vec::new();
        running_sums.resize(num_channels, ST::AtomicType::from(0.0));
        self.running_sums
            .replace(Shared::new(gc_handle, running_sums));
    }

    /// Calculate the RMS of the current window based on its running sum and size
    #[numeric_literals::replace_float_literals(ST::from(literal).unwrap())]
    pub fn calculate_rms(&self, channel: usize) -> ST {
        let running_sums = self.running_sums.get();
        if channel >= running_sums.len() {
            return 0.0;
        }

        let sum = running_sums[channel].get().max(0.0);
        let num_samples = ST::from(self.window.get().num_samples()).unwrap();
        (sum / num_samples).sqrt()
    }
}

pub type RunningRMSProcessor = RunningRMSProcessorImpl<f32>;

/// An `AudioProcessor` which slides a window & calculates a running Squared sum of the input.
///
/// It exposes a `RunningRMSProcessorHandle` which may be called from any thread to get the current
/// RMS in real-time.
///
/// When the internal window's buffer needs to be resized, it's replaced via an atomic pointer swap.
pub struct RunningRMSProcessorImpl<ST: AtomicFloatRepresentable + Float> {
    handle: Shared<RunningRMSProcessorHandleImpl<ST>>,
    duration_samples: usize,
    duration: Duration,
    gc_handle: Handle,
}

impl<ST> RunningRMSProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync + Clone + 'static,
{
    /// Create a `RunningRMSProcessor` which will calculate RMS based on a certain `duration` of
    /// samples.
    pub fn new_with_duration(gc_handle: &Handle, duration: Duration) -> Self {
        let handle = Shared::new(gc_handle, RunningRMSProcessorHandleImpl::new(gc_handle));
        handle
            .duration_micros
            .store(duration.as_micros() as usize, Ordering::Relaxed);

        RunningRMSProcessorImpl {
            handle,
            duration_samples: 0,
            duration,
            gc_handle: gc_handle.clone(),
        }
    }

    pub fn from_handle(handle: Shared<RunningRMSProcessorHandleImpl<ST>>) -> Self {
        let duration = Duration::from_micros(handle.duration_micros.load(Ordering::Relaxed) as u64);
        Self {
            gc_handle: audio_garbage_collector::handle().clone(),
            handle,
            duration_samples: 0,
            duration,
        }
    }

    pub fn handle(&self) -> &Shared<RunningRMSProcessorHandleImpl<ST>> {
        &self.handle
    }
}

impl<ST> AudioProcessor for RunningRMSProcessorImpl<ST>
where
    ST: AtomicFloatRepresentable + Float + 'static,
    ST::AtomicType: Send + Sync + Clone + 'static,
{
    type SampleType = ST;

    fn prepare(&mut self, context: &mut AudioContext) {
        let settings = context.settings;
        self.duration_samples = (settings.sample_rate() * self.duration.as_secs_f32()) as usize;
        self.handle.resize(
            &self.gc_handle,
            settings.output_channels(),
            self.duration_samples,
        );
    }

    fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<Self::SampleType>) {
        if self.duration_samples == 0 {
            return;
        }

        for sample_index in 0..buffer.num_samples() {
            let running_sums = self.handle.running_sums.get();
            let window = self.handle.window.get();
            let mut cursor = self.handle.cursor();

            for channel_index in 0..buffer.num_channels() {
                let value_slot = window.get(channel_index, cursor);
                let previous_value = value_slot.get();

                let sample = *buffer.get(channel_index, sample_index);
                let new_value = sample * sample; // using square rather than abs is around 1% faster
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
}

#[cfg(test)]
mod test {
    use audio_garbage_collector::GarbageCollector;
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_create_handle() {
        let gc = GarbageCollector::default();
        let handle = RunningRMSProcessorHandle::new(gc.handle());

        assert_f_eq!(handle.calculate_rms(0), 0.0);
    }

    #[test]
    fn test_resize() {
        let gc = GarbageCollector::default();
        let handle = RunningRMSProcessorHandle::new(gc.handle());

        handle.resize(gc.handle(), 2, 1000);
        assert_eq!(handle.window.get().num_channels(), 2);
        assert_eq!(handle.window.get().num_samples(), 1000);
        assert_f_eq!(handle.calculate_rms(0), 0.0);
        assert_f_eq!(handle.calculate_rms(1), 0.0);
        assert_eq!(handle.cursor(), 0)
    }

    #[test]
    fn test_create_running_rms_processor() {
        let gc = GarbageCollector::default();
        let mut processor =
            RunningRMSProcessor::new_with_duration(gc.handle(), Duration::from_millis(10));

        let mut context = AudioContext::default();
        context.settings.sample_rate = 44100.0;
        processor.prepare(&mut context);

        assert_eq!(processor.duration_samples, 441);
        assert_eq!(processor.duration, Duration::from_millis(10));
        assert_eq!(
            processor.handle.duration_micros.load(Ordering::Relaxed),
            10_000
        );
    }

    #[test]
    fn test_create_running_rms_processor_from_handle() {
        let gc = GarbageCollector::default();
        let handle = RunningRMSProcessorHandle::new(gc.handle());
        handle.resize(gc.handle(), 2, 1000);
        let handle = Shared::new(gc.handle(), handle);
        let processor = RunningRMSProcessor::from_handle(handle.clone());

        assert_eq!(processor.duration_samples, 0);
        assert_eq!(processor.duration, Duration::from_micros(0));
        assert_eq!(processor.handle.duration_micros.load(Ordering::Relaxed), 0);
        assert_eq!(
            &*processor.handle().clone() as *const RunningRMSProcessorHandle,
            &*handle as *const RunningRMSProcessorHandle
        )
    }

    #[test]
    fn test_audio_process_running() {
        let gc = GarbageCollector::default();
        let mut processor =
            RunningRMSProcessor::new_with_duration(gc.handle(), Duration::from_millis(10));
        let mut test_buffer = AudioBuffer::empty();

        test_buffer.resize_with(2, 1000, || 1.0);
        let mut context = AudioContext::default();
        processor.prepare(&mut context);
        processor.process(&mut context, &mut test_buffer);
        let rms = processor.handle.calculate_rms(0);
        assert!(rms > 0.0);
    }
}
