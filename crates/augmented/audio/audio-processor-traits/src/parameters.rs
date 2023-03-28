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
//! Provides a basic mechanism for defining float parameters and modifying them
//! through introspection at runtime.

use std::convert::TryFrom;

use audio_garbage_collector::{make_shared, Shared};

/// A shared reference to a boxed generic handle
pub type AudioProcessorHandleRef = Shared<Box<dyn AudioProcessorHandle>>;

/// Build a shared reference to a boxed generic handle
pub fn make_handle_ref<T: AudioProcessorHandle + 'static>(v: T) -> AudioProcessorHandleRef {
    make_shared(Box::new(v))
}

/// A type which can create an `AudioProcessorHandleRef`
pub trait AudioProcessorHandleProvider {
    fn generic_handle(&self) -> AudioProcessorHandleRef;
}

/// An empty handle with no parameters
pub struct AudioProcessorEmptyHandle;

impl AudioProcessorHandle for AudioProcessorEmptyHandle {
    fn parameter_count(&self) -> usize {
        0
    }

    fn get_parameter_spec(&self, _index: usize) -> ParameterSpec {
        panic!("There are no parameter specs")
    }

    fn get_parameter(&self, _index: usize) -> Option<ParameterValue> {
        None
    }

    fn set_parameter(&self, _index: usize, _request: ParameterValue) {}
}

/// This trait can be implemented by AudioProcessor handles to provide runtime introspection on
/// the parameters that a processor provides.
pub trait AudioProcessorHandle: Send + Sync {
    /// This method should return the name of the processor. This may displayed in a GUI application
    /// as the effect/instrument name.
    fn name(&self) -> String {
        "AudioProcessorHandle::name can be set at the processor handle with a name for the handle"
            .to_string()
    }

    /// Should return the number of parameters.
    fn parameter_count(&self) -> usize;

    /// After finding the number of parameters a callee will get `ParameterSpec` declarations
    /// giving more metadata about this parameter.
    fn get_parameter_spec(&self, index: usize) -> ParameterSpec;

    /// Should return the value for the parameter at this index
    fn get_parameter(&self, index: usize) -> Option<ParameterValue>;

    /// Should set the value for the parameter at this index
    fn set_parameter(&self, index: usize, request: ParameterValue);
}

/// A runtime typed parameter value
#[derive(PartialEq, Clone, Debug)]
pub enum ParameterValue {
    Float { value: f32 },
}

impl From<f32> for ParameterValue {
    fn from(value: f32) -> Self {
        Self::Float { value }
    }
}

impl TryFrom<ParameterValue> for f32 {
    type Error = ();

    fn try_from(value: ParameterValue) -> Result<Self, Self::Error> {
        let ParameterValue::Float { value } = value;
        Ok(value)
    }
}

#[derive(Debug, Clone)]
pub struct FloatType {
    pub range: (f32, f32),
    pub step: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    Float(FloatType),
}

impl ParameterType {
    pub fn float(&self) -> Option<&FloatType> {
        let ParameterType::Float(inner) = self;
        Some(inner)
    }
}

/// Meta-data around a parameter. A GUI application may use this information to display
/// the label around the parameter and decide what type of control to render to modify it.
#[derive(Debug, Clone)]
pub struct ParameterSpec {
    name: String,
    ty: ParameterType,
}

impl ParameterSpec {
    pub fn new(name: String, ty: ParameterType) -> Self {
        ParameterSpec { name, ty }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &ParameterType {
        &self.ty
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    #[test]
    fn test_parameter_value() {
        use super::ParameterValue;
        let v = ParameterValue::Float { value: 0.5 };
        assert_eq!(v, 0.5.into());
        assert_eq!(f32::try_from(v).unwrap(), 0.5);
    }

    #[test]
    fn test_parameter_type() {
        use super::{FloatType, ParameterType};
        let ty = ParameterType::Float(FloatType {
            range: (0.0, 1.0),
            step: None,
        });
        assert!(ty.float().is_some());
    }

    #[test]
    fn test_parameter_spec() {
        use super::{FloatType, ParameterSpec, ParameterType};
        let spec = ParameterSpec::new(
            "test".to_string(),
            ParameterType::Float(FloatType {
                range: (0.0, 1.0),
                step: None,
            }),
        );
        assert_eq!(spec.name(), "test");
        assert!(spec.ty().float().is_some());
    }

    #[test]
    fn test_parameter_value_from_f32() {
        use super::ParameterValue;
        let v = ParameterValue::Float { value: 0.5 };
        assert_eq!(v, 0.5.into());
    }
}
