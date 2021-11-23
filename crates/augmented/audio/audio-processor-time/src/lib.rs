use audio_garbage_collector::Shared;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{AtomicF32, AudioProcessorSettings, Float};
use std::time::Duration;

pub struct MonoDelayProcessorHandle {
    feedback: AtomicF32,
    delay_time_secs: AtomicF32,
}

impl Default for MonoDelayProcessorHandle {
    fn default() -> Self {
        Self {
            feedback: AtomicF32::from(0.0),
            delay_time_secs: AtomicF32::from(0.8),
        }
    }
}

impl MonoDelayProcessorHandle {
    pub fn set_feedback(&self, value: f32) {
        self.feedback.set(value);
    }

    pub fn set_delay_time_secs(&self, value: f32) {
        self.delay_time_secs.set(value);
    }
}

pub struct MonoDelayProcessor<Sample> {
    delay_buffer: Vec<Sample>,
    current_write_position: usize,
    current_read_position: usize,
    delay_time: f32,
    handle: Shared<MonoDelayProcessorHandle>,
    sample_rate: f32,
    max_delay_time: Duration,
    buffer_size: usize,
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
        let delay_time = Duration::from_millis(800).as_secs_f32();

        Self {
            handle,
            sample_rate: 44100.0,
            max_delay_time,
            delay_time,
            current_write_position: (delay_time * 44100.0) as usize,
            current_read_position: 0,
            delay_buffer: Self::make_vec(0),
            buffer_size: 0,
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

        self.sample_rate = settings.sample_rate();
        self.buffer_size = buffer_size;
        self.delay_buffer.resize(buffer_size, 0.0.into());
        self.current_write_position =
            (self.handle.delay_time_secs.get() * settings.sample_rate()) as usize;
        self.current_read_position = 0;
    }

    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        if self.delay_time - self.handle.delay_time_secs.get() > f32::EPSILON {
            self.delay_time = self.handle.delay_time_secs.get();
            self.current_write_position = (self.current_read_position
                + (self.delay_time * self.sample_rate) as usize)
                % self.buffer_size;
        }

        let delay_output = self.delay_buffer[self.current_read_position];
        self.delay_buffer[self.current_write_position] =
            sample + delay_output * self.handle.feedback.get().into();

        self.current_read_position += 1;
        self.current_write_position += 1;
        if self.current_read_position >= self.buffer_size {
            self.current_read_position = 0;
        }
        if self.current_write_position >= self.buffer_size {
            self.current_write_position = 0;
        }

        delay_output
    }
}
