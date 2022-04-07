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
use vst::util::AtomicFloat;

use crate::audio_io::processor_handle_registry::ProcessorHandleRegistry;
use audio_garbage_collector::{Handle, Shared};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use circular_data_structures::CircularVec;
use std::time::Duration;

pub struct VolumeMeterProcessorHandle {
    pub volume_left: AtomicFloat,
    pub volume_right: AtomicFloat,
    pub peak_left: AtomicFloat,
    pub peak_right: AtomicFloat,
}

// TODO - this is quite a bad strategy ; running RMS processor is a nicer implementation of the same
// thing
pub struct VolumeMeterProcessor {
    handle: Shared<VolumeMeterProcessorHandle>,
    current_index: usize,
    buffer_duration: Duration,
    buffer_duration_samples: usize,
    left_buffer: CircularVec<f32>,
    right_buffer: CircularVec<f32>,
    running_sum_left: f32,
    running_sum_right: f32,
}

impl VolumeMeterProcessor {
    pub fn new(gc_handle: &Handle) -> Self {
        let handle = Shared::new(
            gc_handle,
            VolumeMeterProcessorHandle {
                volume_left: AtomicFloat::new(0.0),
                volume_right: AtomicFloat::new(0.0),
                peak_left: AtomicFloat::new(0.0),
                peak_right: AtomicFloat::new(0.0),
            },
        );
        ProcessorHandleRegistry::current().register("volume-processor", handle.clone());
        VolumeMeterProcessor {
            handle,
            current_index: 0,
            buffer_duration: Duration::from_millis(50),
            buffer_duration_samples: 512 * 4,
            running_sum_left: 0.0,
            running_sum_right: 0.0,
            left_buffer: CircularVec::with_size(512 * 4, 0.0),
            right_buffer: CircularVec::with_size(512 * 4, 0.0),
        }
    }

    pub fn handle(&self) -> &Shared<VolumeMeterProcessorHandle> {
        &self.handle
    }

    pub fn current_volume(&self) -> (f32, f32) {
        (
            self.handle.volume_left.get(),
            self.handle.volume_right.get(),
        )
    }
}

impl AudioProcessor for VolumeMeterProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        let duration_samples =
            (self.buffer_duration.as_secs_f32() * settings.sample_rate()) as usize;
        self.buffer_duration_samples = duration_samples;
        self.left_buffer.resize(duration_samples, 0.0);
        self.right_buffer.resize(duration_samples, 0.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame_index in 0..data.num_samples() {
            let old_left_value = self.left_buffer[self.current_index];
            let old_right_value = self.right_buffer[self.current_index];
            let left_value = *data.get(0, frame_index) * *data.get(0, frame_index);
            let right_value = *data.get(1, frame_index) * *data.get(0, frame_index);
            self.left_buffer[self.current_index] = left_value;
            self.right_buffer[self.current_index] = right_value;

            self.running_sum_left = (self.running_sum_left + left_value - old_left_value).max(0.0);
            self.running_sum_right =
                (self.running_sum_right + right_value - old_right_value).max(0.0);
            self.handle
                .volume_left
                .set((self.running_sum_left / (self.buffer_duration_samples as f32)).sqrt());
            self.handle
                .volume_right
                .set((self.running_sum_right / (self.buffer_duration_samples as f32)).sqrt());

            // // This should decay ; use "InterpolatedValue" / create an atomic version of it
            // if right_value > self.handle.peak_right.get() {
            //     self.handle.peak_right.set(right_value);
            // }
            // if left_value > self.handle.peak_left.get() {
            //     self.handle.peak_left.set(left_value);
            // }

            if self.current_index >= self.buffer_duration_samples {
                self.current_index = 0;
            } else {
                self.current_index += 1;
            }
        }
    }
}
