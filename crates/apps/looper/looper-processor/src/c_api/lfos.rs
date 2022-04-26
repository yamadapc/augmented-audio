use crate::parameters::{LFOMode, LFOParameter, ParameterId};
use crate::{LooperEngine, LooperId};

/// Change LFO parameters on a given track and LFO path
#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_lfo_parameter(
    engine: *mut LooperEngine,
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

/// Map an LFO to another parameter
#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_lfo_mapping(
    engine: *mut LooperEngine,
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
    engine: *mut LooperEngine,
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
