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

use std::time::SystemTime;

use actix::{Actor, Handler, Message};
use actix_system_threads::ActorSystem;
use cacao::defaults::UserDefaults;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use augmented_analytics::{
    AnalyticsEvent, AnalyticsWorker, ClientMetadata, GoogleAnalyticsBackend, GoogleAnalyticsConfig,
};

#[derive(Debug)]
pub enum ServiceAnalyticsEvent {
    Screen {
        content: String,
    },
    Event {
        category: String,
        action: String,
        label: String,
        value: String,
    },
}

pub struct AnalyticsService {
    analytics_enabled: Option<bool>,
    worker_handle: tokio::task::JoinHandle<()>,
    sender: UnboundedSender<AnalyticsEvent>,
}

impl Default for AnalyticsService {
    fn default() -> Self {
        let (sender, handle) = Self::build_worker();

        Self {
            analytics_enabled: None,
            worker_handle: handle,
            sender,
        }
    }
}

impl AnalyticsService {
    /// Builds a handle to the analytics thread
    fn build_worker() -> (UnboundedSender<AnalyticsEvent>, JoinHandle<()>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let mut user_defaults = UserDefaults::standard();
        let analytics_id = user_defaults
            .get("analytics_id")
            .map(|value| value.as_str().map(|s| s.to_string()))
            .flatten()
            .unwrap_or_else(|| {
                let uuid = uuid::Uuid::new_v4().to_string();
                user_defaults.insert("analytics_id", cacao::defaults::Value::String(uuid.clone()));
                uuid
            });
        let worker = AnalyticsWorker::new(
            Default::default(),
            Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
                "UA-74188650-8",
            ))),
            ClientMetadata::new(analytics_id), // <- this should be an anonymous client-id
            receiver,
        );
        let handle = ActorSystem::current().spawn_result(async move { worker.spawn() });
        (sender, handle)
    }
}

impl Actor for AnalyticsService {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.analytics_enabled = UserDefaults::standard()
            .get("analytics_enabled")
            .map(|value| value.as_bool())
            .flatten();
    }
}

#[derive(Message)]
#[rtype(result = "Option<bool>")]
pub struct GetAnalyticsEnabled;

impl Handler<GetAnalyticsEnabled> for AnalyticsService {
    type Result = Option<bool>;

    fn handle(&mut self, _msg: GetAnalyticsEnabled, _ctx: &mut Self::Context) -> Self::Result {
        self.analytics_enabled.into()
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct SendAnalyticsEvent(pub ServiceAnalyticsEvent);

impl Handler<SendAnalyticsEvent> for AnalyticsService {
    type Result = ();

    fn handle(&mut self, msg: SendAnalyticsEvent, _ctx: &mut Self::Context) -> Self::Result {
        let analytics_enabled = self.analytics_enabled.unwrap_or(false);
        if !analytics_enabled {
            return;
        }

        log::info!("Firing analytics event {:?}", msg.0);
        let event = match msg.0 {
            ServiceAnalyticsEvent::Screen { content } => AnalyticsEvent::ScreenView {
                application: "continuous_looper".to_string(),
                application_version: "1.x".to_string(),
                application_id: None,
                application_installer_id: None,
                content,
            },
            ServiceAnalyticsEvent::Event {
                category,
                action,
                value,
                label,
            } => AnalyticsEvent::Event {
                category,
                action,
                label: Some(label),
                value: Some(value),
            },
        };
        if let Err(err) = self.sender.send(event) {
            log::error!("Analytics thread closed {}, restarting...", err);
            let (sender, join_handle) = Self::build_worker();
            self.sender = sender;
            self.worker_handle = join_handle;
        }
    }
}

#[derive(Message)]
#[rtype("()")]
pub struct SetAnalyticsEnabled(pub bool);

impl Handler<SetAnalyticsEnabled> for AnalyticsService {
    type Result = ();

    fn handle(&mut self, msg: SetAnalyticsEnabled, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Set analytics enabled {}", msg.0);
        self.analytics_enabled = Some(msg.0);
        let mut user_defaults = UserDefaults::standard();
        user_defaults.insert("analytics_enabled", cacao::defaults::Value::Bool(msg.0));

        let now = SystemTime::now();
        let now: chrono::DateTime<chrono::Utc> = now.into();
        let now = now.to_rfc3339();
        user_defaults.insert(
            "analytics_enabled__updated_at",
            cacao::defaults::Value::String(now),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reset() {
        cacao::defaults::UserDefaults::standard().remove("analytics_enabled");
    }

    #[actix::test]
    async fn test_analytics_service_set_defaults() {
        let analytics_service = AnalyticsService::default().start();
        let result = analytics_service.send(GetAnalyticsEnabled).await.unwrap();
        assert_eq!(result, None);
    }

    #[actix::test]
    async fn test_analytics_service_starts() {
        let analytics_service = AnalyticsService::default().start();
        let result = analytics_service.send(GetAnalyticsEnabled).await.unwrap();
        assert_eq!(result, None);
    }
}
