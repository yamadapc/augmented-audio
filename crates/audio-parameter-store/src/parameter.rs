use crossbeam::atomic::AtomicCell;
use serde::{Deserialize, Serialize};

pub trait PluginParameterLike {
    fn name(&self) -> String;
    /// This is the suffix to be used after parameter values.
    fn label(&self) -> String;
    fn text(&self) -> String;
    fn value(&self) -> f32;
    fn set_value(&self, value: f32);
    fn can_be_automated(&self) -> bool;
    fn value_range(&self) -> (f32, f32);
    fn value_type(&self) -> ParameterType;
    fn value_precision(&self) -> u32;
}

/// Simple implementation of a parameter.
///
/// Wraps the current value in an "AtomicCell". All other fields are immutable.
///
/// Create parameters using `PluginParameter::builder()` / `PluginParameterBuilder`.
#[derive(Debug)]
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

unsafe impl Send for PluginParameter {}
unsafe impl Sync for PluginParameter {}

impl PluginParameter {
    /// Create a parameter with given options
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

    /// Create a parameter builder
    pub fn builder() -> PluginParameterBuilder {
        PluginParameterBuilder::new()
    }
}

impl PluginParameterLike for PluginParameter {
    /// Get the parameter name
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the parameter label. This is the suffix to be used after parameter values.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Get the parameter value as text.
    fn text(&self) -> String {
        format!("{}", self.value.load())
    }

    /// Get the parameter current value.
    fn value(&self) -> f32 {
        self.value.load()
    }

    /// Set the parameter current value.
    fn set_value(&self, value: f32) {
        self.value.store(value)
    }

    fn can_be_automated(&self) -> bool {
        self.can_be_automated
    }

    /// Max and minimum range for the parameter
    fn value_range(&self) -> (f32, f32) {
        self.value_range
    }

    /// Type of the parameter (to be expanded)
    fn value_type(&self) -> ParameterType {
        self.value_type
    }

    /// Precision (in nº of digits) to be rendered by the front-end
    fn value_precision(&self) -> u32 {
        self.value_precision
    }
}

/// Builder for `PluginParameter`
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

/// Type of parameter. Only number for now.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ParameterType {
    Number,
}

impl Default for ParameterType {
    fn default() -> Self {
        ParameterType::Number
    }
}
