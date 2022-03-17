pub mod step_tracker;

use std::ops::Deref;
use std::sync::atomic::AtomicUsize;

use basedrop::{Shared, SharedCell};
use im::Vector;

use audio_garbage_collector::{make_shared, make_shared_cell};
use augmented_atomics::{AtomicF32, AtomicValue};
use step_tracker::StepTracker;

pub fn find_current_beat_trigger(
    track_trigger_model: &TrackTriggerModel,
    step_tracker: &mut StepTracker,
    position_beats: f64,
) -> Option<Trigger> {
    step_tracker
        .accept(
            track_trigger_model.pattern_step_beats,
            position_beats
                % (track_trigger_model.pattern_length as f64
                    * track_trigger_model.pattern_step_beats),
        )
        .map(|step| track_trigger_model.find_step(step))
        .flatten()
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct LoopTrigger {}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum TriggerInner {
    LoopTrigger(LoopTrigger),
}

#[derive(Debug)]
pub struct TriggerPosition {
    step: AtomicUsize,
    position: AtomicF32,
}

impl Default for TriggerPosition {
    fn default() -> Self {
        Self {
            step: 0.into(),
            position: 0.0.into(),
        }
    }
}

impl PartialEq for TriggerPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position.get() == other.position.get() && self.step.get() == other.step.get()
    }
}

impl TriggerPosition {
    fn beats(&self) -> f32 {
        self.position.get()
    }
}

impl Clone for TriggerPosition {
    fn clone(&self) -> Self {
        Self {
            step: self.step.get().into(),
            position: self.position.get().into(),
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
    pattern_length: usize,
    pattern_step_beats: f64,
    triggers: SharedCell<Vector<Trigger>>,
}

impl Default for TrackTriggerModel {
    fn default() -> Self {
        Self {
            pattern_length: 16,
            pattern_step_beats: 0.25,
            triggers: make_shared_cell(Vector::default()),
        }
    }
}

impl TrackTriggerModel {
    pub fn num_triggers(&self) -> usize {
        self.triggers.get().len()
    }

    pub fn find_step(&self, step: usize) -> Option<Trigger> {
        let triggers = self.triggers.get();
        triggers
            .iter()
            .find(|trigger| trigger.position.step.get() == step)
            .cloned()
    }

    pub fn toggle_trigger(&self, position_beats: usize) {
        let triggers = self.triggers.get();
        let mut triggers: Vector<Trigger> = (*triggers).clone();

        let indexes: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(idx, trigger)| trigger.position.step.get() == position_beats)
            .map(|(idx, _)| idx)
            .collect();
        if !indexes.is_empty() {
            for index in indexes {
                triggers.remove(index);
            }
            self.triggers.set(make_shared(triggers));
        } else {
            let mut trigger = Trigger::default();
            trigger.set_position(TriggerPosition {
                step: position_beats.into(),
                position: (position_beats as f32).into(),
            });
            self.add_trigger(trigger);
        }
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
    fn test_toggle_trigger_with_empty_model() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.toggle_trigger(10);
        assert_eq!(trigger_model.num_triggers(), 1);
    }

    #[test]
    fn test_toggle_trigger_twice_is_noop() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.toggle_trigger(10);
        trigger_model.toggle_trigger(10);
        assert_eq!(trigger_model.num_triggers(), 0);
    }

    #[test]
    fn test_create_and_add_triggers() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_trigger(Trigger::default());
        let triggers = trigger_model.triggers();
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers.get(0).cloned(), Some(Trigger::default()));
        assert_eq!(trigger_model.num_triggers(), 1);
    }

    #[test]
    fn test_bulk_add_triggers() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_triggers(&[Trigger::default(), Trigger::default(), Trigger::default()]);
        let triggers = trigger_model.triggers();
        assert_eq!(trigger_model.num_triggers(), 3);
        assert_eq!(triggers.len(), 3);
        assert_eq!(triggers.get(0).cloned(), Some(Trigger::default()));
        assert_eq!(triggers.get(1).cloned(), Some(Trigger::default()));
        assert_eq!(triggers.get(2).cloned(), Some(Trigger::default()));
    }
}
