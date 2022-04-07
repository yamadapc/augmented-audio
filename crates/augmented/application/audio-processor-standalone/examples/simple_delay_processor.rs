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
use std::time::Duration;

use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct SimpleDelayProcessor {
    current_write_position: usize,
    current_read_position: usize,
    delay_buffers: Vec<Vec<f32>>,
    buffer_size: usize,
}

impl SimpleDelayProcessor {
    fn new() -> Self {
        let max_delay_time = (Duration::from_secs(5).as_secs_f32() * 44100.0) as usize;

        Self {
            current_write_position: (Duration::from_millis(800).as_secs_f32() * 44100.0) as usize,
            current_read_position: 0,
            delay_buffers: vec![
                Self::make_vec(max_delay_time),
                Self::make_vec(max_delay_time),
            ],
            buffer_size: max_delay_time,
        }
    }

    fn make_vec(max_delay_time: usize) -> Vec<f32> {
        let mut v = Vec::with_capacity(max_delay_time);
        v.resize(max_delay_time, 0.0);
        v
    }
}

impl AudioProcessor for SimpleDelayProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        // Mono input stage
        for sample_index in 0..data.num_samples() {
            data.set(0, sample_index, *data.get(1, sample_index));
        }

        // Delay read/write
        for sample_index in 0..data.num_samples() {
            for channel_index in 0..data.num_channels() {
                let input = *data.get(channel_index, sample_index);

                // Read delay output
                let delay_output = self.delay_buffers[channel_index][self.current_read_position];

                // Write input into delay with feedback
                let feedback = 0.3;
                self.delay_buffers[channel_index][self.current_write_position] =
                    input + delay_output * feedback;

                // Output stage
                data.set(channel_index, sample_index, input + delay_output);
            }

            self.current_read_position += 1;
            self.current_write_position += 1;
            if self.current_read_position >= self.buffer_size {
                self.current_read_position = 0;
            }
            if self.current_write_position >= self.buffer_size {
                self.current_write_position = 0;
            }
        }
    }
}

fn main() {
    let processor = SimpleDelayProcessor::new();
    audio_processor_standalone::audio_processor_main(processor);
}
