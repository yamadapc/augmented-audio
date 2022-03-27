use crate::parameters::{build_default_parameters, ParameterId, ParameterValue};
use audio_processor_standalone::standalone_vst::vst::util::ParameterTransfer;
use std::collections::HashMap;

/// Storage for parameters, parameters are copied on changes
pub struct ParametersMap {
    values: Vec<ParameterValue>,
    indexes: HashMap<ParameterId, usize>,
}

impl ParametersMap {
    pub fn new() -> Self {
        let (values, parameter_ids) = build_default_parameters();
        let values = parameter_ids
            .iter()
            .map(|id| values.get(id).unwrap().val().clone())
            .collect();
        let indexes = parameter_ids
            .iter()
            .enumerate()
            .map(|(index, id)| (id.clone(), index))
            .collect();
        Self { values, indexes }
    }

    pub fn get(&self, id: impl Into<ParameterId>) -> &ParameterValue {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        &self.values[index]
    }

    pub fn set(&self, id: impl Into<ParameterId>, value: impl Into<ParameterValue>) {
        let id: ParameterId = id.into();
        let value: ParameterValue = value.into();

        let index: usize = self.indexes[&id];
        let slot: &ParameterValue = &self.values[index];
        slot.set_from(&value);
    }
}
