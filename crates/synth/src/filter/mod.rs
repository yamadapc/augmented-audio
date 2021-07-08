mod cascade;
mod pole_filter;
mod rbj;

use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

/// This module is a port of https://github.com/vinniefalco/DSPFilters/

pub struct LowPassFilterProcessor {
    filter_left: rbj::LowPassFilter<f32>,
    sample_rate: f32,
    cutoff: f32,
    q: f32,
}

impl LowPassFilterProcessor {
    pub fn new() -> Self {
        Self {
            filter_left: rbj::LowPassFilter::new(),
            sample_rate: 44100.0,
            cutoff: 880.0,
            q: 1.0,
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
        self.filter_left
            .setup(self.sample_rate, self.cutoff, self.q);
    }

    pub fn set_q(&mut self, q: f32) {
        self.q = q;
        self.filter_left
            .setup(self.sample_rate, self.cutoff, self.q);
    }
}

impl AudioProcessor for LowPassFilterProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.sample_rate = settings.sample_rate();
        self.filter_left
            .setup(self.sample_rate, self.cutoff, self.q);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.filter_left.process_channel(data, 0);

        // Mono output
        for sample_index in 0..data.num_samples() {
            let left_output = *data.get(0, sample_index);
            for channel_index in 1..data.num_channels() {
                data.set(channel_index, sample_index, left_output);
            }
        }
    }
}
