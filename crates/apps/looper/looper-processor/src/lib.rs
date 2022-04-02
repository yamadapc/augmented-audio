pub use atomic_refcell::AtomicRefCell;

pub use self::audio::multi_track_looper::parameters;
pub use self::audio::multi_track_looper::parameters::EnvelopeParameter;
pub use self::audio::multi_track_looper::parameters::LooperId;
pub use self::audio::multi_track_looper::{MultiTrackLooper, MultiTrackLooperHandle};
pub use self::audio::processor::handle::LooperHandle as LooperProcessorHandle;
pub use self::audio::processor::handle::LooperHandleThread;
pub use self::audio::processor::handle::LooperOptions;
pub use self::audio::processor::handle::QuantizeMode;
pub use self::audio::processor::handle::QuantizeOptions;
pub use self::audio::processor::LooperProcessor;
pub use self::audio::shuffler::LoopShufflerParams;
pub use self::audio::shuffler::LoopShufflerProcessorHandle;
pub use self::audio::time_info_provider::{TimeInfo, TimeInfoProvider, TimeInfoProviderImpl};
pub use self::c_api::*;
pub use self::services::osc_server::setup_osc_server;

mod audio;
#[allow(clippy::missing_safety_doc)]
mod c_api;
mod engine;
mod services;

#[cfg(test)]
mod integration_test;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
