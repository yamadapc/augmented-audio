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

//! This is the `augmented` crate, containing a single entry-point for the Augmented Audio Libraries.
//! Please see <https://github.com/yamadapc/augmented-audio> for more information.
//! 
//! ## Overview
//! `augmented` exports utilities and audio processors aimed at making it easier to build audio
//! applications with rust from a common foundation.  These utilities are experimental and built
//! on spare time by one person who isn't an audio developer. [Your feedback is greatly appreciated](https://github.com/yamadapc/augmented-audio/issues/new).
//!
//! `augmented` is modular and all components exist in several small crates you can pick.
//!
//! ## Core concepts
//! Some concepts are shared between all the crates, for example, the [`audio::processor::AudioProcessor`] and
//! [`audio::processor::simple_processor::MonoAudioProcessor`] traits are how all audio nodes should be
//! implemented.
//!
//! Here's an example gain node:
//! ```
//! use audio_processor_traits::AudioContext;
//! use audio_processor_traits::simple_processor::MonoAudioProcessor;
//! struct ExampleGainProcessor {}
//! impl MonoAudioProcessor for ExampleGainProcessor {
//!     type SampleType = f32;
//!     fn m_process(&mut self, _context: &mut AudioContext, sample: Self::SampleType) -> Self::SampleType {
//!         sample * 0.2
//!     }
//! }
//! ```
//!
//! The two traits are two different ways of defining an [`audio::processor::AudioProcessor`].
//! The [`audio::processor::simple_processor::MonoAudioProcessor`] is a sample-by-sample (or frame by frame)
//! processor function, while [`audio::processor::AudioProcessor`] receives one
//! [`audio::processor::AudioBuffer`] at a time.
//!
//! The processors must implement a `prepare` (or `m_prepare`) method, which receives
//! [`audio::processor::AudioProcessorSettings`] to configure things like channel count or
//! sample-rate.
//!
//! For "simple processors", which are defined as sample-by-sample functions, the
//! [`audio_processor_traits::simple_processor::process_buffer`] and [`audio_processor_traits::BufferProcessor`]
//! handle conversion into a regular buffer processor.
//!
//! Once an audio-processor is implemented, it can use [`application::audio_processor_standalone`] to
//! create a CLI or VST app automatically.
//!
//! This processor can also be a node in an [`audio::processor::graph`].
//!
//! ## Memory management model
//! The crates rely on reference counted pointers to state that is shared between the audio-thread
//! and other threads. This is done with `basedrop::Shared`.
//!
//! The idea is shared state is behind `basedrop::Shared` pointers which are never dropped on the
//! audio-thread. Instead, their destructors are pushed onto a queue and a background thread handles
//! dropping these values.
//!
//! This background thread is managed through [`audio::gc`]. That is a global instance of this
//! system, which exports helpers such as [`audio::gc::make_shared`] to build smart pointers that will
//! be de-allocated by the global background thread.

/// [`vst`] is re-exported for convenience.
pub use vst;

/// Run [`audio::processor::AudioProcessor`]s and [`audio::processor::midi::MidiEventHandler`] as
/// stand-alone applications.
pub mod application;
/// Process audio and MIDI information by defining [`audio::processor::AudioProcessor`]s, using
/// [`audio::processor::utility`] built-in processors or building [`audio::processor::graph`]s. Handle
/// real-time shared memory through ref-counting with [`audio::gc`].
pub mod audio;
/// Data-structures
pub mod data;
/// DSP (filters)
pub mod dsp;
/// GUI utilities, subject to change, wrapping [`iced`]
pub mod gui;
/// Operational utilities
pub mod ops;
/// Testing helpers
pub mod testing;
