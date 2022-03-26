use augmented_atomics::AtomicF32;

use crate::parameters::ParameterId;

type LFOMap = lockfree::map::Map<ParameterId, f32>;

pub struct LFOHandle {
    amount: AtomicF32,
    frequency: AtomicF32,
    map: LFOMap,
}

impl Default for LFOHandle {
    fn default() -> Self {
        LFOHandle {
            amount: 1.0.into(),
            frequency: 1.0.into(),
            map: LFOMap::default(),
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
        self.map
            .get(parameter_id)
            .map(|v| v.val().clone())
            .unwrap_or(0.0)
    }

    pub fn set_amount(&self, value: f32) {
        self.amount.set(value);
    }

    pub fn set_frequency(&self, value: f32) {
        self.frequency.set(value);
    }

    pub fn set_parameter_map(&self, parameter_id: ParameterId, amount: Option<f32>) {
        if let Some(amount) = amount {
            self.map.insert(parameter_id, amount);
        } else {
            self.map.remove(&parameter_id);
        }
    }
}
