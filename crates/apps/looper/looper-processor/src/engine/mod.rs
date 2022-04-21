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

use actix_system_threads::ActorSystemThread;
use audio_processor_standalone::StandaloneHandles;

use crate::audio::multi_track_looper::metrics::audio_processor_metrics::AudioProcessorMetricsActor;
use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::audio::time_info_provider::HostCallback;
use crate::controllers::autosave_controller::AutosaveController;
use crate::controllers::events_controller::EventsController;
use crate::controllers::load_project_controller;
use crate::controllers::load_project_controller::LoadContext;
use crate::services::audio_clip_manager::AudioClipManager;
use crate::services::project_manager::ProjectManager;
#[cfg(any(target_os = "ios", target_os = "macos"))]
use crate::services::{analytics, analytics::AnalyticsService};
use crate::{
    services, setup_osc_server, LooperOptions, MultiTrackLooper, MultiTrackLooperHandle,
    MAX_LOOP_LENGTH_SECS,
};

enum AudioState {
    Standalone(StandaloneHandles),
    Hosted(MultiTrackLooper),
}

pub enum AudioModeParams {
    Standalone,
    Hosted(Option<HostCallback>),
}

pub struct LooperEngineParams {
    pub audio_mode: AudioModeParams,

    /// Feature-flag
    is_persistence_enabled: bool,
}

impl Default for LooperEngineParams {
    fn default() -> Self {
        Self {
            audio_mode: AudioModeParams::Standalone,
            is_persistence_enabled: false,
        }
    }
}

/// This is Continuous' main entry-point
pub struct LooperEngine {
    handle: Shared<MultiTrackLooperHandle>,
    metrics_actor: Mutex<AudioProcessorMetricsActor>,
    audio_clip_manager: Addr<AudioClipManager>,
    project_manager: Addr<ProjectManager>,
    events_controller: Addr<EventsController>,
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    analytics_service: Addr<analytics::AnalyticsService>,
    audio_state: AudioState,
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

        let events_controller = ActorSystemThread::start(EventsController::default());
        let audio_clip_manager = ActorSystemThread::start(AudioClipManager::default());
        let project_manager = ActorSystemThread::start(ProjectManager::default());

        let autosave_controller = {
            if params.is_persistence_enabled {
                let controller = ActorSystemThread::current().spawn_result({
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
        let analytics_service = ActorSystemThread::start(AnalyticsService::default());

        // Start audio
        let audio_state = match params.audio_mode {
            AudioModeParams::Standalone => {
                AudioState::Standalone(audio_processor_standalone::audio_processor_start_with_midi(
                    processor,
                    audio_garbage_collector::handle(),
                ))
            }
            AudioModeParams::Hosted(_) => AudioState::Hosted(processor),
        };

        LooperEngine {
            handle,
            metrics_actor,
            audio_clip_manager,
            project_manager,
            events_controller,
            audio_state,
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            analytics_service,
            _autosave_controller: autosave_controller,
        }
    }

    fn setup_metrics(handle: &Shared<MultiTrackLooperHandle>) -> Mutex<AudioProcessorMetricsActor> {
        let metrics_handle = handle.metrics_handle().clone();
        let metrics_actor = Mutex::new(AudioProcessorMetricsActor::new(metrics_handle));
        metrics_actor
    }

    pub fn processor(&self) -> Option<&MultiTrackLooper> {
        match &self.audio_state {
            AudioState::Standalone(_) => None,
            AudioState::Hosted(processor) => Some(processor),
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

    pub fn events_controller(&self) -> &Addr<EventsController> {
        &self.events_controller
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    pub fn analytics_service(&self) -> &Addr<AnalyticsService> {
        &self.analytics_service
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
