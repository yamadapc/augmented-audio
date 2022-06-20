use crate::{LooperEngine, LooperId};
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
use crate::parameters::{LFOMode, LFOParameter, ParameterId};

/// Change LFO parameters on a given track and LFO path
#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_lfo_parameter(
    engine: *const LooperEngine,
    track_id: usize,
    lfo: usize,
    parameter_id: LFOParameter,
    value: f32,
) {
    let engine = &(*engine);
    engine
        .handle()
        .set_lfo_parameter(LooperId(track_id), lfo, parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_lfo_mode(
    engine: *const LooperEngine,
    looper_id: usize,
    lfo_id: usize,
    mode: LFOMode,
) {
    let engine = &(*engine);
    engine
        .handle()
        .set_lfo_mode(LooperId(looper_id), lfo_id, mode);
}

/// Map an LFO to another parameter
#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_lfo_mapping(
    engine: *const LooperEngine,
    looper_id: usize,
    lfo_id: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle()
        .add_lfo_mapping(LooperId(looper_id), lfo_id, parameter_id, value)
}

/// Unmap an LFO from a parameter
#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_lfo_mapping(
    engine: *const LooperEngine,
    looper_id: usize,
    lfo_id: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle()
        .remove_lfo_mapping(LooperId(looper_id), lfo_id, parameter_id)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_lfo_sample(mode: LFOMode, phase: f32) -> f32 {
    let phase = phase / (std::f32::consts::PI * 2.0);
    match mode {
        LFOMode::LFOModeSine => augmented_oscillator::generators::sine_generator(phase),
        LFOMode::LFOModeSquare => augmented_oscillator::generators::square_generator(phase),
        LFOMode::LFOModeSaw => augmented_oscillator::generators::saw_generator(phase),
    }
}
