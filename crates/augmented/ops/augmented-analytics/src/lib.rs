use std::collections::HashMap;
use std::time::Duration;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct GoogleAnalyticsConfig {
    version: String,
    tracking_id: String,
}

impl GoogleAnalyticsConfig {
    pub fn new(tracking_id: String) -> Self {
        Self {
            version: String::from("1"),
            tracking_id,
        }
    }
}

pub enum AnalyticsBackend {
    GoogleAnalytics(GoogleAnalyticsConfig),
}

pub struct ClientMetadata {
    pub client_id: String,
}

pub struct AnalyticsWorkerConfig {
    pub max_flush_duration: Duration,
    pub max_batch_size: usize,
}

impl Default for AnalyticsWorkerConfig {
    fn default() -> Self {
        Self {
            max_flush_duration: Duration::from_millis(1000),
            max_batch_size: 20,
        }
    }
}

pub struct AnalyticsWorker {
    config: AnalyticsWorkerConfig,
    backend: AnalyticsBackend,
    client_metadata: ClientMetadata,
    events_queue: UnboundedReceiver<AnalyticsEvent>,
}

impl AnalyticsWorker {
    /// Create a new analytics worker with provided parameters
    pub fn new(
        config: AnalyticsWorkerConfig,
        backend: AnalyticsBackend,
        client_metadata: ClientMetadata,
        events_queue: UnboundedReceiver<AnalyticsEvent>,
    ) -> Self {
        AnalyticsWorker {
            config,
            backend,
            client_metadata,
            events_queue,
        }
    }

    /// Runs a debounced loop to flush events
    pub async fn run_loop(&mut self) {
        let mut events = vec![];
        let duration = self.config.max_flush_duration;
        loop {
            let recv_future = self.events_queue.recv();
            let result = tokio::time::timeout(duration, recv_future).await;
            match result {
                Ok(Some(event)) => {
                    events.push(event);
                    if events.len() >= 20 {
                        self.flush(&mut events).await;
                    }
                }
                Ok(None) => {
                    self.flush(&mut events).await;
                    break;
                }
                Err(_) => {
                    self.flush(&mut events).await;
                }
            }
        }
    }

    /// Force flush all events until the sender queue is closed
    pub async fn flush_all(&mut self) {
        let mut events = vec![];
        while let Some(event) = self.events_queue.recv().await {
            log::info!("Flushing event {:?}", event);
            events.push(event);
        }
        self.send_bulk(&events).await;
    }

    /// Flushes the current batch of events and clears the queue
    async fn flush(&mut self, events: &mut Vec<AnalyticsEvent>) {
        self.send_bulk(&events).await;
        events.clear();
    }

    /// Sends a slice of events to the configured back-end
    async fn send_bulk(&mut self, events: &[AnalyticsEvent]) {
        match &self.backend {
            AnalyticsBackend::GoogleAnalytics(config) => {
                let client = reqwest::Client::new();

                let mut body: Vec<HashMap<String, String>> = Vec::new();
                for event in events {
                    let event = Self::build_event(&self.client_metadata, config, event);
                    body.push(event);
                }

                for batch in &body.iter().chunks(self.config.max_batch_size) {
                    let batch = batch
                        .into_iter()
                        .filter_map(|event| serde_urlencoded::to_string(event).ok())
                        .join("\n");

                    if let Ok(request) = client
                        .post("https://www.google-analytics.com/batch")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(batch)
                        .build()
                    {
                        let response = client.execute(request).await;
                        log::info!("Flushed analytics events - Response: {:?}", response);
                    } else {
                        log::error!("Error building analytics request");
                    }
                }
            }
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

#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum AnalyticsEvent {
    ScreenView {
        application: String,
        application_version: String,
        application_id: Option<String>,
        application_installer_id: Option<String>,
        content: String,
    },
    Event {
        category: String,
        action: String,
        label: Option<String>,
        value: Option<String>,
    },
}

impl Default for AnalyticsEvent {
    fn default() -> Self {
        AnalyticsEvent::Event {
            category: "".into(),
            action: "".into(),
            label: None,
            value: None,
        }
    }
}

impl AnalyticsEvent {
    pub fn builder() -> AnalyticsEventBuilder {
        AnalyticsEventBuilder::default()
    }
}

pub struct AnalyticsEventBuilder {}

impl Default for AnalyticsEventBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl AnalyticsEventBuilder {
    pub fn screen() -> ScreenViewEventBuilder {
        ScreenViewEventBuilder::default()
    }
}

pub struct ScreenViewEventBuilder {
    application: String,
    application_version: String,
    application_id: Option<String>,
    application_installer_id: Option<String>,
    content: String,
}

impl Default for ScreenViewEventBuilder {
    fn default() -> Self {
        ScreenViewEventBuilder {
            application: "".to_string(),
            application_version: "".to_string(),
            application_id: None,
            application_installer_id: None,
            content: "".to_string(),
        }
    }
}

impl ScreenViewEventBuilder {
    pub fn application(mut self, application: impl Into<String>) -> Self {
        self.application = application.into();
        self
    }

    pub fn application_version(mut self, application_version: impl Into<String>) -> Self {
        self.application_version = application_version.into();
        self
    }

    pub fn application_id(mut self, application_id: impl Into<String>) -> Self {
        self.application_id = Some(application_id.into());
        self
    }

    pub fn application_installer_id(mut self, application_installer_id: impl Into<String>) -> Self {
        self.application_installer_id = Some(application_installer_id.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn build(self) -> AnalyticsEvent {
        AnalyticsEvent::ScreenView {
            application: self.application,
            application_version: self.application_version,
            application_id: self.application_id,
            application_installer_id: self.application_installer_id,
            content: self.content,
        }
    }
}

pub struct AnalyticsClient {
    events_queue: UnboundedSender<AnalyticsEvent>,
}

impl AnalyticsClient {
    pub fn new(events_queue: UnboundedSender<AnalyticsEvent>) -> Self {
        AnalyticsClient { events_queue }
    }

    pub fn send(&self, event: AnalyticsEvent) {
        // Errors will happen if the receiver is closed.
        // In that case, the events will be dropped.
        if let Err(_) = self.events_queue.send(event) {
            log::error!("Receiver is down, but analytics event was fired.");
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::mpsc::unbounded_channel;

    use crate::{
        AnalyticsBackend, AnalyticsClient, AnalyticsEvent, AnalyticsWorker, ClientMetadata,
        GoogleAnalyticsConfig,
    };

    #[tokio::test]
    async fn test_setup_client() {
        let _ = wisual_logger::try_init_from_env();

        let (sender, receiver) = unbounded_channel();
        let mut worker = AnalyticsWorker {
            config: Default::default(),
            backend: AnalyticsBackend::GoogleAnalytics(GoogleAnalyticsConfig {
                version: String::from("1"),
                tracking_id: String::from("UA-74188650-6"),
            }),
            client_metadata: ClientMetadata {
                client_id: String::from("1"),
            },
            events_queue: receiver,
        };

        {
            let client = AnalyticsClient {
                events_queue: sender,
            };
            client.send(AnalyticsEvent::ScreenView {
                application: String::from("testing_analytics_client"),
                application_version: "0.0.0".to_string(),
                application_id: None,
                application_installer_id: None,
                content: "test".to_string(),
            });
            client.send(AnalyticsEvent::Event {
                category: "interaction".to_string(),
                action: "play".to_string(),
                label: None,
                value: None,
            });
        }

        worker.flush_all().await;
    }
}
