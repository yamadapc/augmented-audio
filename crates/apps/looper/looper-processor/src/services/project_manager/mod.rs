use std::io::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use actix::{Actor, ActorFutureExt, AsyncContext, Handler, Message, ResponseActFuture, WrapFuture};
use atomic_refcell::AtomicRefCell;
use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_file::OutputAudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::AtomicF32;

use crate::services::audio_clip_manager::write_looper_clip;
use crate::{MultiTrackLooper, MultiTrackLooperHandle};

use self::model::LooperVoicePersist;
use self::model::Project;

pub mod model;

#[derive(Debug, thiserror::Error)]
pub enum ProjectManagerError {
    #[error("IO error {0}")]
    IOError(#[from] std::io::Error),
}

pub struct ProjectManager {
    data_path: PathBuf,
    projects: Vec<Shared<Project>>,
}

pub const PROJECT_MANAGER_DATA_PATH_KEY: &'static str = "CONTINUOUS_DATA_PATH";

impl Default for ProjectManager {
    fn default() -> Self {
        let data_path = std::env::var(PROJECT_MANAGER_DATA_PATH_KEY)
            .ok()
            .map(|p| PathBuf::from(p))
            .unwrap_or_else(|| data_path());
        log::info!("Data-path: {:?}", data_path);
        Self::new(data_path)
    }
}

impl ProjectManager {
    fn new(data_path: PathBuf) -> Self {
        Self {
            data_path,
            projects: vec![],
        }
    }
}

impl Actor for ProjectManager {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.address().do_send(LoadLatestProjectMessage);
    }
}

#[derive(Message)]
#[rtype(result = "Result<Shared<Project>, ProjectManagerError>")]
pub struct SaveProjectMessage {
    pub handle: Shared<MultiTrackLooperHandle>,
}

impl Handler<SaveProjectMessage> for ProjectManager {
    type Result = ResponseActFuture<Self, Result<Shared<Project>, ProjectManagerError>>;

    fn handle(&mut self, msg: SaveProjectMessage, _ctx: &mut Self::Context) -> Self::Result {
        let data_path = self.data_path.clone();
        let result_fut = async move {
            let (project_path, manifest_path) = default_project_manifest_path(&data_path);

            let looper_paths = persist_handle_clips(&*msg.handle, &*project_path);
            let project = make_shared(project_from_handle(&*msg.handle, looper_paths));

            write_project(manifest_path, &project).await?;

            Ok(project)
        }
        .into_actor(self)
        .map(|result, _, _| result.map_err(|err| ProjectManagerError::IOError(err)));

        Box::pin(result_fut)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Shared<Project>, ProjectManagerError>")]
pub struct LoadLatestProjectMessage;

impl Handler<LoadLatestProjectMessage> for ProjectManager {
    type Result = ResponseActFuture<Self, Result<Shared<Project>, ProjectManagerError>>;

    fn handle(&mut self, _msg: LoadLatestProjectMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Loading latest project from disk");
        let data_path = self.data_path.clone();
        let latest_project_fut = async { load_latest_project(data_path).await };
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
        .join("Library/Containers")
        .join(bundle_identifier)
        .join("Data/Library/Application Support")
}

async fn load_latest_project(data_path: impl AsRef<Path>) -> Result<Project, ProjectManagerError> {
    let (default_project_path, project_manifest) =
        default_project_manifest_path(data_path.as_ref());

    log::info!("Creating project directory at {:?}", default_project_path);
    tokio::fs::create_dir_all(&default_project_path).await?;

    if let Err(err) = tokio::fs::metadata(&project_manifest).await {
        if err.kind() == std::io::ErrorKind::NotFound {
            log::warn!("project.msgpack manifest doesn't exist, creating default");
            create_default_project(data_path.as_ref()).await?;
        } else {
            log::error!("Failed to read the project.msgpack manifest file");
            Err(err)?
        }
    }

    log::info!("project.msgpack found");
    let contents = tokio::fs::read(project_manifest).await?;
    let result: Project = rmp_serde::from_slice(&contents).unwrap();
    log::debug!("  PROJECT=\n{:#?}\n", result);

    Ok(result)
}

fn default_project_path(data_path: &Path) -> PathBuf {
    data_path.join("default_project")
}

fn default_project_manifest_path(data_path: &Path) -> (PathBuf, PathBuf) {
    let default_project_path = default_project_path(data_path);
    let project_manifest = default_project_path.join("project.msgpack");
    (default_project_path, project_manifest)
}

async fn create_default_project(data_path: &Path) -> Result<(), std::io::Error> {
    let (_, project_manifest_path) = default_project_manifest_path(data_path);
    let project = build_default_project();
    write_project(project_manifest_path, &project).await
}

async fn write_project(project_manifest_path: PathBuf, project: &Project) -> Result<(), Error> {
    let buffer = rmp_serde::to_vec(&project).unwrap();
    log::info!("Writing: {:?}", project_manifest_path);
    tokio::fs::write(project_manifest_path, buffer).await?;
    Ok(())
}

fn persist_handle_clips(
    handle: &MultiTrackLooperHandle,
    project_path: &Path,
) -> Vec<Option<PathBuf>> {
    handle
        .voices()
        .iter()
        .map(|voice| {
            if voice.looper().num_samples() == 0 {
                return None;
            }

            let settings = handle.settings().deref().clone();
            let clip_path = project_path.join(format!("looper_{}.wav", voice.id));
            let clip = voice.looper().looper_clip();

            write_looper_clip(settings, &clip_path, &clip);

            Some(clip_path)
        })
        .collect()
}

fn build_default_project() -> Project {
    let looper = MultiTrackLooper::default();
    let looper_handle = looper.handle();
    project_from_handle(looper_handle, vec![])
}

fn project_from_handle(
    looper_handle: &MultiTrackLooperHandle,
    looper_clips: Vec<Option<PathBuf>>,
) -> Project {
    Project {
        voices: looper_handle
            .voices()
            .iter()
            .map(|voice| LooperVoicePersist::from(voice))
            .collect(),
        scene_state: looper_handle.scene_handle().clone(),
        looper_clips,
        midi_map: looper_handle
            .midi()
            .midi_map()
            .store()
            .get()
            .deref()
            .clone(),
    }
}

#[cfg(test)]
mod test {
    use actix::Actor;

    use super::*;

    #[actix::test]
    async fn test_load_latest_project() {
        wisual_logger::init_from_env();
        let data_path = tempdir::TempDir::new("looper_processor__project_manager").unwrap();
        log::info!("data_path={:?}", data_path.path());

        let latest_project = load_latest_project(data_path.path()).await.unwrap();
        assert!(latest_project.voices.len() > 0);
    }

    #[actix::test]
    async fn test_actor_load_latest_project() {
        wisual_logger::init_from_env();
        let data_path = tempdir::TempDir::new("looper_processor__project_manager").unwrap();
        log::info!("data_path={:?}", data_path.path());
        let project_manager = ProjectManager::new(data_path.path().to_path_buf()).start();
        project_manager
            .send(LoadLatestProjectMessage)
            .await
            .unwrap()
            .unwrap();
    }
}
