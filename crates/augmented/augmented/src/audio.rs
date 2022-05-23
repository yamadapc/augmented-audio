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
pub use cpal;

pub use audio_garbage_collector as gc;
pub use audio_parameter_store as parameter_store;
pub use augmented_adsr_envelope as adsr_envelope;
pub use augmented_oscillator as oscillator;

/// Audio processor implementations
pub mod processor {
    #[doc(inline)]
    pub use audio_processor_analysis as analysis;
    #[doc(inline)]
    pub use audio_processor_bitcrusher as bitcrusher;
    #[doc(inline)]
    pub use audio_processor_dynamics as dynamics;
    #[doc(inline)]
    pub use audio_processor_file as file;
    #[doc(inline)]
    pub use audio_processor_graph as graph;
    #[doc(inline)]
    pub use audio_processor_metronome as metronome;
    #[doc(inline)]
    pub use audio_processor_pitch_shifter as pitch_shifter;
    #[doc(inline)]
    pub use audio_processor_time as time;
    #[doc(inline)]
    pub use audio_processor_traits::*;
    #[doc(inline)]
    pub use audio_processor_utility as utility;
}
