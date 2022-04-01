use std::sync::{Arc, Mutex};

use actix::{Addr, SyncArbiter};
use basedrop::Shared;

use actix_system_threads::ActorSystemThread;
use audio_processor_standalone::StandaloneHandles;

use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsActor;
use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::services::audio_clip_manager::AudioClipManager;
use crate::services::project_manager::ProjectManager;
use crate::{setup_osc_server, MultiTrackLooper, MultiTrackLooperHandle};

pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Arc<Mutex<AudioProcessorMetricsActor>>,
    midi_store: Shared<MidiStoreHandle>,
    audio_clip_manager: Addr<AudioClipManager>,
    project_manager: Addr<ProjectManager>,
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

        let audio_clip_manager = ActorSystemThread::current()
            .spawn_result(async move { SyncArbiter::start(1, || AudioClipManager::default()) });
        let project_manager = ActorSystemThread::start(ProjectManager::default());

        LooperEngine {
            handle,
            audio_handles,
            metrics_actor,
            midi_store,
            audio_clip_manager,
            project_manager,
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

    pub fn audio_clip_manager(&self) -> &Addr<AudioClipManager> {
        &self.audio_clip_manager
    }
}

#[cfg(test)]
mod test {
    use crate::LooperEngine;

    #[test]
    fn test_engine_boots() {
        wisual_logger::init_from_env();
        let _engine = LooperEngine::new();
    }
}