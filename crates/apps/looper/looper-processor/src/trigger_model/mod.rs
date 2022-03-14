use std::ops::Deref;
use std::sync::atomic::AtomicUsize;

use basedrop::{Shared, SharedCell};
use im::Vector;

use audio_garbage_collector::{make_shared, make_shared_cell};
use augmented_atomics::{AtomicF32, AtomicValue};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct LoopTrigger {}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum TriggerInner {
    LoopTrigger(LoopTrigger),
}

#[derive(Debug)]
pub enum TriggerPosition {
    BeatsUsize { pos: AtomicUsize },
    BeatsF32 { pos: AtomicF32 },
}

impl Default for TriggerPosition {
    fn default() -> Self {
        Self::BeatsF32 { pos: 0.0.into() }
    }
}

impl PartialEq for TriggerPosition {
    fn eq(&self, other: &Self) -> bool {
        use TriggerPosition::*;
        match self {
            BeatsUsize { pos } => {
                if let BeatsUsize { pos: pos2 } = other {
                    pos.get() == pos2.get()
                } else {
                    false
                }
            }
            BeatsF32 { pos } => {
                if let BeatsF32 { pos: pos2 } = other {
                    pos.get() == pos2.get()
                } else {
                    false
                }
            }
        }
    }
}

impl TriggerPosition {
    fn beats(&self) -> f32 {
        match self {
            TriggerPosition::BeatsUsize { pos } => pos.get() as f32,
            TriggerPosition::BeatsF32 { pos } => pos.get(),
        }
    }
}

impl Clone for TriggerPosition {
    fn clone(&self) -> Self {
        use TriggerPosition::*;

        match self {
            BeatsUsize { pos } => BeatsUsize {
                pos: pos.get().into(),
            },
            BeatsF32 { pos } => BeatsF32 { pos: pos.clone() },
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Trigger {
    inner: TriggerInner,
    position: TriggerPosition,
}

impl Default for Trigger {
    fn default() -> Self {
        Trigger {
            inner: TriggerInner::LoopTrigger(LoopTrigger::default()),
            position: TriggerPosition::default(),
        }
    }
}

impl Trigger {
    pub fn beats(&self) -> f32 {
        self.position.beats()
    }

    pub fn set_position(&mut self, position: TriggerPosition) {
        self.position = position;
    }
}

pub struct TrackTriggerModel {
    triggers: SharedCell<Vector<Trigger>>,
}

impl Default for TrackTriggerModel {
    fn default() -> Self {
        Self {
            triggers: make_shared_cell(Vector::default()),
        }
    }
}

impl TrackTriggerModel {
    pub fn len(&self) -> usize {
        self.triggers.get().len()
    }

    pub fn add_trigger(&self, trigger: Trigger) {
        let triggers = self.triggers.get();
        let mut triggers: Vector<Trigger> = (*triggers).clone();
        triggers.push_back(trigger);
        log::info!("Track triggers={:?}", triggers);
        self.triggers.set(make_shared(triggers));
    }

    pub fn add_triggers(&self, triggers: &[Trigger]) {
        let mut triggers: Vector<Trigger> = Vector::from(triggers);
        self.triggers.set(make_shared(triggers));
    }

    pub fn triggers(&self) -> Shared<Vector<Trigger>> {
        self.triggers.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_and_add_triggers() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_trigger(Trigger::default());
        let triggers = trigger_model.triggers();
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers.get(0).cloned(), Some(Trigger::default()));
        assert_eq!(trigger_model.len(), 1);
    }

    #[test]
    fn test_bulk_add_triggers() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_triggers(&[Trigger::default(), Trigger::default(), Trigger::default()]);
        let triggers = trigger_model.triggers();
        assert_eq!(trigger_model.len(), 3);
        assert_eq!(triggers.len(), 3);
        assert_eq!(triggers.get(0).cloned(), Some(Trigger::default()));
        assert_eq!(triggers.get(1).cloned(), Some(Trigger::default()));
        assert_eq!(triggers.get(2).cloned(), Some(Trigger::default()));
    }
}
