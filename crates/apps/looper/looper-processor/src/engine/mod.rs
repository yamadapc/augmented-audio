use std::sync::{Arc, Mutex};

use basedrop::Shared;

use audio_processor_standalone::StandaloneHandles;

use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsActor;
use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::{setup_osc_server, MultiTrackLooper, MultiTrackLooperHandle};

pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Arc<Mutex<AudioProcessorMetricsActor>>,
    midi_store: Shared<MidiStoreHandle>,
    #[allow(unused)]
    audio_handles: StandaloneHandles,
}

impl LooperEngine {
    pub fn new() -> Self {
        wisual_logger::init_from_env();
        log::info!("LooperEngine setup sequence started");

        let processor = MultiTrackLooper::new(Default::default(), 8);
        let handle = processor.handle().clone();
        let audio_handles = audio_processor_standalone::audio_processor_start_with_midi(
            processor,
            audio_garbage_collector::handle(),
        );
        setup_osc_server(handle.clone());

        let metrics_actor = Arc::new(Mutex::new(AudioProcessorMetricsActor::new(
            handle.metrics_handle().clone(),
        )));
        let midi_store = handle.midi().clone();

        LooperEngine {
            handle,
            audio_handles,
            metrics_actor,
            midi_store,
        }
    }

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }

    pub fn metrics_actor(&self) -> &Arc<Mutex<AudioProcessorMetricsActor>> {
        &self.metrics_actor
    }

    pub fn midi_store(&self) -> &Shared<MidiStoreHandle> {
        &self.midi_store
    }

    pub fn audio_handles(&self) -> &StandaloneHandles {
        &self.audio_handles
    }
}
