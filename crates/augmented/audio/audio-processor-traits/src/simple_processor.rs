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
//! Provides what is in some cases a simpler form of expressing signal processing.

use crate::parameters::{AudioProcessorHandleProvider, AudioProcessorHandleRef};
use crate::{AudioBuffer, AudioContext, AudioProcessor, MidiEventHandler, MidiMessageLike, Zero};
use std::ops::{AddAssign, DivAssign};

pub trait MonoAudioProcessor {
    type SampleType: Copy;

    fn m_prepare(&mut self, _context: &mut AudioContext) {}
    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        sample: Self::SampleType,
    ) -> Self::SampleType {
        sample
    }
}

#[derive(Default)]
pub struct MonoCopyProcessor<Processor: MonoAudioProcessor> {
    processor: Processor,
}

impl<Processor: MonoAudioProcessor> MonoCopyProcessor<Processor> {
    pub fn new(processor: Processor) -> MonoCopyProcessor<Processor> {
        Self { processor }
    }

    pub fn processor(&self) -> &Processor {
        &self.processor
    }

    pub fn processor_mut(&mut self) -> &mut Processor {
        &mut self.processor
    }
}

impl<Processor: MonoAudioProcessor> AudioProcessor for MonoCopyProcessor<Processor>
where
    Processor::SampleType: Zero + AddAssign + DivAssign + From<f32>,
{
    type SampleType = Processor::SampleType;

    fn prepare(&mut self, context: &mut AudioContext) {
        self.processor.m_prepare(context);
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        for sample_num in 0..data.num_samples() {
            let mut mono_input = Self::SampleType::zero();
            for channel_num in 0..data.num_channels() {
                mono_input += *data.get(channel_num, sample_num);
            }
            mono_input /= Self::SampleType::from(data.num_channels() as f32);

            let output = self.processor.m_process(context, mono_input);

            for channel_num in 0..data.num_channels() {
                data.set(channel_num, sample_num, output);
            }
        }
    }
}

impl<Processor: MidiEventHandler + MonoAudioProcessor> MidiEventHandler
    for MonoCopyProcessor<Processor>
{
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        self.processor.process_midi_events(midi_messages)
    }
}

pub struct MultiChannel<Processor: MonoAudioProcessor> {
    processors: Vec<Processor>,
    factory: Box<dyn Fn() -> Processor + Send>,
}

impl<Processor: MonoAudioProcessor> MultiChannel<Processor> {
    pub fn new(factory: impl Fn() -> Processor + 'static + Send) -> MultiChannel<Processor> {
        Self {
            processors: vec![],
            factory: Box::new(factory),
        }
    }

    pub fn for_each(&mut self, mut f: impl FnMut(&mut Processor)) {
        for processor in &mut self.processors {
            f(processor);
        }
    }
}

impl<Processor: MonoAudioProcessor> AudioProcessor for MultiChannel<Processor> {
    type SampleType = Processor::SampleType;

    fn prepare(&mut self, context: &mut AudioContext) {
        self.processors = (0..context.settings.input_channels)
            .map(|_| (self.factory)())
            .collect();
        for processor in &mut self.processors {
            processor.m_prepare(context);
        }
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        for (channel, processor) in data.channels_mut().iter_mut().zip(&mut self.processors) {
            for sample in channel {
                *sample = processor.m_process(context, *sample);
            }
        }
    }
}

impl<Processor: AudioProcessorHandleProvider + MonoAudioProcessor> AudioProcessorHandleProvider
    for MultiChannel<Processor>
{
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        self.processors[0].generic_handle()
    }
}

pub fn process_buffer<Processor, SampleType>(
    context: &mut AudioContext,
    processor: &mut Processor,
    signal: &mut AudioBuffer<SampleType>,
) where
    Processor: MonoAudioProcessor<SampleType = SampleType>,
    SampleType: Copy + AddAssign + num::Zero,
{
    for sample_num in 0..signal.num_samples() {
        let mut mono_input = SampleType::zero();
        for channel_num in 0..signal.num_channels() {
            mono_input += *signal.get(channel_num, sample_num);
        }

        let output = processor.m_process(context, mono_input);

        for channel_num in 0..signal.num_channels() {
            signal.set(channel_num, sample_num, output);
        }
    }
}
