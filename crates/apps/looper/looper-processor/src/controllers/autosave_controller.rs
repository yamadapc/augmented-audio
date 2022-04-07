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
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use actix::Addr;
use anyhow::Result;
use basedrop::Shared;
use tokio::task::JoinHandle;

use augmented_atomics::AtomicValue;

use crate::services::project_manager::{ProjectManager, SaveProjectMessage};
use crate::MultiTrackLooperHandle;

const AUTOSAVE_INTERVAL_SECS: u64 = 30;

pub struct AutosaveController {
    is_running: Arc<AtomicBool>,
    task_handle: JoinHandle<()>,
}

impl AutosaveController {
    pub fn new(
        project_manager: Addr<ProjectManager>,
        handle: Shared<MultiTrackLooperHandle>,
    ) -> Self {
        let is_running = Arc::new(AtomicBool::new(true));
        let task_handle = {
            let project_manager = project_manager;
            let is_running = is_running.clone();
            actix::spawn(async move {
                Self::autosave_loop(is_running, project_manager, handle).await;
            })
        };

        Self {
            is_running,
            task_handle,
        }
    }

    async fn autosave_loop(
        is_running: Arc<AtomicBool>,
        project_manager: Addr<ProjectManager>,
        handle: Shared<MultiTrackLooperHandle>,
    ) {
        let mut interval = tokio::time::interval(Duration::from_secs(AUTOSAVE_INTERVAL_SECS));
        while is_running.get() {
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

impl Drop for AutosaveController {
    fn drop(&mut self) {
        self.task_handle.abort();
        self.is_running.set(false);
    }
}
