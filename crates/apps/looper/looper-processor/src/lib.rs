pub use atomic_refcell::AtomicRefCell;

pub use c_api::*;
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

mod audio_processor_metrics;
mod audio_thread_logger;
mod c_api;
mod loop_quantization;
mod midi_map;
mod multi_track_looper;
mod osc_server;
mod processor;
mod sequencer;
mod slice_worker;
mod tempo_estimation;
mod time_info_provider;
mod trigger_model;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
