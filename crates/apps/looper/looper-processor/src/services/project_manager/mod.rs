use std::ops::Deref;
use std::path::{Path, PathBuf};

use actix::{Actor, AsyncContext, Handler, Message, ResponseActFuture};
use serde::{Deserialize, Serialize};

use crate::audio::multi_track_looper::looper_voice::LooperVoice;
use crate::audio::multi_track_looper::ParametersMap;
use crate::services::file_manager::AudioClipModel;
use crate::services::project_manager::model::LooperVoicePersist;
use crate::MultiTrackLooper;

use self::model::Project;

mod model;

pub enum ProjectManagerError {}

#[derive(Default)]
pub struct ProjectManager {}

impl ProjectManager {
    /// Currently is only going to be sound on macos
    fn data_path() -> PathBuf {
        let bundle_identifier = "beijaflor-io.Sequencer-Mac";
        let home_dir = Path::new(std::env!("HOME"));
        home_dir
            .join("Library/Application Support")
            .join(bundle_identifier)
    }

    fn default_project_path() -> PathBuf {
        let data_path = Self::data_path();
        data_path.join("default_project")
    }

    fn default_project_manifest_path() -> (PathBuf, PathBuf) {
        let default_project_path = Self::default_project_path();
        let project_manifest = default_project_path.join("project.msgpack");
        (default_project_path, project_manifest)
    }

    async fn load_latest_project() {
        let (default_project_path, project_manifest) = Self::default_project_manifest_path();

        log::info!("Creating project directory at {:?}", default_project_path);
        tokio::fs::create_dir_all(&default_project_path).await;

        if let Err(err) = tokio::fs::metadata(&project_manifest).await {
            if err.kind() == std::io::ErrorKind::NotFound {
                log::warn!("project.msgpack manifest doesn't exist, creating default");
                Self::create_default_project().await;
            } else {
                log::error!("Failed to read the project.msgpack manifest file");
                return;
            }
        }

        log::info!("project.msgpack found");
        let contents = tokio::fs::read(project_manifest).await.unwrap();
        let result: Project = rmp_serde::from_slice(&contents).unwrap();
        log::info!("  PROJECT=\n{:#?}\n", result);
    }

    async fn create_default_project() {
        let default_project_path = Self::default_project_path();
        let project_manifest = default_project_path.join("project.msgpack");
        let looper = MultiTrackLooper::default();
        let project = Project {
            voices: looper
                .handle()
                .voices()
                .iter()
                .map(|voice| LooperVoicePersist::from(voice))
                .collect(),
            scene_state: looper.handle().scene_handle().clone(),
            looper_clips: vec![],
            midi_map: looper
                .handle()
                .midi()
                .midi_map()
                .store()
                .get()
                .deref()
                .clone(),
        };
        let buffer = rmp_serde::to_vec(&project).unwrap();
        log::info!("Writing: {:?}", project_manifest);
        tokio::fs::write(project_manifest, buffer).await.unwrap();
    }
}

impl Actor for ProjectManager {
    type Context = actix::Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Project, ProjectManagerError>")]
pub struct LoadLatestProjectMessage;

impl Handler<LoadLatestProjectMessage> for ProjectManager {
    type Result = ResponseActFuture<Self, Result<Project, ProjectManagerError>>;

    fn handle(&mut self, msg: LoadLatestProjectMessage, ctx: &mut Self::Context) -> Self::Result {
        let default_project_path = Self::default_project_path();
        ctx.spawn(actix::fut::wrap_future(async {}));
        todo!()
    }
}

#[cfg(test)]
mod test {
    use actix::Actor;

    use super::*;

    #[actix::test]
    async fn test_load_latest_project() {
        wisual_logger::init_from_env();
        let (_default_project_path, project_manifest) =
            ProjectManager::default_project_manifest_path();
        tokio::fs::remove_file(project_manifest).await.unwrap();
        let latest_project = ProjectManager::load_latest_project().await;
    }

    #[actix::test]
    async fn test_actor_load_latest_project() {
        let project_manager = ProjectManager::default().start();
        project_manager
            .send(LoadLatestProjectMessage)
            .await
            .unwrap();
    }
}
