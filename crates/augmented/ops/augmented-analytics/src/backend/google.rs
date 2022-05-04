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
use std::collections::HashMap;

use async_trait::async_trait;
use itertools::Itertools;
use mockall_double::double;

use crate::model::{AnalyticsEvent, ClientMetadata};

use super::backend_trait::AnalyticsBackend;
#[double]
use super::reqwest_executor::RequestExecutor;

pub struct GoogleAnalyticsConfig {
    /// The version of the GA API (default to 1)
    pub version: String,
    pub tracking_id: String,
}

impl Default for GoogleAnalyticsConfig {
    fn default() -> Self {
        Self {
            version: String::from("1"),
            tracking_id: String::from(""),
        }
    }
}

impl GoogleAnalyticsConfig {
    pub fn new(tracking_id: impl Into<String>) -> Self {
        Self {
            version: String::from("1"),
            tracking_id: tracking_id.into(),
        }
    }
}

/// Analytics back-end for Google Analytics
pub struct GoogleAnalyticsBackend {
    client: reqwest::Client,
    config: GoogleAnalyticsConfig,
    executor: RequestExecutor,
}

impl GoogleAnalyticsBackend {
    /// Create a back-end with config
    pub fn new(config: GoogleAnalyticsConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            executor: RequestExecutor::default(),
        }
    }

    #[doc(hidden)]
    pub fn new_with_executor(config: GoogleAnalyticsConfig, executor: RequestExecutor) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            executor,
        }
    }

    fn build_event(
        client_metadata: &ClientMetadata,
        config: &GoogleAnalyticsConfig,
        event: &AnalyticsEvent,
    ) -> HashMap<String, String> {
        match event {
            AnalyticsEvent::ScreenView {
                application,
                application_id,
                application_installer_id,
                application_version,
                content,
            } => {
                let mut event = HashMap::new();
                event.insert("v".into(), config.version.clone());
                event.insert("tid".into(), config.tracking_id.clone());
                event.insert("cid".into(), client_metadata.client_id.clone());
                event.insert("t".into(), "screenview".into());
                event.insert("av".into(), application_version.clone());
                event.insert("an".into(), application.clone());
                if let Some(aid) = application_id {
                    event.insert("aid".into(), aid.clone());
                }
                if let Some(aiid) = application_installer_id {
                    event.insert("aiid".into(), aiid.clone());
                }
                event.insert("cd".into(), content.clone());
                event
            }
            AnalyticsEvent::Event {
                action,
                category,
                label,
                value,
            } => {
                let mut event = HashMap::new();
                event.insert("v".into(), config.version.clone());
                event.insert("tid".into(), config.tracking_id.clone());
                event.insert("cid".into(), client_metadata.client_id.clone());
                event.insert("t".into(), "event".into());
                event.insert("ea".into(), action.clone());
                event.insert("ec".into(), category.clone());
                if let Some(label) = label {
                    event.insert("el".into(), label.clone());
                }
                if let Some(value) = value {
                    event.insert("ev".into(), value.clone());
                }
                event
            }
        }
    }
}

#[async_trait]
impl AnalyticsBackend for GoogleAnalyticsBackend {
    async fn send_bulk(
        &mut self,
        metadata: &ClientMetadata,
        events: &[AnalyticsEvent],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event_body = events
            .iter()
            .map(|event| Self::build_event(metadata, &self.config, event))
            .filter_map(|event| serde_urlencoded::to_string(event).ok());

        let chunks: Vec<String> = event_body
            .chunks(20)
            .into_iter()
            .map(|batch| batch.into_iter().join("\n"))
            .collect();

        for batch in chunks {
            let request = self
                .client
                .post("https://www.google-analytics.com/batch")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(batch)
                .build()
                .map_err(|err| {
                    Box::<dyn std::error::Error + Send + Sync>::from(format!(
                        "Failed to build request: {}",
                        err
                    ))
                })?;

            let status = self.executor.execute(&self.client, request).await?;
            log::info!(
                "Flushed analytics events event_count={} status={:?}",
                events.len(),
                status
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::*;
    use crate::backend::reqwest_executor;

    #[tokio::test]
    async fn test_send_bulk_screen() {
        let mut executor = RequestExecutor::default();
        executor
            .expect_execute()
            .return_once(|_, _| Ok(reqwest::StatusCode::try_from(200).unwrap()));

        let mut backend = GoogleAnalyticsBackend::new_with_executor(
            GoogleAnalyticsConfig::new("UA-1234"),
            executor,
        );
        let metadata = ClientMetadata::new("1234");
        let events = vec![AnalyticsEvent::screen().build()];

        backend.send_bulk(&metadata, &events).await.unwrap();
    }

    #[tokio::test]
    async fn test_send_bulk() {
        let mut executor = RequestExecutor::default();
        executor
            .expect_execute()
            .return_once(|_, _| Ok(reqwest::StatusCode::try_from(200).unwrap()));

        let mut backend = GoogleAnalyticsBackend::new_with_executor(
            GoogleAnalyticsConfig::new("UA-1234"),
            executor,
        );
        let metadata = ClientMetadata::new("1234");
        let events = vec![AnalyticsEvent::event()
            .action("view")
            .category("video")
            .build()];

        backend.send_bulk(&metadata, &events).await.unwrap();
    }

    #[tokio::test]
    async fn test_send_bulk_failure() {
        let mut executor = RequestExecutor::default();
        executor
            .expect_execute()
            .return_once(|_, _| Err(reqwest_executor::ExecutorError::MockError));

        let mut backend = GoogleAnalyticsBackend::new_with_executor(
            GoogleAnalyticsConfig::new("UA-1234"),
            executor,
        );
        let metadata = ClientMetadata::new("1234");
        let events = vec![AnalyticsEvent::event()
            .action("view")
            .category("video")
            .build()];

        assert!(backend.send_bulk(&metadata, &events).await.is_err());
    }
}
