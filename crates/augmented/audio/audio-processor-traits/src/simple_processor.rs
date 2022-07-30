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
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use std::ops::Deref;

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over.
pub trait SimpleAudioProcessor {
    type SampleType: Copy;

    /// Prepare for playback based on current audio settings
    fn s_prepare(&mut self, _settings: AudioProcessorSettings) {}

    /// Process a single sample. If the input is mult-channel, will run for each channel by default.
    /// If the processor is multi-channel, implement s_process_frame instead.
    ///
    /// `s_process_frame` is what should be called by consumers & its not required to implement a
    /// sound `s_process` method.
    fn s_process(&mut self, sample: Self::SampleType) -> Self::SampleType {
        sample
    }

    /// Process a multi-channel frame.
    ///
    /// By default calls s_process.
    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        for sample in frame {
            *sample = self.s_process(*sample);
        }
    }
}

/// Wrapper over `SimpleAudioProcessor` to provide an `AudioProcessor` impl.
#[derive(Clone, Default, Debug)]
pub struct BufferProcessor<Processor>(pub Processor);

/// Process a buffer of samples with a `SimpleAudioProcessor`
#[inline]
pub fn process_buffer<Processor, BufferType>(processor: &mut Processor, data: &mut BufferType)
where
    Processor: SimpleAudioProcessor,
    <Processor as SimpleAudioProcessor>::SampleType: Copy,
    BufferType: AudioBuffer<SampleType = Processor::SampleType>,
{
    for frame in data.frames_mut() {
        processor.s_process_frame(frame);
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

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.0.s_prepare(settings);
    }

    #[inline]
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        process_buffer(&mut self.0, data)
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
