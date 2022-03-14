use std::ptr::null;

use basedrop::Shared;

use audio_processor_standalone::StandaloneHandles;
use audio_processor_traits::AudioProcessorSettings;

use crate::multi_track_looper::LooperVoice;
use crate::trigger_model::{TrackTriggerModel, Trigger, TriggerPosition};
use crate::{
    setup_osc_server, LooperId, LooperOptions, LooperProcessorHandle, MultiTrackLooper,
    MultiTrackLooperHandle, TimeInfoProvider,
};

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
pub unsafe extern "C" fn looper_engine__get_looper_position(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> f32 {
    (*engine).handle.get_position_percent(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_voice(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> *mut LooperVoice {
    let voice = &(*engine).handle.voices()[looper_id];
    voice as *const LooperVoice as *mut _
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
    (*engine).handle.time_info_provider().stop();
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__playhead_play(engine: *mut LooperEngine) {
    (*engine).handle.time_info_provider().play();
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_playhead_position(
    engine: *mut LooperEngine,
) -> CTimeInfo {
    let time_info_provider = (*engine).handle.time_info_provider();
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
    log::info!("looper_engine - Clearing {}", looper_id);
    (*engine).handle.set_volume(LooperId(looper_id), volume);
}

#[no_mangle]
pub unsafe extern "C" fn looper_voice__get_triggers(
    voice: *mut LooperVoice,
) -> SharedPtr<TrackTriggerModel> {
    SharedPtr::from((*voice).triggers().clone())
}

#[no_mangle]
pub unsafe extern "C" fn track_trigger_model__get_len(
    track_trigger_model: SharedPtr<TrackTriggerModel>,
) -> usize {
    (*track_trigger_model.0).len()
}

pub unsafe extern "C" fn track_trigger_model__add_trigger(
    track_trigger_model: SharedPtr<TrackTriggerModel>,
    position_beats: usize,
) {
    let mut trigger = Trigger::default();
    trigger.set_position(TriggerPosition::BeatsUsize {
        pos: position_beats.into(),
    });
    (*track_trigger_model.0).add_trigger(trigger);
}

#[no_mangle]
pub unsafe extern "C" fn trigger_get_beats(trigger: *const Trigger) -> f32 {
    (*trigger).beats()
}

#[no_mangle]
pub unsafe extern "C" fn track_trigger_model__get_elem(
    track_trigger_model: SharedPtr<TrackTriggerModel>,
    index: usize,
) -> *const Trigger {
    if let Some(trigger) = (*track_trigger_model.0).triggers().get(index) {
        trigger as *const Trigger
    } else {
        null() as *const Trigger
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_voice__get_looper_handle(
    voice: *mut LooperVoice,
) -> SharedPtr<LooperProcessorHandle> {
    SharedPtr::from((*voice).looper().clone())
}

#[no_mangle]
pub unsafe extern "C" fn looper_handle__free(handle: SharedPtr<LooperProcessorHandle>) {
    std::mem::drop(Box::from_raw(handle.0));
}

#[no_mangle]
pub unsafe extern "C" fn looper_handle__is_recording(
    handle: SharedPtr<LooperProcessorHandle>,
) -> bool {
    (*handle.0).is_recording()
}

#[no_mangle]
pub unsafe extern "C" fn looper_handle__is_playing_back(
    handle: SharedPtr<LooperProcessorHandle>,
) -> bool {
    (*handle.0).is_playing_back()
}

#[repr(C)]
#[no_mangle]
pub struct ExampleBuffer {
    pub ptr: *mut f32,
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
