use std::collections::HashMap;

use async_trait::async_trait;
use itertools::Itertools;

use crate::model::{AnalyticsEvent, ClientMetadata};

use super::backend_trait::AnalyticsBackend;

pub struct GoogleAnalyticsConfig {
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

pub struct GoogleAnalyticsBackend {
    client: reqwest::Client,
    config: GoogleAnalyticsConfig,
}

impl GoogleAnalyticsBackend {
    pub fn google(config: GoogleAnalyticsConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
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
        let mut body: Vec<HashMap<String, String>> = Vec::new();
        for event in events {
            let event = Self::build_event(metadata, &self.config, event);
            body.push(event);
        }

        let chunks: Vec<String> = body
            .iter()
            .chunks(20)
            .into_iter()
            .map(|batch| {
                batch
                    .into_iter()
                    .filter_map(|event| serde_urlencoded::to_string(event).ok())
                    .join("\n")
            })
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
            let response = self.client.execute(request).await?;
            log::info!("Flushed analytics events response={:?}", response);
        }

        Ok(())
    }
}
