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
use actix::Addr;
use basedrop::Shared;

use actix_system_threads::ActorSystemThread;
use audio_processor_traits::AudioBuffer;

use crate::audio::multi_track_looper::looper_voice::LooperVoice;
use crate::audio::multi_track_looper::parameters::{build_default_parameters, ParameterId};
use crate::audio::multi_track_looper::ParametersMap;
use crate::controllers::events_controller::{ApplicationEvent, BroadcastMessage, EventsController};
use crate::services::audio_clip_manager::{AudioClipManager, AudioClipModelRef, LoadClipMessage};
use crate::services::project_manager::model::{LooperVoicePersist, Project};
use crate::services::project_manager::{LoadLatestProjectMessage, ProjectManager};
use crate::MultiTrackLooperHandle;

pub struct LoadContext {
    pub handle: Shared<MultiTrackLooperHandle>,
    pub project_manager: Addr<ProjectManager>,
    pub audio_clip_manager: Addr<AudioClipManager>,
    pub events_controller: Addr<EventsController>,
}

pub fn load_and_hydrate_latest_project(context: LoadContext) -> anyhow::Result<()> {
    ActorSystemThread::current()
        .spawn_result(async { run_load_latest_project(context).await })
        .map_err(|err| {
            log::error!("Failed to load latest project: {}", err);
            err
        })
}

async fn run_load_latest_project(context: LoadContext) -> anyhow::Result<()> {
    log::info!("Starting to load project from disk");
    let LoadContext {
        handle,
        project_manager,
        audio_clip_manager,
        events_controller,
    } = context;

    log::info!("Asking for latest project");
    let latest_project: Shared<Project> = project_manager.send(LoadLatestProjectMessage).await??;

    log::info!("Loaded previous project, hydrating...");
    {
        // MIDI in-place copy
        for (spec, action) in latest_project.midi_map.iter() {
            handle.midi().midi_map().add(*spec, action.clone());
        }
    }
    {
        // scene in-place
        let (_, parameter_ids) = build_default_parameters();
        let handle_scene = handle.scene_handle();
        handle_scene.set_slider(latest_project.scene_state.get_slider());
        for (source_scene, destination_scene) in latest_project
            .scene_state
            .scenes()
            .iter()
            .zip(handle_scene.scenes().iter())
        {
            for (scene_source_voice, scene_destination_voice) in source_scene
                .scene_parameters()
                .iter()
                .zip(destination_scene.scene_parameters().iter())
            {
                copy_parameters(&parameter_ids, scene_source_voice, scene_destination_voice)
            }
        }
    }
    {
        let (_, parameter_ids) = build_default_parameters();
        // parameter in-place
        for (source_voice, destination_voice) in latest_project.voices.iter().zip(handle.voices()) {
            copy_parameters(
                &parameter_ids,
                &source_voice.parameter_values,
                destination_voice.user_parameters(),
            );
            let trigger_model = destination_voice.trigger_model();
            trigger_model.add_triggers(&source_voice.triggers.triggers);
            copy_lfo(&parameter_ids, source_voice, destination_voice)
        }
    }
    {
        // clips in-place
        copy_clips(
            &handle,
            &audio_clip_manager,
            &events_controller,
            &latest_project,
        )
        .await?;
    }

    Ok(())
}

async fn copy_clips(
    destination: &Shared<MultiTrackLooperHandle>,
    audio_clip_manager: &Addr<AudioClipManager>,
    events_controller: &Addr<EventsController>,
    latest_project: &Project,
) -> anyhow::Result<()> {
    for (source_clip, destination_voice) in
        latest_project.looper_clips.iter().zip(destination.voices())
    {
        if let Some(path) = source_clip {
            let clip: AudioClipModelRef = audio_clip_manager
                .send(LoadClipMessage {
                    path: path.to_path_buf(),
                })
                .await??;
            let buffer = clip.contents();
            log::info!(
                "Read clip num_channels={} num_samples={}",
                buffer.num_channels(),
                buffer.num_samples()
            );
            destination_voice.looper().set_looper_buffer(buffer);
            events_controller
                .send(BroadcastMessage(
                    ApplicationEvent::ApplicationEventLooperClipUpdated {
                        looper_id: destination_voice.id,
                    },
                ))
                .await?;
        }
    }
    Ok(())
}

fn copy_lfo(
    parameter_ids: &[ParameterId],
    source_voice: &LooperVoicePersist,
    destination_voice: &LooperVoice,
) {
    let lfo = destination_voice.lfo1();
    for parameter in parameter_ids {
        lfo.set_parameter_map(parameter.clone(), Some(source_voice.lfo1.get(parameter)));
    }
    let lfo = destination_voice.lfo2();
    for parameter in parameter_ids {
        lfo.set_parameter_map(parameter.clone(), Some(source_voice.lfo2.get(parameter)));
    }
}

fn copy_parameters(
    parameter_ids: &[ParameterId],
    source_map: &ParametersMap,
    destination_map: &ParametersMap,
) {
    for parameter_id in parameter_ids {
        if source_map.has_value(parameter_id.clone()) {
            let value = source_map.get(parameter_id.clone());
            destination_map.set(parameter_id.clone(), value.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use crate::audio::multi_track_looper::ParametersMap;
    use crate::controllers::load_project_controller::copy_parameters;
    use crate::parameters::{build_parameter_ids, SourceParameter};

    #[test]
    fn test_copy_parameters() {
        let parameter_ids = build_parameter_ids();

        let parameters_map1 = ParametersMap::new();
        parameters_map1.set(SourceParameter::Start, 0.8);

        let parameters_map2 = ParametersMap::new();
        parameters_map2.set(SourceParameter::Start, 0.5);

        copy_parameters(&parameter_ids, &parameters_map1, &parameters_map2);
        assert_f_eq!(parameters_map2.get(SourceParameter::Start).as_float(), 0.8);
        assert!(!parameters_map1.has_value(SourceParameter::End));
        assert!(!parameters_map2.has_value(SourceParameter::End));
    }
}
