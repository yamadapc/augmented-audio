use crate::MonoDelayProcessorHandle;
use audio_garbage_collector::Shared;
use audio_processor_traits::parameters::{
    AudioProcessorHandle, FloatType, ParameterSpec, ParameterType, ParameterValue,
};

pub struct GenericHandle(pub Shared<MonoDelayProcessorHandle>);

impl AudioProcessorHandle for GenericHandle {
    fn name(&self) -> String {
        "Delay".to_string()
    }

    fn parameter_count(&self) -> usize {
        2
    }

    fn get_parameter_spec(&self, index: usize) -> ParameterSpec {
        let specs = [
            ParameterSpec::new(
                "Delay".into(),
                ParameterType::Float(FloatType {
                    range: (0.01, 5.0),
                    step: None,
                }),
            ),
            ParameterSpec::new(
                "Feedback".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
        ];
        specs[index].clone()
    }

    fn get_parameter(&self, index: usize) -> Option<ParameterValue> {
        if index == 0 {
            Some(self.0.delay_time_secs.get().into())
        } else if index == 1 {
            Some(self.0.feedback.get().into())
        } else {
            None
        }
    }

    fn set_parameter(&self, index: usize, request: ParameterValue) {
        if let Ok(value) = request.try_into() {
            if index == 0 {
                self.0.delay_time_secs.set(value);
            } else if index == 1 {
                self.0.feedback.set(value);
            }
        }
    }
}
