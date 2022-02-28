pub use processor::handle::LooperHandle as LooperProcessorHandle;
pub use processor::handle::LooperOptions;
pub use processor::handle::QuantizeMode;
pub use processor::handle::QuantizeOptions;
pub use processor::LooperProcessor;
pub use sequencer::LoopSequencerParams;
pub use sequencer::LoopSequencerProcessorHandle;
pub use time_info_provider::{TimeInfo, TimeInfoProvider, TimeInfoProviderImpl};

mod loop_quantization;
mod midi_map;
mod multi_track_looper;
mod processor;
mod sequencer;
mod time_info_provider;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;
