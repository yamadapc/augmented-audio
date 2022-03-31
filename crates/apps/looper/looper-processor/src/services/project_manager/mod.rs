use std::ops::Deref;
use std::path::{Path, PathBuf};

use actix::{Actor, ActorFutureExt, Handler, Message, ResponseActFuture, WrapFuture};
use basedrop::Shared;

use audio_garbage_collector::make_shared;

use crate::services::project_manager::model::LooperVoicePersist;
use crate::MultiTrackLooper;

use self::model::Project;

mod model;

#[derive(Debug, thiserror::Error)]
pub enum ProjectManagerError {
    #[error("IO error {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Default)]
pub struct ProjectManager {
    projects: Vec<Shared<Project>>,
}

impl Actor for ProjectManager {
    type Context = actix::Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Shared<Project>, ProjectManagerError>")]
pub struct LoadLatestProjectMessage;

impl Handler<LoadLatestProjectMessage> for ProjectManager {
    type Result = ResponseActFuture<Self, Result<Shared<Project>, ProjectManagerError>>;

    fn handle(&mut self, _msg: LoadLatestProjectMessage, _ctx: &mut Self::Context) -> Self::Result {
        let latest_project_fut = async { load_latest_project().await };
        let result_fut = latest_project_fut
            .into_actor(self)
            .map(|project, act, _ctx| {
                let project = make_shared(project?);
                act.projects.push(project.clone());
                Ok(project)
            });

        Box::pin(result_fut)
    }
}

/// Currently is only going to be sound on macos
fn data_path() -> PathBuf {
    let bundle_identifier = "beijaflor-io.Sequencer-Mac";
    let home_dir = Path::new(std::env!("HOME"));
    home_dir
        .join("Library/Application Support")
        .join(bundle_identifier)
}

async fn load_latest_project() -> Result<Project, ProjectManagerError> {
    let (default_project_path, project_manifest) = default_project_manifest_path();

    log::info!("Creating project directory at {:?}", default_project_path);
    tokio::fs::create_dir_all(&default_project_path).await?;

    if let Err(err) = tokio::fs::metadata(&project_manifest).await {
        if err.kind() == std::io::ErrorKind::NotFound {
            log::warn!("project.msgpack manifest doesn't exist, creating default");
            create_default_project().await?;
        } else {
            log::error!("Failed to read the project.msgpack manifest file");
            Err(err)?
        }
    }

    log::info!("project.msgpack found");
    let contents = tokio::fs::read(project_manifest).await?;
    let result: Project = rmp_serde::from_slice(&contents).unwrap();
    log::info!("  PROJECT=\n{:#?}\n", result);

    Ok(result)
}

fn default_project_path() -> PathBuf {
    let data_path = data_path();
    data_path.join("default_project")
}

fn default_project_manifest_path() -> (PathBuf, PathBuf) {
    let default_project_path = default_project_path();
    let project_manifest = default_project_path.join("project.msgpack");
    (default_project_path, project_manifest)
}

async fn create_default_project() -> Result<(), std::io::Error> {
    let default_project_path = default_project_path();
    let project_manifest = default_project_path.join("project.msgpack");
    let project = build_default_project();
    let buffer = rmp_serde::to_vec(&project).unwrap();
    log::info!("Writing: {:?}", project_manifest);
    tokio::fs::write(project_manifest, buffer).await?;
    Ok(())
}

fn build_default_project() -> Project {
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
    project
}

#[cfg(test)]
mod test {
    use actix::Actor;

    use super::*;

    async fn cleanup() {
        let (_default_project_path, project_manifest) = default_project_manifest_path();
        tokio::fs::remove_file(project_manifest).await.unwrap();
    }

    #[actix::test]
    async fn test_load_latest_project() {
        wisual_logger::init_from_env();
        cleanup().await;

        let latest_project = load_latest_project().await.unwrap();
        assert!(latest_project.voices.len() > 0);
    }

    #[actix::test]
    async fn test_actor_load_latest_project() {
        wisual_logger::init_from_env();
        cleanup().await;

        let project_manager = ProjectManager::default().start();
        project_manager
            .send(LoadLatestProjectMessage)
            .await
            .unwrap()
            .unwrap();
    }
}
