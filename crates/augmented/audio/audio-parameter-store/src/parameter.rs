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
            self.name.unwrap_or_default(),
            self.label.unwrap_or_default(),
            self.can_be_automated.unwrap_or(true),
            self.value_range.unwrap_or((0., 1.)),
            self.value_type.unwrap_or_default(),
            self.value_precision.unwrap_or(2),
        )
    }
}

/// Type of parameter. Only number for now.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ParameterType {
    Number,
}

impl Default for ParameterType {
    fn default() -> Self {
        ParameterType::Number
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_constructor() {
        let _ = PluginParameter::new(
            Default::default(),
            "".to_string(),
            "".to_string(),
            false,
            (0.0, 0.0),
            Default::default(),
            0,
        );
    }

    #[test]
    fn test_get_builder() {
        let _ = PluginParameter::builder();
    }

    #[test]
    fn test_build_and_get_name() {
        let parameter = PluginParameter::builder().name("Hello world").build();
        assert_eq!(parameter.name(), "Hello world");
    }

    #[test]
    fn test_build_and_get_label() {
        let parameter = PluginParameter::builder().label("Hz").build();
        assert_eq!(parameter.label(), "Hz");
    }

    #[test]
    fn test_build_and_get_can_be_automated() {
        let parameter = PluginParameter::builder().can_be_automated(false).build();
        assert_eq!(parameter.can_be_automated(), false);
    }

    #[test]
    fn test_build_and_get_value_range() {
        let parameter = PluginParameter::builder().value_range(0.0, 440.0).build();
        assert_eq!(parameter.value_range(), (0.0, 440.0));
    }

    #[test]
    fn test_build_and_get_value_type() {
        let parameter = PluginParameter::builder()
            .value_type(ParameterType::Number)
            .build();
        assert_eq!(parameter.value_type(), ParameterType::Number);
    }

    #[test]
    fn test_build_and_get_value_precision() {
        let parameter = PluginParameter::builder().value_precision(20).build();
        assert_eq!(parameter.value_precision(), 20);
    }

    #[test]
    fn test_get_and_set_value() {
        let parameter = PluginParameter::builder().initial_value(20.0).build();
        assert_f_eq!(parameter.value(), 20.0);
        parameter.set_value(30.0);
        assert_f_eq!(parameter.value(), 30.0);
    }

    #[test]
    fn test_default_parameter_type() {
        let default_type: ParameterType = Default::default();
        assert_eq!(default_type, ParameterType::Number);
    }
}
