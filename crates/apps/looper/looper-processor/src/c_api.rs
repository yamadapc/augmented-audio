use atomic_refcell::{AtomicRef, AtomicRefCell};
use std::ops::Deref;
use std::ptr::null;

use basedrop::Shared;

use audio_processor_standalone::StandaloneHandles;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::AtomicF32;

use crate::multi_track_looper::{LFOParameter, LooperVoice, ParameterId, SourceParameter};
use crate::processor::handle::LooperState;
use crate::trigger_model::{TrackTriggerModel, Trigger, TriggerPosition};
use crate::{
    setup_osc_server, EnvelopeParameter, LooperId, LooperOptions, LooperProcessorHandle,
    MultiTrackLooper, MultiTrackLooperHandle, TimeInfoProvider,
};

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
    audio_handles: StandaloneHandles,
}

#[no_mangle]
pub extern "C" fn looper_engine__new() -> *mut LooperEngine {
    wisual_logger::init_from_env();

    let processor = MultiTrackLooper::new(Default::default(), 8);
    let handle = processor.handle().clone();
    let audio_handles = audio_processor_standalone::audio_processor_start_with_midi(
        processor,
        audio_garbage_collector::handle(),
    );
    setup_osc_server(handle.clone());

    let engine = LooperEngine {
        handle,
        audio_handles,
    };
    Box::into_raw(Box::new(engine))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__num_loopers(engine: *mut LooperEngine) -> usize {
    (*engine).handle.voices().len()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__record(engine: *mut LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Recording {}", looper_id);
    (*engine).handle.toggle_recording(LooperId(looper_id));
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
    ParameterId::ParameterIdSource { parameter }
}

#[no_mangle]
pub extern "C" fn looper_engine__envelope_parameter_id(
    parameter: EnvelopeParameter,
) -> ParameterId {
    ParameterId::ParameterIdEnvelope { parameter }
}

#[no_mangle]
pub extern "C" fn looper_engine__lfo_parameter_id(
    lfo: usize,
    parameter: LFOParameter,
) -> ParameterId {
    ParameterId::ParameterIdLFO { lfo, parameter }
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

#[repr(C)]
#[no_mangle]
pub struct CTimeInfo {
    position_samples: f64,
    position_beats: f64,
    // -1 means none
    tempo: f64, // -1 means none
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
#[no_mangle]
pub struct ExampleBuffer {
    pub ptr: *const f32,
    pub count: usize,
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
