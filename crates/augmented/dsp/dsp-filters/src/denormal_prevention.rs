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
use num::Float;

static VERY_SMALL_AMOUNT: f64 = 1e-8;

/// Hack to prevent denormals
pub struct DenormalPrevention<Sample: Float> {
    value: Sample,
}

impl<Sample: Float> Default for DenormalPrevention<Sample> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Sample: Float> DenormalPrevention<Sample> {
    pub fn new() -> Self {
        DenormalPrevention {
            value: Sample::from(VERY_SMALL_AMOUNT).unwrap(),
        }
    }
}

impl<Sample: Float> DenormalPrevention<Sample> {
    #[inline]
    pub fn alternating_current(&mut self) -> Sample {
        self.value = -self.value;
        self.value
    }

    #[inline]
    pub fn direct_current(&mut self) -> Sample {
        Sample::from(VERY_SMALL_AMOUNT).unwrap()
    }
}
