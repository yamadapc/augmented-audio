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

use audio_garbage_collector::Shared;
use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{AtomicF32, AudioProcessorSettings, Float};
use augmented_atomics::AtomicValue;
use generic_handle::GenericHandle;

mod generic_handle;

pub struct MonoDelayProcessorHandle {
    feedback: AtomicF32,
    delay_time_secs: AtomicF32,
    current_write_position: AtomicUsize,
    current_read_position: AtomicUsize,
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
}

impl Default for MonoDelayProcessorHandle {
    fn default() -> Self {
        Self {
            feedback: AtomicF32::from(0.3),
            delay_time_secs: AtomicF32::from(0.2),
            current_write_position: AtomicUsize::new(0),
            sample_rate: AtomicF32::new(44100.0),
            buffer_size: AtomicUsize::new(1),
            current_read_position: AtomicUsize::new(0),
        }
    }
}

impl MonoDelayProcessorHandle {
    pub fn set_feedback(&self, value: f32) {
        self.feedback.set(value);
    }

    pub fn set_delay_time_secs(&self, value: f32) {
        self.delay_time_secs.set(value);

        let write_position = self.current_write_position.get();
        let sample_rate = self.sample_rate.get();
        let buffer_size = self.buffer_size.get();
        self.current_read_position.store(
            (write_position - (value * sample_rate) as usize + buffer_size) % buffer_size,
            Ordering::Relaxed,
        );
    }
}

pub struct MonoDelayProcessor<Sample> {
    delay_buffer: Vec<Sample>,
    handle: Shared<MonoDelayProcessorHandle>,
    max_delay_time: Duration,
}

impl<Sample> AudioProcessorHandleProvider for MonoDelayProcessor<Sample> {
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(GenericHandle(self.handle.clone()))
    }
}

impl<Sample: Float + From<f32>> Default for MonoDelayProcessor<Sample> {
    fn default() -> Self {
        Self::default_with_handle(audio_garbage_collector::handle())
    }
}

impl<Sample: Float + From<f32>> MonoDelayProcessor<Sample> {
    pub fn default_with_handle(handle: &audio_garbage_collector::Handle) -> Self {
        let max_delay_time = Duration::from_secs(5);
        let processor_handle = Shared::new(handle, MonoDelayProcessorHandle::default());

        Self::new(max_delay_time, processor_handle)
    }

    pub fn new(max_delay_time: Duration, handle: Shared<MonoDelayProcessorHandle>) -> Self {
        Self {
            handle,
            max_delay_time,
            delay_buffer: Self::make_vec(max_delay_time.as_secs() as usize),
        }
    }

    pub fn handle(&self) -> &Shared<MonoDelayProcessorHandle> {
        &self.handle
    }

    fn make_vec(max_delay_time: usize) -> Vec<Sample> {
        let mut v = Vec::with_capacity(max_delay_time);
        v.resize(max_delay_time, 0.0.into());
        v
    }

    pub fn read(&self) -> Sample {
        let delay_samples = self.delay_samples();
        let offset = delay_samples - delay_samples.floor();
        let offset: Sample = offset.into();
        let buffer_size = self.handle.buffer_size.get();

        let mut current_read_position = self.handle().current_read_position.get();
        let delay_output = interpolate(
            self.delay_buffer[current_read_position],
            self.delay_buffer[(current_read_position + 1) % buffer_size],
            offset,
        );

        current_read_position += 1;
        if current_read_position >= buffer_size {
            current_read_position = 0;
        }
        self.handle
            .current_read_position
            .store(current_read_position, Ordering::Relaxed);

        delay_output
    }

    pub fn write(&mut self, sample: Sample) {
        let mut current_write_position = self.handle().current_write_position.get();
        self.delay_buffer[current_write_position] = sample;

        current_write_position += 1;
        if current_write_position >= self.handle.buffer_size.get() {
            current_write_position = 0;
        }
        self.handle
            .current_write_position
            .store(current_write_position, Ordering::Relaxed);
    }

    fn delay_samples(&self) -> f32 {
        self.handle.delay_time_secs.get() * self.handle.sample_rate.get()
    }
}

fn interpolate<S>(s1: S, s2: S, offset: S) -> S
where
    S: Float + From<f32>,
{
    let one: S = 1.0_f32.into();
    let offset: S = offset;
    let rhs = offset * s2;
    let lhs = (one - offset) * s1;
    // assert!(rhs < S::epsilon());
    // assert!(lhs - s1 < S::epsilon());
    lhs + rhs
}

impl<Sample: Float + From<f32>> SimpleAudioProcessor for MonoDelayProcessor<Sample> {
    type SampleType = Sample;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        let buffer_size = (self.max_delay_time.as_secs_f32() * settings.sample_rate()) as usize;

        self.handle
            .buffer_size
            .store(buffer_size, Ordering::Relaxed);
        self.delay_buffer.resize(buffer_size, 0.0.into());
        self.handle
            .sample_rate
            .store(settings.sample_rate(), Ordering::Relaxed);

        self.handle.current_write_position.store(
            (self.handle.delay_time_secs.get() * settings.sample_rate()) as usize,
            Ordering::Relaxed,
        );
        self.handle
            .current_read_position
            .store(0, Ordering::Relaxed);
    }

    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        let delay_output = self.read();

        let write_sample = sample + delay_output * self.handle.feedback.get().into();
        self.write(write_sample);

        delay_output
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        let sample = frame[0];
        let delay_output = self.s_process(sample);
        frame[0] = delay_output;
        frame[1] = delay_output;
    }
}
