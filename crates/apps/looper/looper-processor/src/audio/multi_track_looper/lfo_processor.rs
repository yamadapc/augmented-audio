use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use augmented_atomics::AtomicF32;

use crate::parameters::{build_default_parameters, build_parameter_indexes, ParameterId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LFOHandleMap {
    values: Vec<AtomicF32>,
    indexes: FxHashMap<ParameterId, usize>,
}

impl LFOHandleMap {
    pub fn get(&self, id: &ParameterId) -> f32 {
        self.values[self.indexes[id]].get()
    }
}

pub struct LFOHandle {
    amount: AtomicF32,
    frequency: AtomicF32,
    map: LFOHandleMap,
}

impl Default for LFOHandle {
    fn default() -> Self {
        LFOHandle {
            amount: 1.0.into(),
            frequency: 1.0.into(),
            map: LFOHandleMap {
                values: build_default_parameters()
                    .1
                    .iter()
                    .map(|_| 0.0.into())
                    .collect(),
                indexes: build_parameter_indexes(&build_default_parameters().1),
            },
        }
    }
}

impl LFOHandle {
    pub fn amount(&self) -> f32 {
        self.amount.get()
    }

    pub fn frequency(&self) -> f32 {
        self.frequency.get()
    }

    pub fn set_amount(&self, value: f32) {
        self.amount.set(value);
    }

    pub fn set_frequency(&self, value: f32) {
        self.frequency.set(value);
    }

    pub fn modulation_amount(&self, parameter_id: &ParameterId) -> f32 {
        let index = self.map.indexes[parameter_id];
        self.map.values[index].get()
    }

    pub fn set_parameter_map(&self, parameter_id: ParameterId, amount: Option<f32>) {
        let index = self.map.indexes[&parameter_id];
        if let Some(amount) = amount {
            self.map.values[index].set(amount);
        } else {
            self.map.values[index].set(0.0);
        }
    }

    pub fn map(&self) -> &LFOHandleMap {
        &self.map
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use crate::parameters::SourceParameter;

    use super::*;

    #[test]
    fn test_create_lfo_handle() {
        let _handle = LFOHandle::default();
    }

    #[test]
    fn test_set_amount() {
        let handle = LFOHandle::default();
        handle.set_amount(0.88);
        assert_f_eq!(handle.amount(), 0.88);
    }

    #[test]
    fn test_set_frequency() {
        let handle = LFOHandle::default();
        handle.set_frequency(44.44);
        assert_f_eq!(handle.frequency(), 44.44);
    }

    #[test]
    fn test_add_modulation() {
        let handle = LFOHandle::default();
        let amount = handle.modulation_amount(&SourceParameter::Start.into());
        assert_f_eq!(amount, 0.0);

        handle.set_parameter_map(SourceParameter::Start.into(), Some(0.5));

        let amount = handle.modulation_amount(&SourceParameter::Start.into());
        assert_f_eq!(amount, 0.5);
    }

    #[test]
    fn test_remove_modulation() {
        let handle = LFOHandle::default();
        let amount = handle.modulation_amount(&SourceParameter::Start.into());
        assert_f_eq!(amount, 0.0);

        handle.set_parameter_map(SourceParameter::Start.into(), Some(0.5));
        handle.set_parameter_map(SourceParameter::Start.into(), None);

        let amount = handle.modulation_amount(&SourceParameter::Start.into());
        assert_f_eq!(amount, 0.0);
    }
}
