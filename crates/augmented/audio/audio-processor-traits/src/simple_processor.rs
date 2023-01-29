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
//!
//! The [`SimpleAudioProcessor`] is essentially a function of `f32` to `f32` (or a
//! function that takes a multi-channel `frame` of `f32`s and mutates it.
//!
//! Additionally, [`process_buffer`] and [`BufferProcessor`] are exposed to "lift" a
//! [`SimpleAudioProcessor`] onto a buffered [`AudioProcessor`] instance.

use crate::parameters::{AudioProcessorHandleProvider, AudioProcessorHandleRef};
use crate::{
    AudioBuffer, AudioContext, AudioProcessor, AudioProcessorSettings, MidiEventHandler,
    MidiMessageLike,
};
use num::traits::FloatConst;
use num::Float;
use std::ops::Deref;

pub trait MonoAudioProcessor {
    type SampleType: Copy;

    fn m_prepare(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}
    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        sample: Self::SampleType,
    ) -> Self::SampleType {
        sample
    }
}

impl<M, ST> SimpleAudioProcessor for M
where
    M: MonoAudioProcessor<SampleType = ST>,
    ST: Copy + std::iter::Sum<ST> + Float + FloatConst,
{
    type SampleType = M::SampleType;

    fn s_prepare(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        self.m_prepare(context, settings);
    }

    fn s_process_frame(&mut self, context: &mut AudioContext, frame: &mut [Self::SampleType]) {
        let sum = frame.iter().cloned().sum();
        let output =
            self.m_process(context, sum) / ST::from(frame.len()).unwrap_or_else(|| ST::one());

        for sample in frame.iter_mut() {
            *sample = output;
        }
    }
}

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over.
pub trait SimpleAudioProcessor {
    type SampleType: Copy;

    /// Prepare for playback based on current audio settings
    fn s_prepare(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}

    /// Process a multi-channel frame.
    fn s_process_frame(&mut self, _context: &mut AudioContext, _frame: &mut [Self::SampleType]) {}
}

/// Wrapper over `SimpleAudioProcessor` to provide an `AudioProcessor` impl.
#[derive(Clone, Default, Debug)]
pub struct BufferProcessor<Processor>(pub Processor);

/// Process a buffer of samples with a `SimpleAudioProcessor`
#[inline]
pub fn process_buffer<Processor, BufferType>(
    context: &mut AudioContext,
    processor: &mut Processor,
    data: &mut BufferType,
) where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
    BufferType: AudioBuffer<SampleType = Processor::SampleType>,
{
    for frame in data.frames_mut() {
        processor.s_process_frame(context, frame);
    }
}

impl<Processor> Deref for BufferProcessor<Processor> {
    type Target = Processor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Processor> AudioProcessorHandleProvider for BufferProcessor<Processor>
where
    Processor: AudioProcessorHandleProvider,
{
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        self.0.generic_handle()
    }
}

impl<Processor> AudioProcessor for BufferProcessor<Processor>
where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
{
    type SampleType = <Processor as SimpleAudioProcessor>::SampleType;

    fn prepare(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        self.0.s_prepare(context, settings);
    }

    #[inline]
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        context: &mut AudioContext,
        data: &mut BufferType,
    ) {
        process_buffer(context, &mut self.0, data)
    }
}

impl<Processor> MidiEventHandler for BufferProcessor<Processor>
where
    Processor: MidiEventHandler,
{
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        self.0.process_midi_events(midi_messages)
    }
}
