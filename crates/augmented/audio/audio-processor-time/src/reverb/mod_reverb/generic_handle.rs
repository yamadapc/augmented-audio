use audio_garbage_collector::Shared;
use audio_processor_traits::parameters::{AudioProcessorHandle, ParameterSpec, ParameterValue};

use crate::mod_reverb::ModReverbHandle;

pub struct GenericHandle(pub Shared<ModReverbHandle>);

impl AudioProcessorHandle for GenericHandle {
    fn parameter_count(&self) -> usize {
        0
    }

    fn get_parameter_spec(&self, _index: usize) -> ParameterSpec {
        todo!("NOT IMPLEMENTED")
    }

    fn get_parameter(&self, _index: usize) -> Option<ParameterValue> {
        None
    }

    fn set_parameter(&self, _index: usize, _request: ParameterValue) {
        todo!("NOT IMPLEMENTED")
    }
}
