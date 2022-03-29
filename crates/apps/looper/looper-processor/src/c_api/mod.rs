use std::ffi::c_void;


use std::sync::{Arc, Mutex};

use atomic_refcell::AtomicRefCell;

use basedrop::Shared;

use audio_processor_standalone::StandaloneHandles;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::{AtomicF32, AtomicValue};

use crate::multi_track_looper::metrics::audio_processor_metrics::{
    AudioProcessorMetricsActor, AudioProcessorMetricsStats,
};
use crate::multi_track_looper::midi_store::MidiStoreHandle;
use crate::multi_track_looper::parameters::{
    CQuantizeMode, EnvelopeParameter, LFOParameter, LooperId, ParameterId, SourceParameter,
    TempoControl,
};
use crate::multi_track_looper::slice_worker::SliceResult;
use crate::parameters::ParameterValue;
use crate::processor::handle::{LooperHandleThread, LooperState};
use crate::{setup_osc_server, MultiTrackLooper, MultiTrackLooperHandle, TimeInfoProvider};

pub use self::entity_id::*;
pub use self::foreign_callback::*;
pub use self::midi_callback::*;

mod entity_id;
mod foreign_callback;
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

pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Arc<Mutex<AudioProcessorMetricsActor>>,
    midi_store: Shared<MidiStoreHandle>,
    events_callback: Option<EventsCallback>,
    #[allow(unused)]
    audio_handles: StandaloneHandles,
}

impl LooperEngine {
    fn new() -> Self {
        wisual_logger::init_from_env();

        let processor = MultiTrackLooper::new(Default::default(), 8);
        let handle = processor.handle().clone();
        let audio_handles = audio_processor_standalone::audio_processor_start_with_midi(
            processor,
            audio_garbage_collector::handle(),
        );
        setup_osc_server(handle.clone());

        let metrics_actor = Arc::new(Mutex::new(AudioProcessorMetricsActor::new(
            handle.metrics_handle().clone(),
        )));
        let midi_store = handle.midi().clone();

        
        LooperEngine {
            handle,
            audio_handles,
            metrics_actor,
            midi_store,
            events_callback: None,
        }
    }
}

#[no_mangle]
pub extern "C" fn looper_engine__new() -> *mut LooperEngine {
    let engine = LooperEngine::new();
    into_ptr(engine)
}

#[no_mangle]
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
            None => CParameterValueNone,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_parameter_value(
    engine: *mut LooperEngine,
    looper_id: LooperId,
    parameter_id: ParameterId,
) -> CParameterValue {
    let engine = &(*engine);
    if let Some(value) = engine.handle.get_parameter(looper_id, &parameter_id) {
        CParameterValue::from(value)
    } else {
        CParameterValue::CParameterValueNone
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__free(engine: *mut LooperEngine) {
    let _ = Box::from_raw(engine);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__num_loopers(engine: *mut LooperEngine) -> usize {
    (*engine).handle.voices().len()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__record(engine: *mut LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Recording {}", looper_id);
    (*engine)
        .handle
        .toggle_recording(LooperId(looper_id), LooperHandleThread::OtherThread)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__play(engine: *mut LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Playing {}", looper_id);
    (*engine).handle.toggle_playback(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__clear(engine: *mut LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Clearing {}", looper_id);
    (*engine).handle.clear(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_active_looper(
    engine: *mut LooperEngine,
    looper_id: usize,
) {
    (*engine).handle.set_active_looper(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_num_samples(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> usize {
    (*engine).handle.get_num_samples(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_state(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> LooperState {
    (*engine).handle.get_looper_state(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_position(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> f32 {
    (*engine).handle.get_position_percent(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__toggle_trigger(
    engine: *mut LooperEngine,
    looper_id: usize,
    position_beats: usize,
) {
    (*engine)
        .handle
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
    engine: *mut LooperEngine,
    looper_id: usize,
    parameter_id: ParameterId,
    value: bool,
) {
    (*engine)
        .handle
        .set_boolean_parameter(LooperId(looper_id), parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_scene_slider_value(
    engine: *mut LooperEngine,
    value: f32,
) {
    (*engine).handle.set_scene_value(value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_lfo_mapping(
    engine: *mut LooperEngine,
    looper_id: usize,
    lfo_id: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle
        .add_lfo_mapping(LooperId(looper_id), lfo_id, parameter_id, value)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_lfo_mapping(
    engine: *mut LooperEngine,
    looper_id: usize,
    lfo_id: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle
        .remove_lfo_mapping(LooperId(looper_id), lfo_id, parameter_id)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_scene_parameter_lock(
    engine: *mut LooperEngine,
    scene_id: usize,
    looper_id: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle
        .add_scene_parameter_lock(scene_id, LooperId(looper_id), parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_scene_parameter_lock(
    engine: *mut LooperEngine,
    scene_id: usize,
    looper_id: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle
        .remove_scene_parameter_lock(scene_id, LooperId(looper_id), parameter_id);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_parameter_lock(
    engine: *mut LooperEngine,
    looper_id: usize,
    position_beats: usize,
    parameter_id: ParameterId,
    value: f32,
) {
    (*engine)
        .handle
        .add_parameter_lock(LooperId(looper_id), position_beats, parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__remove_parameter_lock(
    engine: *mut LooperEngine,
    looper_id: usize,
    position_beats: usize,
    parameter_id: ParameterId,
) {
    (*engine)
        .handle
        .remove_parameter_lock(LooperId(looper_id), position_beats, parameter_id);
}

pub enum EngineEvent {}

#[repr(C)]
pub struct EventsCallback {
    userdata: *mut c_void,
    callback: extern "C" fn(*mut c_void, *mut EngineEvent),
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_events_callback(
    engine: *mut LooperEngine,
    callback: EventsCallback,
) {
    (*engine).events_callback = Some(callback);
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
pub unsafe extern "C" fn looper_engine__playhead_stop(engine: *mut LooperEngine) {
    (*engine).handle.stop();
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__playhead_play(engine: *mut LooperEngine) {
    (*engine).handle.play();
}

pub enum LooperBuffer {
    Some {
        inner: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    },
    None,
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_envelope_parameter(
    engine: *mut LooperEngine,
    track_id: usize,
    envelope_parameter_id: EnvelopeParameter,
    value: f32,
) {
    let engine = &(*engine);
    engine
        .handle
        .set_envelope_parameter(LooperId(track_id), envelope_parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_source_parameter_int(
    engine: *mut LooperEngine,
    track_id: usize,
    parameter_id: SourceParameter,
    value: i32,
) {
    let engine = &(*engine);
    engine.handle.set_int_parameter(
        LooperId(track_id),
        ParameterId::ParameterIdSource(parameter_id),
        value,
    )
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_quantization_mode(
    engine: *mut LooperEngine,
    track_id: usize,
    quantization_mode: CQuantizeMode,
) {
    let engine = &(*engine);
    engine
        .handle
        .set_quantization_mode(LooperId(track_id), quantization_mode)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_tempo_control(
    engine: *mut LooperEngine,
    track_id: usize,
    tempo_control: TempoControl,
) {
    let engine = &(*engine);
    engine
        .handle
        .set_tempo_control(LooperId(track_id), tempo_control)
}

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
        .handle
        .set_lfo_parameter(LooperId(track_id), lfo, parameter_id, value);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_buffer(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> *mut LooperBuffer {
    let engine = &(*engine);
    into_ptr(
        if let Some(buffer) = engine.handle.get_looper_buffer(LooperId(looper_id)) {
            LooperBuffer::Some { inner: buffer }
        } else {
            LooperBuffer::None
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn looper_buffer__free(buffer: *mut LooperBuffer) {
    let _ = Box::from_raw(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn looper_buffer__num_samples(buffer: *mut LooperBuffer) -> usize {
    let buffer = &(*buffer);
    match buffer {
        LooperBuffer::Some { inner } => inner.borrow().num_samples(),
        LooperBuffer::None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_buffer__get(buffer: *mut LooperBuffer, index: usize) -> f32 {
    let buffer = &(*buffer);
    match buffer {
        LooperBuffer::Some { inner } => {
            let inner = inner.borrow();
            let mut total = 0.0;
            for channel in 0..inner.num_channels() {
                total += inner.get(channel, index).get();
            }
            total
        }
        LooperBuffer::None => 0.0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_slices(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> *mut Option<SliceResult> {
    let engine = &(*engine);
    into_ptr(engine.handle.get_looper_slices(LooperId(looper_id)))
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__free(buffer: *mut Option<SliceResult>) {
    let _ = Box::from_raw(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__length(buffer: *mut Option<SliceResult>) -> usize {
    (*buffer)
        .as_ref()
        .map(|buffer| buffer.markers().len())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__get(
    buffer: *mut Option<SliceResult>,
    index: usize,
) -> usize {
    (*buffer)
        .as_ref()
        .and_then(|buffer| buffer.markers().get(index))
        .map(|marker| marker.position_samples)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_source_parameter(
    engine: *mut LooperEngine,
    looper_id: usize,
    parameter_id: SourceParameter,
    value: f32,
) {
    (*engine)
        .handle
        .set_source_parameter(LooperId(looper_id), parameter_id, value)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_tempo(engine: *mut LooperEngine, tempo: f32) {
    let handle = &(*engine).handle;
    handle.set_tempo(tempo);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_playhead_position(
    engine: *mut LooperEngine,
) -> CTimeInfo {
    let handle = &(*engine).handle;
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
    engine: *mut LooperEngine,
    looper_id: usize,
    volume: f32,
) {
    (*engine).handle.set_volume(LooperId(looper_id), volume);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_metronome_volume(
    engine: *mut LooperEngine,
    volume: f32,
) {
    (*engine).handle.set_metronome_volume(volume);
}

#[repr(C)]
pub struct ExampleBuffer {
    pub ptr: *const f32,
    pub count: usize,
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_stats(
    engine: *mut LooperEngine,
) -> AudioProcessorMetricsStats {
    let metrics_actor = &(*engine).metrics_actor;
    if let Ok(mut metrics_actor) = metrics_actor.lock() {
        metrics_actor.poll()
    } else {
        AudioProcessorMetricsStats::default()
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper__get_example_buffer() -> ExampleBuffer {
    use audio_processor_file::AudioFileProcessor;

    let settings = AudioProcessorSettings::default();
    let mut processor = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &audio_processor_testing_helpers::relative_path!(
            "../../../augmented/audio/audio-processor-analysis/hiphop-drum-loop.mp3"
        ),
    )
    .unwrap();
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
    ExampleBuffer {
        ptr: output_ptr,
        count: size,
    }
}
