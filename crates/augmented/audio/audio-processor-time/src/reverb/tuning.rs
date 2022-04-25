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
pub const STEREO_SPREAD: usize = 23;

// pub const MUTED: f32 = 0.0;
pub const FIXED_GAIN: f32 = 0.015;
pub const SCALE_WET: f32 = 3.0;
pub const SCALE_DRY: f32 = 2.0;
pub const SCALE_DAMP: f32 = 0.4;
pub const SCALE_ROOM: f32 = 0.28;
pub const OFFSET_ROOM: f32 = 0.7;
pub const INITIAL_ROOM: f32 = 0.5;
pub const INITIAL_DAMP: f32 = 0.5;
pub const INITIAL_WET: f32 = 1.0 / SCALE_WET;
pub const INITIAL_DRY: f32 = 0.0;
pub const INITIAL_WIDTH: f32 = 1.0;

// pub const INITIALMODE: f32 = 0.0;
// pub const FREEZEMODE: f32 = 0.5;

pub const ALL_PASS_TUNING: usize = 556;
pub const ALLPASSTUNING_R1: usize = 556 + STEREO_SPREAD;
pub const ALLPASSTUNING_L2: usize = 441;
pub const ALLPASSTUNING_R2: usize = 441 + STEREO_SPREAD;
pub const ALLPASSTUNING_L3: usize = 341;
pub const ALLPASSTUNING_R3: usize = 341 + STEREO_SPREAD;
pub const ALLPASSTUNING_L4: usize = 225;
pub const ALLPASSTUNING_R4: usize = 225 + STEREO_SPREAD;

pub const COMBTUNING_L1: usize = 1116;
pub const COMBTUNING_R1: usize = 1116 + STEREO_SPREAD;
pub const COMBTUNING_L2: usize = 1188;
pub const COMBTUNING_R2: usize = 1188 + STEREO_SPREAD;
pub const COMBTUNING_L3: usize = 1277;
pub const COMBTUNING_R3: usize = 1277 + STEREO_SPREAD;
pub const COMBTUNING_L4: usize = 1356;
pub const COMBTUNING_R4: usize = 1356 + STEREO_SPREAD;
pub const COMBTUNING_L5: usize = 1422;
pub const COMBTUNING_R5: usize = 1422 + STEREO_SPREAD;
pub const COMBTUNING_L6: usize = 1491;
pub const COMBTUNING_R6: usize = 1491 + STEREO_SPREAD;
pub const COMBTUNING_L7: usize = 1557;
pub const COMBTUNING_R7: usize = 1557 + STEREO_SPREAD;
pub const COMBTUNING_L8: usize = 1617;
pub const COMBTUNING_R8: usize = 1617 + STEREO_SPREAD;
