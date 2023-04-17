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

use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};

struct SimpleDelayProcessor {
    current_write_position: usize,
    current_read_positions: Vec<(usize, usize)>,
    buffer_size: usize,
    delay_buffers: Vec<Vec<f32>>,
}

impl SimpleDelayProcessor {
    fn new() -> Self {
        let num_reads = 5;
        let delay_time = 1000.0;
        let tap_increment_right = 0.0 / num_reads as f32;
        let tap_increment_left = 500.0 / num_reads as f32;

        let write_position =
            (Duration::from_millis(delay_time as u64).as_secs_f32() * 44100.0) as usize;
        let read_positions: Vec<(usize, usize)> = (0..num_reads)
            .map(|i| {
                let tap_time_left =
                    tap_increment_left + tap_increment_left * (i as f32 / num_reads as f32);
                let tap_time_right =
                    tap_increment_right + tap_increment_right * (i as f32 / num_reads as f32);
                (
                    (Duration::from_secs_f32(tap_time_left / 1000.0).as_secs_f32() * 44100.0)
                        as usize,
                    (Duration::from_secs_f32(tap_time_right / 1000.0).as_secs_f32() * 44100.0)
                        as usize,
                )
            })
            .collect();

        for pos in &read_positions {
            if pos.0 >= write_position {
                panic!("Delay will cause feedback loop");
            }
            if pos.1 >= write_position {
                panic!("Delay will cause feedback loop");
            }
        }

        let max_delay_time = (Duration::from_secs(10).as_secs_f32() * 44100.0) as usize;
        Self {
            current_write_position: write_position,
            current_read_positions: read_positions,
            buffer_size: max_delay_time,
            delay_buffers: vec![
                Self::make_vec(max_delay_time),
                Self::make_vec(max_delay_time),
            ],
        }
    }

    fn make_vec(max_delay_time: usize) -> Vec<f32> {
        let mut v = Vec::new();
        v.resize(max_delay_time, 0.0);
        v
    }
}

impl AudioProcessor for SimpleDelayProcessor {
    type SampleType = f32;

    fn process(&mut self, _context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        // Mono input stage
        for sample_index in 0..data.num_samples() {
            data.set(0, sample_index, *data.get(1, sample_index));
        }

        let num_read_positions = self.delay_buffers.len() as f32;
        // Delay read/write
        for sample_index in 0..data.num_samples() {
            for channel_index in 0..data.num_channels() {
                let input = *data.get(channel_index, sample_index);

                // Read delay output
                let delay_output: f32 = self
                    .current_read_positions
                    .iter()
                    .enumerate()
                    .map(|(index, read_position)| {
                        let read_index = if channel_index == 0 {
                            read_position.0
                        } else {
                            read_position.1
                        };

                        let volume = (1.0 - index as f32) / num_read_positions;

                        volume * self.delay_buffers[channel_index][read_index]
                    })
                    .sum();
                let delay_output = 0.4 * delay_output / num_read_positions;

                // Write input
                let feedback = delay_output * 0.3;
                self.delay_buffers[channel_index][self.current_write_position] = input + feedback;

                // Output stage
                data.set(channel_index, sample_index, input + delay_output);
            }

            for pos in &mut self.current_read_positions {
                let mut p0 = pos.0 + 1;
                let mut p1 = pos.1 + 1;
                if p0 >= self.buffer_size {
                    p0 = 0;
                }
                if p1 >= self.buffer_size {
                    p1 = 0;
                }
                *pos = (p0, p1);
            }
            self.current_write_position += 1;
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
