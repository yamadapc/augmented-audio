use augmented_atomics::AtomicF32;
use std::collections::HashMap;

use crate::parameters::{build_default_parameters, build_parameter_indexes, ParameterId};

type LFOMap = lockfree::map::Map<ParameterId, f32>;

pub struct LFOHandle {
    amount: AtomicF32,
    frequency: AtomicF32,
    values: Vec<AtomicF32>,
    indexes: HashMap<ParameterId, usize>,
}

impl Default for LFOHandle {
    fn default() -> Self {
        LFOHandle {
            amount: 1.0.into(),
            frequency: 1.0.into(),
            values: build_default_parameters()
                .1
                .iter()
                .map(|_| 0.0.into())
                .collect(),
            indexes: build_parameter_indexes(&build_default_parameters().1),
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

    pub fn modulation_amount(&self, parameter_id: &ParameterId) -> f32 {
        let index = self.indexes[parameter_id];
        self.values[index].get()
    }

    pub fn set_amount(&self, value: f32) {
        self.amount.set(value);
    }

    pub fn set_frequency(&self, value: f32) {
        self.frequency.set(value);
    }

    pub fn set_parameter_map(&self, parameter_id: ParameterId, amount: Option<f32>) {
        let index = self.indexes[&parameter_id.into()];
        if let Some(amount) = amount {
            self.values[index].set(amount);
        } else {
            self.values[index].set(0.0);
        }
    }
}
