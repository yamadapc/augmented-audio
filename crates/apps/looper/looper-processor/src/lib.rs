//! This crate implements a multi-track, single-track and shuffling-loopers.
//!
//! Multi-track loopers support sequencing, parameter locking, LFOs and effects. The multi-track
//! looper is a sampler/live-looper with a C API. This is integrated onto the Continuous Looper.
//!
//! Internals:
//!
//! * [`audio`] - Contains most of the audio-thread bits
//!   - [`audio::processor`] - Is a looper
//!   - [`audio::shuffler`] - Is a shuffling looper
//!   - [`audio::loop_quantization`] - Quantization modes & logic
//!   - [`audio::multi_track_looper`] - Continuous Looper processor
//!   - [`audio::midi_map`] - MIDI map store (used in Continuous)
//! * [`services`] - Some IO support for Continuous
//! * [`c_api`] - C API
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

pub mod audio;
#[allow(clippy::missing_safety_doc)]
pub mod c_api;
pub mod engine;
pub mod services;

#[cfg(test)]
pub mod integration_test;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
