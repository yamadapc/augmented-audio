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

pub use num;
pub use num::Float;
pub use num::Zero;

pub use atomic_float::{AtomicF32, AtomicF64};
pub use audio_buffer::AudioBuffer;
pub use context::AudioContext;
pub use midi::{MidiEventHandler, MidiMessageLike, NoopMidiEventHandler};
pub use noop_processors::*;
pub use settings::*;

/// Atomic F32 implementation with `num` trait implementations
pub mod atomic_float;
/// Provides an abstraction for audio buffers that works for [`cpal`] and [`vst`] layouts
pub mod audio_buffer;
/// The "staged context" for audio processors
pub mod context;
/// Provides an abstraction for MIDI processing that works for stand-alone and [`vst`] events
pub mod midi;
/// Parameters for [`AudioProcessor`]
pub mod parameters;
/// Simpler audio processor trait, ingesting sample by sample
pub mod simple_processor;

mod noop_processors;
mod settings;

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over. See some [examples here](https://github.com/yamadapc/augmented-audio/tree/master/crates/augmented/application/audio-processor-standalone/examples).
pub trait AudioProcessor {
    type SampleType: Sized;

    /// Prepare for playback based on current audio settings
    fn prepare(&mut self, _context: &mut AudioContext) {}

    /// Process a block of samples by mutating the input `AudioBuffer`
    fn process(&mut self, _context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>);
}
