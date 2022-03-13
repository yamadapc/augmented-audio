use basedrop::Shared;

use audio_processor_standalone::StandaloneHandles;

use crate::multi_track_looper::LooperVoice;
use crate::{
    setup_osc_server, LooperId, LooperOptions, LooperProcessorHandle, MultiTrackLooper,
    MultiTrackLooperHandle,
};

pub struct SharedPtr<T>(*mut Shared<T>);

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
    if let Some(voice) = (*engine).handle.get(LooperId(looper_id)) {
        voice.looper().toggle_recording();
    }
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
pub unsafe extern "C" fn looper_engine__get_voice(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> *mut LooperVoice {
    let voice = &(*engine).handle.voices()[looper_id];
    voice as *const LooperVoice as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn looper_voice__get_looper_handle(
    voice: *mut LooperVoice,
) -> SharedPtr<LooperProcessorHandle> {
    SharedPtr(Box::into_raw(Box::new((*voice).looper().clone())))
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
