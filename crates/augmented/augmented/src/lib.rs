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
/// VST
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
