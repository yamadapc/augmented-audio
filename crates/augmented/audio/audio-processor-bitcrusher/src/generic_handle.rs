use audio_garbage_collector::Shared;
use audio_processor_traits::parameters::{
    AudioProcessorHandle, FloatType, ParameterSpec, ParameterType, ParameterValue,
};

use super::BitCrusherHandle;

pub struct BitCrusherHandleRef(Shared<BitCrusherHandle>);

impl BitCrusherHandleRef {
    pub fn new(inner: Shared<BitCrusherHandle>) -> Self {
        BitCrusherHandleRef(inner)
    }
}

impl AudioProcessorHandle for BitCrusherHandleRef {
    fn parameter_count(&self) -> usize {
        1
    }

    fn get_parameter_spec(&self, _index: usize) -> ParameterSpec {
        ParameterSpec::new(
            "Bit rate".into(),
            ParameterType::Float(FloatType {
                range: (100.0, self.0.sample_rate()),
                step: None,
            }),
        )
    }

    fn get_parameter(&self, _index: usize) -> Option<ParameterValue> {
        Some(ParameterValue::Float {
            value: self.0.bit_rate(),
        })
    }

    fn set_parameter(&self, _index: usize, request: ParameterValue) {
        if let ParameterValue::Float { value } = request {
            self.0.set_bit_rate(value);
        }
    }
}
