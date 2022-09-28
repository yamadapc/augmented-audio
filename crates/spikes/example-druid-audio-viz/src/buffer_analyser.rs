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
use atomic_queue::Queue;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use basedrop::{Handle, Shared};

pub struct BufferAnalyserProcessor {
    buffer: Shared<Queue<f32>>,
}

impl BufferAnalyserProcessor {
    pub fn new(handle: &Handle) -> Self {
        BufferAnalyserProcessor {
            buffer: Shared::new(handle, Queue::new((5. * 4410.0) as usize)),
        }
    }

    pub fn queue(&self) -> Shared<Queue<f32>> {
        self.buffer.clone()
    }
}

impl AudioProcessor for BufferAnalyserProcessor {
    type SampleType = f32;

    fn prepare(&mut self, _settings: AudioProcessorSettings) {
        // assert_eq!(settings.sample_rate(), 44100.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            self.buffer.push(frame[0]);
            frame.iter_mut().for_each(|s| *s = 0.0);
        }
    }
}
