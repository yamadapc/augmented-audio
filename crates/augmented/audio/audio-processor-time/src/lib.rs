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

//! [![crates.io](https://img.shields.io/crates/v/audio-processor-time.svg)](https://crates.io/crates/audio-processor-time)
//! [![docs.rs](https://docs.rs/audio-processor-time/badge.svg)](https://docs.rs/audio-processor-time/)
//! - - -
//! Time-based effects.
//!
//! Contains a mono delay processor implementation and a version of "FreeVerb".
//!
//! Also WIP implementations of a chorus processor and a modulated diffused reverb.
//!
//! # References
//! * FreeVerb - https://ccrma.stanford.edu/~jos/pasp/Freeverb.html
//! * "Let's Write a Reverb - Geraint Luff - ADC21" - https://www.youtube.com/watch?v=6ZK2Goiyotk
//! * "Audio Effects: Theory, Implementation and Application" - https://www.amazon.com/Audio-Effects-Theory-Implementation-Application/dp/1466560282

pub use mono_delay::*;
pub use reverb::*;

pub mod chorus;
pub mod mono_delay;
pub mod reverb;
