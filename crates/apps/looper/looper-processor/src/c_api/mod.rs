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

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null;

use basedrop::Shared;

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use augmented_atomics::AtomicValue;

pub use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsStats;
use crate::audio::multi_track_looper::parameters::ParameterValue;
use crate::audio::multi_track_looper::parameters::{
    CQuantizeMode, EnvelopeParameter, LFOParameter, LooperId, ParameterId, SourceParameter,
    TempoControl,
};
pub use crate::engine::LooperEngine;
use crate::TimeInfoProvider;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::analytics::*;
pub use self::audio_clip_manager::*;
pub use self::audio_io_settings_controller::*;
pub use self::entity_id::*;
pub use self::events::*;
pub use self::foreign_callback::*;
pub use self::lfos::*;
pub use self::looper::*;
pub use self::metrics::*;
pub use self::midi_callback::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod analytics;
mod audio_clip_manager;
mod audio_io_settings_controller;
pub mod effects;
mod entity_id;
mod events;
mod foreign_callback;
mod lfos;
mod looper;
mod metrics;
mod midi_callback;

fn into_ptr<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

pub struct SharedPtr<T>(*mut Shared<T>);

impl<T> From<Shared<T>> for SharedPtr<T> {
    fn from(ptr: Shared<T>) -> Self {
        SharedPtr(Box::into_raw(Box::new(ptr)))
    }
}

#[no_mangle]
pub extern "C" fn looper_engine__new() -> *const LooperEngine {
    let engine = LooperEngine::default();
    into_ptr(engine)
}

#[repr(C)]
pub enum CParameterValue {
    CParameterValueFloat(f32),
    CParameterValueBool(bool),
    CParameterValueEnum(usize),
    CParameterValueInt(i32),
    CParameterValueNone,
}

impl From<ParameterValue> for CParameterValue {
    fn from(value: ParameterValue) -> Self {
        use crate::c_api::CParameterValue::*;
        use ParameterValue::*;

        match value {
            Float(f) => CParameterValueFloat(f.get()),
            Bool(b) => CParameterValueBool(b.get()),
            Enum(e) => CParameterValueEnum(e.get()),
            Int(i) => CParameterValueInt(i.get()),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_parameter_value(
    engine: *const LooperEngine,
    looper_id: LooperId,
    parameter_id: ParameterId,
) -> CParameterValue {
    let engine = &(*engine);
    if let Some(value) = engine.handle().get_parameter(looper_id, &parameter_id) {
        CParameterValue::from(value)
    } else {
        CParameterValue::CParameterValueNone
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__free(engine: *const LooperEngine) {
    let _ = Box::from_raw(engine as *mut LooperEngine);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__toggle_trigger(
    engine: *const LooperEngine,
    looper_id: usize,
    position_beats: usize,
) {
    (*engine)
        .handle()
        .toggle_trigger(LooperId(looper_id), position_beats)
}

#[no_mangle]
pub extern "C" fn looper_engine__source_parameter_id(parameter: SourceParameter) -> ParameterId {
    ParameterId::ParameterIdSource(parameter)
}

#[no_mangle]
pub extern "C" fn looper_engine__envelope_parameter_id(
    parameter: EnvelopeParameter,
) -> ParameterId {
    ParameterId::ParameterIdEnvelope(parameter)
}

#[no_mangle]
pub extern "C" fn looper_engine__lfo_parameter_id(
    lfo: usize,
    parameter: LFOParameter,
) -> ParameterId {
    ParameterId::ParameterIdLFO(lfo, parameter)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_boolean_parameter(
    engine: *const LooperEngine,
    looper_id: usize,
    parameter_id: ParameterId,
    value: bool,
) {
    (*engine)
        .handle()
        .set_boolean_parameter(LooperId(looper_id), parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_scene_slider_value(
    engine: *const LooperEngine,
    value: f32,
) {
    (*engine).handle().set_scene_value(value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_scene_parameter_lock(
    engine: *const LooperEngine,
    scene_id: usize,
    looper_id: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle()
        .add_scene_parameter_lock(scene_id, LooperId(looper_id), parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_scene_parameter_lock(
    engine: *const LooperEngine,
    scene_id: usize,
    looper_id: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle()
        .remove_scene_parameter_lock(scene_id, LooperId(looper_id), parameter_id);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_parameter_lock(
    engine: *const LooperEngine,
    looper_id: usize,
    position_beats: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle()
        .add_parameter_lock(LooperId(looper_id), position_beats, parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_parameter_lock(
    engine: *const LooperEngine,
    looper_id: usize,
    position_beats: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle()
        .remove_parameter_lock(LooperId(looper_id), position_beats, parameter_id);
}

#[repr(C)]
pub struct CTimeInfo {
    position_samples: f64,
    position_beats: f64,
    // -1 means none
    tempo: f64, // -1 means none
    is_playing: bool,
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__playhead_stop(engine: *const LooperEngine) {
    (*engine).handle().stop();
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__playhead_play(engine: *const LooperEngine) {
    (*engine).handle().play();
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_envelope_parameter(
    engine: *const LooperEngine,
    track_id: usize,
    envelope_parameter_id: EnvelopeParameter,
    value: f32,
) {
    let engine = &(*engine);
    engine
        .handle()
        .set_envelope_parameter(LooperId(track_id), envelope_parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_source_parameter_int(
    engine: *const LooperEngine,
    track_id: usize,
    parameter_id: SourceParameter,
    value: i32,
) {
    let engine = &(*engine);
    engine.handle().set_int_parameter(
        LooperId(track_id),
        ParameterId::ParameterIdSource(parameter_id),
        value,
    )
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_quantization_mode(
    engine: *const LooperEngine,
    track_id: usize,
    quantization_mode: CQuantizeMode,
) {
    let engine = &(*engine);
    engine
        .handle()
        .set_quantization_mode(LooperId(track_id), quantization_mode)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_tempo_control(
    engine: *const LooperEngine,
    track_id: usize,
    tempo_control: TempoControl,
) {
    let engine = &(*engine);
    engine
        .handle()
        .set_tempo_control(LooperId(track_id), tempo_control)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_source_parameter(
    engine: *const LooperEngine,
    looper_id: usize,
    parameter_id: SourceParameter,
    value: f32,
) {
    (*engine)
        .handle()
        .set_source_parameter(LooperId(looper_id), parameter_id, value)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_tempo(engine: *const LooperEngine, tempo: f32) {
    let handle = &(*engine).handle();
    handle.set_tempo(tempo);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_playhead_position(
    engine: *const LooperEngine,
) -> CTimeInfo {
    let handle = &(*engine).handle();
    let time_info_provider = handle.time_info_provider();
    let time_info = time_info_provider.get_time_info();

    CTimeInfo {
        position_samples: time_info.position_samples(),
        position_beats: time_info.position_beats().unwrap_or(-1.0),
        tempo: time_info.tempo().unwrap_or(-1.0),
        is_playing: time_info.is_playing(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_volume(
    engine: *const LooperEngine,
    looper_id: usize,
    volume: f32,
) {
    (*engine).handle().set_volume(LooperId(looper_id), volume);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_metronome_volume(
    engine: *const LooperEngine,
    volume: f32,
) {
    (*engine).handle().set_metronome_volume(volume);
}

#[repr(C)]
pub struct ExampleBuffer {
    pub ptr: *const f32,
    pub count: usize,
}

fn get_example_buffer(example_path: &CStr) -> anyhow::Result<ExampleBuffer> {
    let settings = AudioProcessorSettings::default();
    let example_path: String = example_path.to_str()?.to_string();
    let mut processor = audio_processor_file::AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &example_path,
    )?;
    processor.prepare(settings);
    let channels = processor.buffer().clone();
    let mut output = vec![];
    for (s1, s2) in channels[0].iter().zip(channels[1].clone()) {
        output.push(s1 + s2);
    }
    let mut output = output.into_boxed_slice();
    let output_ptr = output.as_mut_ptr();
    let size = output.len();
    std::mem::forget(output);

    Ok(ExampleBuffer {
        ptr: output_ptr,
        count: size,
    })
}

#[no_mangle]
pub unsafe extern "C" fn looper__get_example_buffer(example_path: *const c_char) -> ExampleBuffer {
    let example_path: &CStr = CStr::from_ptr(example_path);
    get_example_buffer(example_path).unwrap_or_else(|err: anyhow::Error| {
        log::error!("Failed to open example file {}", err);
        ExampleBuffer {
            ptr: null(),
            count: 0,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn looper__init_logging() {
    wisual_logger::init_from_env();
}
