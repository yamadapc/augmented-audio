use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
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
