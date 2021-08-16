use std::time::Duration;

use circular_data_structures::CircularVec;

pub struct InternalBuffer<SampleType> {
    size: usize,
    channels: Vec<CircularVec<SampleType>>,
}

impl<SampleType: num::Float> InternalBuffer<SampleType> {
    pub fn new() -> Self {
        InternalBuffer {
            channels: Vec::new(),
            size: 0,
        }
    }

    pub fn num_samples(&self) -> usize {
        self.size
    }

    pub fn channel(&mut self, channel_index: usize) -> &mut CircularVec<SampleType> {
        &mut self.channels[channel_index]
    }

    pub fn resize(&mut self, num_channels: usize, sample_rate: f32, duration: Duration) {
        let duration_samples = (duration.as_secs_f32() * sample_rate) as usize;
        self.size = duration_samples;
        self.channels.clear();
        for _channel in 0..num_channels {
            let channel_buffer = CircularVec::with_size(duration_samples, SampleType::zero());
            self.channels.push(channel_buffer);
        }
    }
}
