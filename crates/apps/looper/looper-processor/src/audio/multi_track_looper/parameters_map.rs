use std::sync::atomic::AtomicBool;

use rustc_hash::FxHashMap;
use serde_derive::{Deserialize, Serialize};

use augmented_atomics::AtomicValue;

use crate::parameters::{
    build_default_parameters, build_parameter_indexes, ParameterId, ParameterValue,
};

/// Storage for parameters, parameters are copied on changes
#[derive(Debug, Serialize, Deserialize)]
pub struct ParametersMap {
    values: Vec<ParameterValue>,
    has_value: Vec<AtomicBool>,
    // Uses fast insecure hash function (around 2x speedup)
    indexes: FxHashMap<ParameterId, usize>,
}

impl Default for ParametersMap {
    fn default() -> Self {
        Self::new()
    }
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

    #[inline]
    pub fn get(&self, id: impl Into<ParameterId>) -> &ParameterValue {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        &self.values[index]
    }

    #[inline]
    pub fn get_option(&self, id: impl Into<ParameterId>) -> Option<&ParameterValue> {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        if self.has_value[index].get() {
            Some(&self.values[index])
        } else {
            None
        }
    }

    /// Returns true if the parameter has been set after the default
    #[inline]
    pub fn has_value(&self, id: impl Into<ParameterId>) -> bool {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        self.has_value[index].get()
    }

    #[inline]
    pub fn set(&self, id: impl Into<ParameterId>, value: impl Into<ParameterValue>) {
        let id: ParameterId = id.into();
        let value: ParameterValue = value.into();

        let index: usize = self.indexes[&id];
        let slot: &ParameterValue = &self.values[index];
        slot.set_from(&value);
        self.has_value[index].set(true);
    }

    #[inline]
    pub fn unset(&self, id: impl Into<ParameterId>) {
        let id: ParameterId = id.into();
        let index: usize = self.indexes[&id];
        self.has_value[index].set(false);
    }
}

impl Clone for ParametersMap {
    fn clone(&self) -> Self {
        Self {
            values: self.values.iter().map(|value| value.clone()).collect(),
            has_value: self
                .has_value
                .iter()
                .map(|has_value| has_value.get().into())
                .collect(),
            indexes: self.indexes.clone(),
        }
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
        assert_eq!(ps.get(SourceParameter::Start).as_float(), 0.0_f32);
        ps.set(SourceParameter::Start, 0.5);
        assert_eq!(ps.get(SourceParameter::Start).as_float(), 0.5_f32);
    }
}
