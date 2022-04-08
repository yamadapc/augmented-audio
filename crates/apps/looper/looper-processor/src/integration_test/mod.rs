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
use std::time::Duration;

use actix_system_threads::ActorSystemThread;

use crate::audio::multi_track_looper::effects_processor::EffectType;
use crate::audio::multi_track_looper::parameters::LooperId;
use crate::audio::processor::handle::LooperHandleThread;
use crate::audio::processor::handle::LooperState;
use crate::engine::LooperEngine;
use crate::services::project_manager::PROJECT_MANAGER_DATA_PATH_KEY;

#[test]
#[ignore]
fn test_start_engine_and_record_audio() {
    let data_path = tempdir::TempDir::new("test_start_engine_and_record_audio").unwrap();
    std::env::set_var(
        PROJECT_MANAGER_DATA_PATH_KEY,
        data_path.path().to_str().unwrap(),
    );

    let engine = LooperEngine::default();
    // wait for audio-thread to start
    std::thread::sleep(Duration::from_secs(3));

    let looper_id = LooperId(0);
    let state = engine.handle().get_looper_state(looper_id);
    assert_eq!(state, LooperState::Empty);

    // Add reverb
    engine.handle().voices()[0]
        .effects()
        .add_effect(EffectType::EffectTypeReverb);
    engine.handle().voices()[0]
        .effects()
        .add_effect(EffectType::EffectTypeFilter);
    engine.handle().voices()[0]
        .effects()
        .add_effect(EffectType::EffectTypeDelay);

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
