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
            values: self.values.to_vec(),
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
    fn test_set_int_parameter() {
        let ps = ParametersMap::new();
        assert_eq!(ps.get(SourceParameter::SliceId).as_int(), 0);
        ps.set(SourceParameter::SliceId, 2);
        assert_eq!(ps.get(SourceParameter::SliceId).as_int(), 2);
    }

    #[test]
    fn test_set_and_get_a_value() {
        let ps = ParametersMap::new();
        assert_eq!(ps.get(SourceParameter::Start).as_float(), 0.0_f32);
        ps.set(SourceParameter::Start, 0.5);
        assert_eq!(ps.get(SourceParameter::Start).as_float(), 0.5_f32);
    }
}
