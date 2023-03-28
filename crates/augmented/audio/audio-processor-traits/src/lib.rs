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
//! Provides abstractions for implementing:
//!
//! * Audio processing nodes
//! * MIDI processing nodes
//! * Audio buffers
//!
//! An audio processor implemented with these traits may work with multiple sample types, audio
//! buffer types and audio processing back-ends.
//!
//! Start looking at [AudioProcessor], then have a look at [AudioBuffer] and [MidiEventHandler].
//!
//! # Running the audio processors as CLIs, GUIs, VSTs or CPAL threads
//! See [`audio_processor_standalone`]
//!
//! # Running a graph of audio processors
//! See [`audio_processor_graph`]
//!
//! # AudioProcessor implementations
//! See: <https://github.com/yamadapc/augmented-audio/>
//!
use std::marker::PhantomData;

pub use num;
pub use num::Float;
pub use num::Zero;

pub use atomic_float::{AtomicF32, AtomicF64};
pub use audio_buffer::{AudioBuffer, InterleavedAudioBuffer, OwnedAudioBuffer, VecAudioBuffer};
pub use context::AudioContext;
pub use midi::{MidiEventHandler, MidiMessageLike, NoopMidiEventHandler};
pub use settings::*;
pub use simple_processor::{BufferProcessor, SimpleAudioProcessor};

/// Atomic F32 implementation with `num` trait implementations
pub mod atomic_float;
/// Provides an abstraction for audio buffers that works for [`cpal`] and [`vst`] layouts
pub mod audio_buffer;
pub mod combinators;
/// The "staged context" for audio processors
pub mod context;
/// Provides an abstraction for MIDI processing that works for stand-alone and [`vst`] events
pub mod midi;
/// Parameters for [`AudioProcessor`]
pub mod parameters;
/// Simpler audio processor trait, ingesting sample by sample
pub mod simple_processor;

mod settings;

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over. See some [examples here](https://github.com/yamadapc/augmented-audio/tree/master/crates/augmented/application/audio-processor-standalone/examples).
pub trait AudioProcessor {
    type SampleType;

    /// Prepare for playback based on current audio settings
    fn prepare(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}

    /// Process a block of samples by mutating the input `AudioBuffer`
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _context: &mut AudioContext,
        data: &mut BufferType,
    );
}

/// Auto-implemented object version of the audio-processor trait.
///
/// Given a known buffer-type, audio-processors can be made into objects using this type.
pub trait SliceAudioProcessor {
    fn prepare_slice(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}
    fn process_slice(&mut self, _context: &mut AudioContext, num_channels: usize, data: &mut [f32]);
}

impl<Processor> SliceAudioProcessor for Processor
where
    Processor: AudioProcessor<SampleType = f32>,
{
    fn prepare_slice(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        <Processor as AudioProcessor>::prepare(self, context, settings);
    }

    fn process_slice(&mut self, context: &mut AudioContext, num_channels: usize, data: &mut [f32]) {
        let mut buffer = InterleavedAudioBuffer::new(num_channels, data);
        <Processor as AudioProcessor>::process(self, context, &mut buffer);
    }
}

/// Auto-implemented object version of the audio-processor trait.
///
/// Given a known buffer-type, audio-processors can be made into objects using this type.
pub trait ObjectAudioProcessor<BufferType> {
    fn prepare_obj(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}
    fn process_obj(&mut self, context: &mut AudioContext, data: &mut BufferType);
}

impl<SampleType, BufferType, Processor> ObjectAudioProcessor<BufferType> for Processor
where
    SampleType: Float + Send,
    BufferType: AudioBuffer<SampleType = SampleType>,
    Processor: AudioProcessor<SampleType = SampleType>,
{
    fn prepare_obj(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        <Processor as AudioProcessor>::prepare(self, context, settings);
    }

    fn process_obj(&mut self, context: &mut AudioContext, data: &mut BufferType) {
        <Processor as AudioProcessor>::process(self, context, data);
    }
}

/// An audio-processor which doesn't do any work.
pub struct NoopAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> Default for NoopAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType> NoopAudioProcessor<SampleType> {
    pub fn new() -> Self {
        NoopAudioProcessor(PhantomData::default())
    }
}

impl<SampleType: Send + Copy> SimpleAudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;
    fn s_process_frame(&mut self, _context: &mut AudioContext, _frame: &mut [SampleType]) {}
}

/// An audio-processor which mutes all channels.
pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> SilenceAudioProcessor<SampleType> {
    pub fn new() -> Self {
        SilenceAudioProcessor(PhantomData)
    }
}

impl<SampleType> Default for SilenceAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType: Float + Send> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _context: &mut AudioContext,
        output: &mut BufferType,
    ) {
        for sample in output.slice_mut() {
            *sample = SampleType::zero();
        }
    }
}
