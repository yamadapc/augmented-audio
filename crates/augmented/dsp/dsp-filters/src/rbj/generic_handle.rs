use audio_processor_traits::parameters::{
    AudioProcessorHandle, FloatType, ParameterSpec, ParameterType, ParameterValue,
};

pub struct GenericHandle {}

impl AudioProcessorHandle for GenericHandle {
    fn name(&self) -> String {
        "Filter".to_string()
    }

    fn parameter_count(&self) -> usize {
        0
    }

    fn get_parameter_spec(&self, _index: usize) -> ParameterSpec {
        ParameterSpec::new(
            "FilterProcessor - No parameter".to_string(),
            ParameterType::Float(FloatType {
                range: (0.0, 0.0),
                step: None,
            }),
        )
    }

    fn get_parameter(&self, _index: usize) -> Option<ParameterValue> {
        None
    }

    fn set_parameter(&self, _index: usize, _request: ParameterValue) {}
}
