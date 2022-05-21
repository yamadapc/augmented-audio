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

//! Provides implementations of some audio analysis processors.
//!
//! * **Peak detector** - [`peak_detector`]
//! * **FFT (Windowed/Overlapped)** - [`fft_processor`]
//! * **Transient detection** (not real-time) - [`transient_detection::stft`]
//! * **Window functions** - [`window_functions`]
//!
//! ## RMS
//! Real-time safe, per-sample (ticked by UI thread) RMS calculation.
//!
//! ## Peak detector
//! Peak detector with adjustable attack/release times.
//!
//! ## FFT
//! `rustfft` audio-processor, forwards or backwards, real-time safe, FFT.
//!
//! Applies a Hann window by default. Several window functions are exported by [`window_functions`].
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/window_functions/windows--HannWindow.png)
//!
//! Then performs FFT with N bins.
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/fft_processor.png--FFT_sine_440Hz.png)
//!
//! Overlap is configurable
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/screen.png)
//!
//! ## Envelope follower
//!
//! Envelope follower implementation with adjustable attack/release times.
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/audio-envelope.png)
//!
//! ## Transient detection
//!
//! Implements "[A Transient Detection Algorithm for Audio Using Iterative Analysis of STFT.](https://www.researchgate.net/profile/Balaji-Thoshkahna/publication/220723752_A_Transient_Detection_Algorithm_for_Audio_Using_Iterative_Analysis_of_STFT/links/0deec52e6331412aed000000/A-Transient-Detection-Algorithm-for-Audio-Using-Iterative-Analysis-of-STFT.pdf)".
//!
//! Does polyphonic transient detection, able to output signal or markers
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/audio-processor-analysis/src/transient_detection/stft.png)
//!
//! ## Window functions
//! Several window functions are implemented and configurable.

#[warn(missing_docs)]
pub mod envelope_follower_processor;

pub mod fft_processor;

/// Peak detector implementation
pub mod peak_detector;

/// RMS implementation suitable for GUI reacting to magnitude of the signal. Accumulates values on
/// a circular buffer, the consumer calculates the RMS value based on it.
pub mod running_rms_processor;

/// Polyphonic transient detection implementation
pub mod transient_detection;

/// Many window functions
pub mod window_functions;
