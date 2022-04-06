use std::sync::Mutex;

use actix::Addr;
use basedrop::Shared;

use actix_system_threads::ActorSystemThread;
use audio_processor_standalone::StandaloneHandles;

use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsActor;
use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::controllers::autosave_controller::AutosaveController;
use crate::controllers::load_project_controller;
use crate::services::audio_clip_manager::AudioClipManager;
use crate::services::project_manager::ProjectManager;
use crate::{services, setup_osc_server, MultiTrackLooper, MultiTrackLooperHandle};

pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Mutex<AudioProcessorMetricsActor>,
    audio_clip_manager: Addr<AudioClipManager>,
    project_manager: Addr<ProjectManager>,
    _autosave_controller: AutosaveController,
    _audio_handles: StandaloneHandles,
}

impl LooperEngine {
    pub fn new() -> Self {
        wisual_logger::init_from_env();
        log::info!("LooperEngine setup sequence started");

        let processor = MultiTrackLooper::default();
        let handle = processor.handle().clone();

        let metrics_handle = handle.metrics_handle().clone();
        let metrics_actor = Mutex::new(AudioProcessorMetricsActor::new(metrics_handle));

        let audio_clip_manager = ActorSystemThread::start(AudioClipManager::default());
        let project_manager = ActorSystemThread::start(ProjectManager::default());
        let _ = load_project_controller::load_and_hydrate_latest_project(
            &handle,
            &project_manager,
            &audio_clip_manager,
        );

        let autosave_controller = ActorSystemThread::current().spawn_result({
            let project_manager = project_manager.clone();
            let handle = handle.clone();
            async move { AutosaveController::new(project_manager, handle) }
        });
        setup_osc_server(handle.clone());

        // Start audio
        let audio_handles = audio_processor_standalone::audio_processor_start_with_midi(
            processor,
            audio_garbage_collector::handle(),
        );

        LooperEngine {
            handle,
            metrics_actor,
            audio_clip_manager,
            project_manager,
            _autosave_controller: autosave_controller,
            _audio_handles: audio_handles,
        }
    }

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }

    pub fn metrics_actor(&self) -> &Mutex<AudioProcessorMetricsActor> {
        &self.metrics_actor
    }

    pub fn midi_store(&self) -> &Shared<MidiStoreHandle> {
        self.handle.midi()
    }

    pub fn audio_clip_manager(&self) -> &Addr<AudioClipManager> {
        &self.audio_clip_manager
    }
}

impl LooperEngine {
    pub async fn save_project(&self) {
        self.project_manager
            .send(services::project_manager::SaveProjectMessage {
                handle: self.handle.clone(),
            })
            .await
            .unwrap()
            .unwrap();
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
