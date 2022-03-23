pub mod step_tracker;

use std::ops::Deref;
use std::sync::atomic::AtomicUsize;

use basedrop::{Shared, SharedCell};
use im::{HashMap, Vector};

use crate::multi_track_looper::parameters::ParameterId;
use audio_garbage_collector::{make_shared, make_shared_cell};
use augmented_atomics::{AtomicF32, AtomicValue};
use step_tracker::StepTracker;

pub fn find_running_beat_trigger<'a>(
    track_trigger_model: &TrackTriggerModel,
    triggers: &'a Shared<Vector<Trigger>>,
    position_beats: f64,
) -> impl Iterator<Item = Trigger> + 'a {
    let position_beats = position_beats
        % (track_trigger_model.pattern_length as f64 * track_trigger_model.pattern_step_beats);
    let step_length_beats = track_trigger_model.pattern_step_beats;
    let current_step = (position_beats / step_length_beats) as usize;

    triggers
        .deref()
        .iter()
        .filter(move |trigger| trigger.position.step.get() == current_step)
        .cloned()
}

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
        .and_then(|step| track_trigger_model.find_step(step))
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct TriggerLock {
    value: f32,
}

impl TriggerLock {
    pub fn value(&self) -> f32 {
        self.value
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct LoopTrigger {
    locks: HashMap<ParameterId, TriggerLock>,
}

#[derive(Clone, PartialEq, Debug)]
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

    pub fn add_lock(&mut self, parameter_id: ParameterId, value: f32) {
        let TriggerInner::LoopTrigger(loop_trigger) = &mut self.inner;
        loop_trigger
            .locks
            .insert(parameter_id, TriggerLock { value });
    }

    pub fn locks(&self) -> impl Iterator<Item = (&ParameterId, &TriggerLock)> {
        let TriggerInner::LoopTrigger(loop_trigger) = &self.inner;
        loop_trigger.locks.iter()
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

    pub fn add_lock(&self, position_beats: usize, parameter_id: ParameterId, value: f32) {
        let triggers = self.triggers.get();
        let mut triggers: Vector<Trigger> = (*triggers).clone();
        if let Some(trigger) = triggers
            .iter_mut()
            .find(|trigger| trigger.position.step.get() == position_beats)
        {
            trigger.add_lock(parameter_id, value);
        }
        self.triggers.set(make_shared(triggers));
    }

    pub fn toggle_trigger(&self, position_beats: usize) {
        let triggers = self.triggers.get();
        let mut triggers: Vector<Trigger> = (*triggers).clone();

        let indexes: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(_idx, trigger)| trigger.position.step.get() == position_beats)
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
        let triggers: Vector<Trigger> = Vector::from(triggers);
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
