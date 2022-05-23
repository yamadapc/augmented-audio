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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct ClientMetadata {
    pub client_id: String,
}

impl ClientMetadata {
    pub fn new(client_id: impl Into<String>) -> Self {
        ClientMetadata {
            client_id: client_id.into(),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
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
    pub fn screen() -> ScreenViewEventBuilder {
        ScreenViewEventBuilder::default()
    }

    pub fn event() -> EventBuilder {
        EventBuilder::default()
    }
}

pub struct EventBuilder {
    category: String,
    action: String,
    label: Option<String>,
    value: Option<String>,
}

impl Default for EventBuilder {
    fn default() -> Self {
        EventBuilder {
            category: "".to_string(),
            action: "".to_string(),
            label: None,
            value: None,
        }
    }
}

impl EventBuilder {
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(self) -> AnalyticsEvent {
        AnalyticsEvent::Event {
            category: self.category,
            action: self.action,
            label: self.label,
            value: self.value,
        }
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

    pub fn version(mut self, application_version: impl Into<String>) -> Self {
        self.application_version = application_version.into();
        self
    }

    pub fn id(mut self, application_id: impl Into<String>) -> Self {
        self.application_id = Some(application_id.into());
        self
    }

    pub fn installer_id(mut self, application_installer_id: impl Into<String>) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_client_metadata() {
        let client_metadata = ClientMetadata::new("xyz");
        assert_eq!(client_metadata.client_id, "xyz");
        assert_eq!(
            client_metadata,
            ClientMetadata {
                client_id: "xyz".into()
            }
        );
    }

    #[test]
    fn test_event_builder_works_with_all_fields() {
        let event = AnalyticsEvent::event()
            .value("10")
            .action("click")
            .label("sign-up button")
            .category("ui")
            .build();
        assert_eq!(
            event,
            AnalyticsEvent::Event {
                category: "ui".to_string(),
                action: "click".to_string(),
                label: Some("sign-up button".to_string()),
                value: Some("10".to_string())
            }
        );
    }

    #[test]
    fn test_event_builder_works_with_missing_optional_fields() {
        let event = AnalyticsEvent::event()
            .action("click")
            .category("ui")
            .build();
        assert_eq!(
            event,
            AnalyticsEvent::Event {
                category: "ui".to_string(),
                action: "click".to_string(),
                label: None,
                value: None
            }
        );
    }

    #[test]
    fn test_event_builder_returns_defaults() {
        let event = AnalyticsEvent::event().build();
        assert_eq!(event, AnalyticsEvent::default());
    }

    #[test]
    fn test_screen_builder_works_with_all_fields() {
        let screen = AnalyticsEvent::screen()
            .application("test-application")
            .version("0.99")
            .content("home")
            .id("app-id")
            .installer_id("installer-id")
            .build();
        assert_eq!(
            screen,
            AnalyticsEvent::ScreenView {
                application: "test-application".to_string(),
                application_version: "0.99".to_string(),
                application_id: Some("app-id".to_string()),
                application_installer_id: Some("installer-id".to_string()),
                content: "home".to_string()
            }
        );
    }

    #[test]
    fn test_screen_builder_returns_defaults() {
        let screen = AnalyticsEvent::screen().build();
        assert_eq!(
            screen,
            AnalyticsEvent::ScreenView {
                application: "".to_string(),
                application_version: "".to_string(),
                application_id: None,
                application_installer_id: None,
                content: "".to_string()
            }
        );
    }
}
