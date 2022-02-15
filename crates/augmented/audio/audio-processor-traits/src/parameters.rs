use std::sync::Arc;

pub type AudioProcessorHandleRef = Arc<dyn AudioProcessorHandle>;

/// This trait can be implemented by AudioProcessor handles to provide runtime introspection on
/// the parameters that a processor provides.
pub trait AudioProcessorHandle: Send + Sync {
    fn parameter_count(&self) -> usize;
    fn get_parameter_spec(&self, index: usize) -> ParameterSpec;

    fn get_parameter(&self, index: usize) -> Option<ParameterValue>;
    fn set_parameter(&self, index: usize, request: ParameterValue);
}

pub enum ParameterValue {
    Float { value: f32 },
}

pub enum ParameterType {
    Float {
        range: (f32, f32),
        step: Option<f32>,
    },
}

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
