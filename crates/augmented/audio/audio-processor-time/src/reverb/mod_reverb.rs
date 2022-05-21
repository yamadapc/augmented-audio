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
use crate::MonoDelayProcessor;
use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandle, AudioProcessorHandleProvider, AudioProcessorHandleRef,
    ParameterSpec, ParameterValue,
};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor,
};
use rand::Rng;

fn shuffle(frame: &mut [f32]) {
    let mut rng = rand::thread_rng();
    for i in 0..frame.len() {
        frame[i] = frame[rng.gen_range(0..frame.len())];
    }
}

fn flip_polarities(frame: &mut [f32]) {
    for i in 0..frame.len() {
        frame[i] = -frame[i];
    }
}

struct ModReverbHandle {}

pub struct ModReverbProcessor {
    handle: Shared<ModReverbHandle>,
    delay_processor: [MonoDelayProcessor<f32>; 4],
}

struct GenericHandle(Shared<ModReverbHandle>);

impl AudioProcessorHandleProvider for ModReverbProcessor {
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(GenericHandle(self.handle.clone()))
    }
}

impl AudioProcessorHandle for GenericHandle {
    fn parameter_count(&self) -> usize {
        0
    }

    fn get_parameter_spec(&self, index: usize) -> ParameterSpec {
        todo!()
    }

    fn get_parameter(&self, index: usize) -> Option<ParameterValue> {
        todo!()
    }

    fn set_parameter(&self, index: usize, request: ParameterValue) {
        todo!()
    }
}

impl Default for ModReverbProcessor {
    fn default() -> Self {
        Self {
            handle: make_shared(ModReverbHandle {}),
            delay_processor: [
                MonoDelayProcessor::default(),
                MonoDelayProcessor::default(),
                MonoDelayProcessor::default(),
                MonoDelayProcessor::default(),
            ],
        }
    }
}

impl AudioProcessor for ModReverbProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        let mut base_delay = 0.2;
        for d in &mut self.delay_processor {
            d.s_prepare(settings);
            d.handle().set_delay_time_secs(base_delay);
            base_delay + 0.2;
            d.handle().set_feedback(0.4)
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            for (sample, delay_processor) in frame.iter_mut().zip(&mut self.delay_processor) {
                *sample = delay_processor.s_process(*sample);
            }
            shuffle(frame);
            flip_polarities(frame);
        }
    }
}
