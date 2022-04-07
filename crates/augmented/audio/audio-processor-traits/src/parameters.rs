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
use std::convert::TryFrom;

use audio_garbage_collector::{make_shared, Shared};

pub type AudioProcessorHandleRef = Shared<Box<dyn AudioProcessorHandle>>;
pub fn make_handle_ref<T: AudioProcessorHandle + 'static>(v: T) -> AudioProcessorHandleRef {
    make_shared(Box::new(v))
}

pub trait AudioProcessorHandleProvider {
    fn generic_handle(&self) -> AudioProcessorHandleRef;
}

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
    fn parameter_count(&self) -> usize;
    fn get_parameter_spec(&self, index: usize) -> ParameterSpec;

    fn get_parameter(&self, index: usize) -> Option<ParameterValue>;
    fn set_parameter(&self, index: usize, request: ParameterValue);
}

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

#[derive(Clone)]
pub struct FloatType {
    pub range: (f32, f32),
    pub step: Option<f32>,
}

#[derive(Clone)]
pub enum ParameterType {
    Float(FloatType),
}

impl ParameterType {
    pub fn float(&self) -> Option<&FloatType> {
        let ParameterType::Float(inner) = self;
        Some(inner)
    }
}

#[derive(Clone)]
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
