# audio-processor-standalone
[![crates.io](https://img.shields.io/crates/v/audio-processor-standalone.svg)](https://crates.io/crates/audio-processor-standalone)
[![docs.rs](https://docs.rs/audio-processor-standalone/badge.svg)](https://docs.rs/audio-processor-standalone/)
- - -

Provides a stand-alone audio-processor runner for `AudioProcessor` implementations.

```rust
// Imports
use std::time::Duration;

use audio_processor_traits::{AudioBuffer, AudioProcessor};
use circular_data_structures::CircularVec;

// Declare a delay `audio_processor_traits::AudioProcessor` implementation
struct SimpleDelayProcessor {
    current_write_position: usize,
    current_read_position: usize,
    delay_buffers: Vec<CircularVec<f32>>,
}

impl SimpleDelayProcessor {
    fn new() -> Self {
        Self {
            current_write_position: (Duration::from_millis(800).as_secs_f32() * 44100.0) as usize,
            current_read_position: 0,
            delay_buffers: vec![
                CircularVec::with_size(
                    (Duration::from_secs(5).as_secs_f32() * 44100.0) as usize,
                    0.0,
                ),
                CircularVec::with_size(
                    (Duration::from_secs(5).as_secs_f32() * 44100.0) as usize,
                    0.0,
                ),
            ],
        }
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
        }
    }
}

// Run it
fn main() {
    let processor = SimpleDelayProcessor::new();
    audio_processor_standalone::audio_processor_main(processor);
}
```
