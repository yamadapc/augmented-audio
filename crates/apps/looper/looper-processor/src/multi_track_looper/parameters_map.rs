use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

use augmented_atomics::AtomicValue;

use crate::parameters::{
    build_default_parameters, build_parameter_indexes, ParameterId, ParameterValue,
};

/// Storage for parameters, parameters are copied on changes
pub struct ParametersMap {
    values: Vec<ParameterValue>,
    has_value: Vec<AtomicBool>,
    indexes: HashMap<ParameterId, usize>,
}

impl ParametersMap {
    pub fn new() -> Self {
        let (values, parameter_ids) = build_default_parameters();
        let values = parameter_ids
            .iter()
            .map(|id| values.get(id).unwrap().clone())
            .collect();
        let indexes = build_parameter_indexes(&parameter_ids);
        let has_value = parameter_ids.iter().map(|_| false.into()).collect();
        Self {
            values,
            has_value,
            indexes,
        }
    }

    pub fn get(&self, id: impl Into<ParameterId>) -> &ParameterValue {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        &self.values[index]
    }

    /// Returns true if the parameter has been set after the default
    pub fn has_value(&self, id: impl Into<ParameterId>) -> bool {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        self.has_value[index].get()
    }

    pub fn set(&self, id: impl Into<ParameterId>, value: impl Into<ParameterValue>) {
        let id: ParameterId = id.into();
        let value: ParameterValue = value.into();

        let index: usize = self.indexes[&id];
        let slot: &ParameterValue = &self.values[index];
        slot.set_from(&value);
        self.has_value[index].set(true);
    }
}

#[cfg(test)]
mod test {
    use crate::parameters::SourceParameter;

    use super::*;

    #[test]
    fn test_create_parameters() {
        let _ps = ParametersMap::new();
    }

    #[test]
    fn test_set_and_get_a_value() {
        let ps = ParametersMap::new();
        assert_eq!(ps.get(SourceParameter::Start).inner_float(), 0.0_f32);
        ps.set(SourceParameter::Start, 0.5);
        assert_eq!(ps.get(SourceParameter::Start).inner_float(), 0.5_f32);
    }
}
