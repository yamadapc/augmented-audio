use std::sync::Arc;

mod audio_engine_service;
mod audio_options_service;
mod callbacks;
mod chart_handler;
mod examples;

use audio_engine_service::AudioEngineService;

/// CLI version of recording_buddy
fn main() {
    wisual_logger::init_from_env();
    let engine: Arc<AudioEngineService> = AudioEngineService::default().into();
    engine.start();
    std::thread::park();
}
