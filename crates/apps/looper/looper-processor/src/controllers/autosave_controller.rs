use std::time::Duration;

use actix::Addr;
use anyhow::Result;
use basedrop::Shared;
use tokio::task::JoinHandle;

use crate::services::project_manager::{ProjectManager, SaveProjectMessage};
use crate::MultiTrackLooperHandle;

const AUTOSAVE_INTERVAL_SECS: u64 = 30;

pub struct AutosaveController {
    task_handle: JoinHandle<()>,
}

impl Drop for AutosaveController {
    fn drop(&mut self) {
        self.task_handle.abort();
    }
}

impl AutosaveController {
    pub fn new(
        project_manager: Addr<ProjectManager>,
        handle: Shared<MultiTrackLooperHandle>,
    ) -> Self {
        let task_handle = {
            let project_manager = project_manager.clone();
            actix::spawn(async move {
                Self::autosave_loop(project_manager, handle).await;
            })
        };

        Self { task_handle }
    }

    async fn autosave_loop(
        project_manager: Addr<ProjectManager>,
        handle: Shared<MultiTrackLooperHandle>,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(AUTOSAVE_INTERVAL_SECS));
        loop {
            interval.tick().await;
            if let Err(err) = Self::autosave_flush(&project_manager, &handle).await {
                log::error!("Failed to auto-save {}", err);
            }
        }
    }

    async fn autosave_flush(
        project_manager: &Addr<ProjectManager>,
        handle: &Shared<MultiTrackLooperHandle>,
    ) -> Result<()> {
        log::info!("Triggered auto-save message");
        project_manager
            .send(SaveProjectMessage {
                handle: handle.clone(),
            })
            .await??;
        Ok(())
    }
}
