pub use audio_engine_service::AudioEngineService;
pub use audio_options_service::{AudioOptions, AudioOptionsService, AvailableAudioOptions};
pub use examples::async_operation;

mod audio_engine_service;
mod audio_options_service;
mod callbacks;
mod chart_handler;
mod examples;

pub fn initialize_logger() {
    let _ = wisual_logger::try_init_from_env();
}

uniffi_macros::include_scaffolding!("augmented");
