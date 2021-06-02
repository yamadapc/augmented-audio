use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ParameterType {
    Number,
}

impl Default for ParameterType {
    fn default() -> Self {
        ParameterType::Number
    }
}

/// Simple implementation of a parameter
pub struct PluginParameter {
    /// This is the only mutable field.
    ///
    /// Not all platforms will support this. This will not work properly depending on whether the
    /// CPU supports atomic f32 instructions.
    value: AtomicCell<f32>,
    name: String,
    label: String,
    can_be_automated: bool,
    value_range: (f32, f32),
    value_type: ParameterType,
    value_precision: u32,
}

impl PluginParameter {
    pub fn new(
        value: AtomicCell<f32>,
        name: String,
        label: String,
        can_be_automated: bool,
        value_range: (f32, f32),
        value_type: ParameterType,
        value_precision: u32,
    ) -> Self {
        PluginParameter {
            value,
            name,
            label,
            can_be_automated,
            value_range,
            value_type,
            value_precision,
        }
    }

    pub fn builder() -> PluginParameterBuilder {
        PluginParameterBuilder::new()
    }
}

unsafe impl Send for PluginParameter {}
unsafe impl Sync for PluginParameter {}

impl PluginParameter {}

impl PluginParameter {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn text(&self) -> String {
        format!("{}", self.value.load())
    }

    pub fn value(&self) -> f32 {
        self.value.load()
    }

    pub fn set_value(&self, value: f32) {
        self.value.store(value)
    }

    pub fn can_be_automated(&self) -> bool {
        self.can_be_automated
    }

    pub fn value_range(&self) -> (f32, f32) {
        self.value_range
    }

    pub fn value_type(&self) -> ParameterType {
        self.value_type
    }

    pub fn value_precision(&self) -> u32 {
        self.value_precision
    }
}

pub struct PluginParameterBuilder {
    initial_value: Option<f32>,
    name: Option<String>,
    label: Option<String>,
    can_be_automated: Option<bool>,
    value_range: Option<(f32, f32)>,
    value_type: Option<ParameterType>,
    value_precision: Option<u32>,
}

impl PluginParameterBuilder {
    fn new() -> Self {
        PluginParameterBuilder {
            initial_value: None,
            name: None,
            label: None,
            can_be_automated: None,
            value_range: None,
            value_type: None,
            value_precision: None,
        }
    }

    pub fn initial_value(mut self, value: f32) -> Self {
        self.initial_value = Some(value);
        self
    }

    pub fn name(mut self, value: &str) -> Self {
        self.name = Some(value.to_string());
        self
    }

    pub fn label(mut self, value: &str) -> Self {
        self.label = Some(value.to_string());
        self
    }

    pub fn can_be_automated(mut self, value: bool) -> Self {
        self.can_be_automated = Some(value);
        self
    }

    pub fn value_range(mut self, min: f32, max: f32) -> Self {
        self.value_range = Some((min, max));
        self
    }

    pub fn value_type(mut self, value_type: ParameterType) -> Self {
        self.value_type = Some(value_type);
        self
    }

    pub fn value_precision(mut self, value_precision: u32) -> Self {
        self.value_precision = Some(value_precision);
        self
    }

    pub fn build(self) -> PluginParameter {
        PluginParameter::new(
            AtomicCell::new(self.initial_value.unwrap_or(0.0)),
            self.name.unwrap_or_else(|| "".to_string()),
            self.label.unwrap_or_else(|| "".to_string()),
            self.can_be_automated.unwrap_or(true),
            self.value_range.unwrap_or((0., 1.)),
            self.value_type.unwrap_or_default(),
            self.value_precision.unwrap_or(2),
        )
    }
}
