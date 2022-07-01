// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::sync::Mutex;
use std::time::Duration;

use actix::Addr;
use basedrop::Shared;

use actix_system_threads::ActorSystem;

use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsActor;
use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::controllers::audio_io_settings_controller::AudioIOSettingsController;
use crate::controllers::audio_state_controller::{AudioModeParams, AudioStateController};
use crate::controllers::autosave_controller::AutosaveController;
use crate::controllers::events_controller::EventsController;
use crate::controllers::load_project_controller;
use crate::controllers::load_project_controller::LoadContext;
use crate::services::audio_clip_manager::AudioClipManager;
use crate::services::project_manager::ProjectManager;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use crate::services::{
    analytics::AnalyticsService,
    analytics::{send_analytics, ServiceAnalyticsEvent},
};
use crate::{
    services, setup_osc_server, LooperOptions, MultiTrackLooper, MultiTrackLooperHandle,
    MAX_LOOP_LENGTH_SECS,
};

pub struct LooperEngineParams {
    pub audio_mode: AudioModeParams,

    /// Feature-flag
    is_persistence_enabled: bool,
}

impl Default for LooperEngineParams {
    fn default() -> Self {
        Self {
            audio_mode: AudioModeParams::Standalone,
            is_persistence_enabled: true,
        }
    }
}

/// This is Continuous' main entry-point
///
/// How to treat this struct:
///
/// * NO logic, only holds pointers to other bits of the app
/// * This is the entry-point for the C API. The C API is not safe and mutability in this struct
///   may end-up in disaster
pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Mutex<AudioProcessorMetricsActor>,
    audio_clip_manager: Addr<AudioClipManager>,
    project_manager: Addr<ProjectManager>,
    events_controller: Addr<EventsController>,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    analytics_service: Addr<AnalyticsService>,
    audio_state_controller: Addr<AudioStateController>,

    audio_io_settings_controller: AudioIOSettingsController,
    _autosave_controller: Option<AutosaveController>,
}

impl Default for LooperEngine {
    fn default() -> Self {
        Self::new(LooperEngineParams::default())
    }
}

impl LooperEngine {
    pub fn new(params: LooperEngineParams) -> Self {
        wisual_logger::init_from_env();
        log::info!("LooperEngine setup sequence started");

        let processor = MultiTrackLooper::new(
            LooperOptions {
                max_loop_length: Duration::from_secs_f32(MAX_LOOP_LENGTH_SECS),
                host_callback: if let AudioModeParams::Hosted(host_callback) = params.audio_mode {
                    host_callback
                } else {
                    None
                },
            },
            8,
        );
        let handle = processor.handle().clone();

        let metrics_actor = Self::setup_metrics(&handle);
        setup_osc_server(handle.clone());

        let events_controller = ActorSystem::start(EventsController::default());
        let audio_clip_manager = ActorSystem::start(AudioClipManager::default());
        let project_manager = ActorSystem::start(ProjectManager::default());

        let autosave_controller = {
            if params.is_persistence_enabled {
                let controller = ActorSystem::current().spawn_result({
                    let project_manager = project_manager.clone();
                    let handle = handle.clone();
                    async move { AutosaveController::new(project_manager, handle) }
                });
                let _ = load_project_controller::load_and_hydrate_latest_project(LoadContext {
                    events_controller: events_controller.clone(),
                    handle: handle.clone(),
                    project_manager: project_manager.clone(),
                    audio_clip_manager: audio_clip_manager.clone(),
                });
                Some(controller)
            } else {
                None
            }
        };
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let analytics_service = ActorSystem::start(AnalyticsService::default());

        // Start audio
        let audio_state_controller =
            ActorSystem::start(AudioStateController::new(params.audio_mode, processor));

        #[cfg(any(target_os = "ios", target_os = "macos"))]
        {
            let looper_loaded_event = ServiceAnalyticsEvent::Event {
                category: "operational".to_string(),
                action: "loaded".to_string(),
                label: "LooperEngine".to_string(),
                value: "0".to_string(),
            };
            send_analytics(&analytics_service, looper_loaded_event);
        }

        let audio_io_settings_controller =
            AudioIOSettingsController::new(audio_state_controller.clone());

        LooperEngine {
            handle,
            metrics_actor,
            audio_clip_manager,
            project_manager,
            events_controller,
            audio_state_controller,
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            analytics_service,
            audio_io_settings_controller,
            _autosave_controller: autosave_controller,
        }
    }

    fn setup_metrics(handle: &Shared<MultiTrackLooperHandle>) -> Mutex<AudioProcessorMetricsActor> {
        let metrics_handle = handle.metrics_handle().clone();

        Mutex::new(AudioProcessorMetricsActor::new(metrics_handle))
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

    pub fn events_controller(&self) -> &Addr<EventsController> {
        &self.events_controller
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    pub fn analytics_service(&self) -> &Addr<AnalyticsService> {
        &self.analytics_service
    }

    pub fn audio_state_controller(&self) -> &Addr<AudioStateController> {
        &self.audio_state_controller
    }

    pub fn audio_io_settings_controller(&self) -> &AudioIOSettingsController {
        &self.audio_io_settings_controller
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
        let _engine = LooperEngine::default();
    }
}
