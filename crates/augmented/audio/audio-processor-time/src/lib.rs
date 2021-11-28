use audio_garbage_collector::Shared;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{AtomicF32, AudioProcessorSettings, Float};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

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
            feedback: AtomicF32::from(0.0),
            delay_time_secs: AtomicF32::from(0.8),
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
        let read_position = self.current_read_position.load(Ordering::Relaxed);
        let sample_rate = self.sample_rate.load(Ordering::Relaxed);
        let buffer_size = self.buffer_size.load(Ordering::Relaxed);
        self.current_write_position.store(
            (read_position + (value * sample_rate) as usize) % buffer_size,
            Ordering::Relaxed,
        );
    }
}

pub struct MonoDelayProcessor<Sample> {
    delay_buffer: Vec<Sample>,
    handle: Shared<MonoDelayProcessorHandle>,
    max_delay_time: Duration,
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
            // delay_time,
            // sample_rate: 44100.0,
            // current_write_position: (delay_time * 44100.0) as usize,
            // current_read_position: 0,
            // buffer_size: 0,
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
        let mut current_read_position = self.handle.current_read_position.load(Ordering::Relaxed);
        let mut current_write_position = self.handle.current_write_position.load(Ordering::Relaxed);

        let delay_output = self.delay_buffer[current_read_position];
        self.delay_buffer[current_write_position] =
            sample + delay_output * self.handle.feedback.get().into();

        current_read_position += 1;
        current_write_position += 1;

        let buffer_size = self.handle.buffer_size.load(Ordering::Relaxed);
        if current_read_position >= buffer_size {
            current_read_position = 0;
        }
        if current_write_position >= buffer_size {
            current_write_position = 0;
        }

        self.handle
            .current_read_position
            .store(current_read_position, Ordering::Relaxed);
        self.handle
            .current_write_position
            .store(current_write_position, Ordering::Relaxed);

        delay_output
    }
}
