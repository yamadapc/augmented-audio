pub use atomic_refcell::AtomicRefCell;

pub use c_api::*;
pub use multi_track_looper::parameters;
pub use multi_track_looper::parameters::EnvelopeParameter;
pub use multi_track_looper::parameters::LooperId;
pub use multi_track_looper::{MultiTrackLooper, MultiTrackLooperHandle};
pub use osc_server::setup_osc_server;
pub use processor::handle::LooperHandle as LooperProcessorHandle;
pub use processor::handle::LooperOptions;
pub use processor::handle::QuantizeMode;
pub use processor::handle::QuantizeOptions;
pub use processor::LooperProcessor;
pub use sequencer::LoopSequencerParams;
pub use sequencer::LoopSequencerProcessorHandle;
pub use time_info_provider::{TimeInfo, TimeInfoProvider, TimeInfoProviderImpl};

#[allow(clippy::missing_safety_doc)]
mod c_api;
mod loop_quantization;
mod midi_map;
mod multi_track_looper;
mod osc_server;
mod processor;
mod sequencer;
mod time_info_provider;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
