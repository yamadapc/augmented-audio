use crate::audio::processor::handle::LooperState;
use crate::{LooperEngine, LooperHandleThread, LooperId};
use actix_system_threads::ActorSystemThread;
use std::time::Duration;

#[test]
fn test_start_engine_and_record_audio() {
    let engine = LooperEngine::new();
    // wait for audio-thread to start
    std::thread::sleep(Duration::from_secs(3));

    let looper_id = LooperId(0);
    let state = engine.handle().get_looper_state(looper_id);
    assert_eq!(state, LooperState::Empty);

    // Record 1s of audio
    engine
        .handle()
        .toggle_recording(looper_id, LooperHandleThread::OtherThread);
    std::thread::sleep(Duration::from_secs(1));

    // Stop recording
    engine
        .handle()
        .toggle_recording(looper_id, LooperHandleThread::OtherThread);

    let state = engine.handle().get_looper_state(looper_id);
    assert_eq!(state, LooperState::Playing);
    loop {
        if let Some(_) = engine.handle().get_looper_slices(looper_id) {
            break;
        }
    }

    let _ = ActorSystemThread::current().spawn_result(async move {
        engine.save_project().await;
    });
}
