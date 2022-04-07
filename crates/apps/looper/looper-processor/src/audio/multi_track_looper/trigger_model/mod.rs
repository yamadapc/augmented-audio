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
//! This module contains [`TrackTriggerModel`] and related functions.
//!
//! [`TrackTriggerModel`] is an object containing step-sequencer state. In particular it holds:
//!
//! * A list of [`Trigger`] objects, which should have some position within the sequencer
//!   ([`TriggerPosition`]) and optionally have any number of [`TriggerLock`]s associated to
//!   different [`ParameterId`]s
//! * The sequencer options, such as number of steps (length) and step-size in beats
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;

use basedrop::{Shared, SharedCell};
use serde::{Deserialize, Serialize};

use audio_garbage_collector::{make_shared, make_shared_cell};
use augmented_atomics::{AtomicF32, AtomicValue};
use step_tracker::StepTracker;

use crate::audio::multi_track_looper::parameters::ParameterId;

pub mod step_tracker;

/// Return an iterator for all the triggers that are currently playing
pub fn find_running_beat_trigger<'a>(
    track_trigger_model: &TrackTriggerModel,
    triggers: &'a Shared<Vec<Trigger>>,
    position_beats: f64,
) -> impl Iterator<Item = &'a Trigger> + 'a {
    let position_beats = position_beats
        % (track_trigger_model.pattern_length as f64 * track_trigger_model.pattern_step_beats);
    let step_length_beats = track_trigger_model.pattern_step_beats;
    let current_step = (position_beats / step_length_beats) as usize;

    triggers
        .deref()
        .iter()
        .filter(move |trigger| trigger.position.step.get() == current_step)
}

/// If a trigger has just been fired, return a reference to it.
/// This mutates the `StepTracker`.
pub fn find_current_beat_trigger<'a>(
    track_trigger_model: &'a TrackTriggerModel,
    triggers: &'a Shared<Vec<Trigger>>,
    step_tracker: &mut StepTracker,
    position_beats: f64,
) -> Option<&'a Trigger> {
    step_tracker
        .accept(
            track_trigger_model.pattern_step_beats,
            position_beats
                % (track_trigger_model.pattern_length as f64
                    * track_trigger_model.pattern_step_beats),
        )
        .and_then(|step| track_trigger_model.find_step(triggers, step))
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TriggerLock {
    value: f32,
}

impl TriggerLock {
    pub fn value(&self) -> f32 {
        self.value
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LoopTrigger {
    locks: HashMap<ParameterId, TriggerLock>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TriggerInner {
    LoopTrigger(LoopTrigger),
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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

    pub fn step(&self) -> usize {
        self.position.step.get()
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

    pub fn remove_lock(&mut self, parameter_id: ParameterId) {
        let TriggerInner::LoopTrigger(loop_trigger) = &mut self.inner;
        loop_trigger.locks.remove(&parameter_id);
    }

    pub fn locks(&self) -> impl Iterator<Item = (&ParameterId, &TriggerLock)> {
        let TriggerInner::LoopTrigger(loop_trigger) = &self.inner;
        loop_trigger.locks.iter()
    }
}

pub struct TrackTriggerModel {
    pattern_length: usize,
    pattern_step_beats: f64,
    triggers: SharedCell<Vec<Trigger>>,
}

impl Default for TrackTriggerModel {
    fn default() -> Self {
        Self {
            pattern_length: 16,
            pattern_step_beats: 0.25,
            triggers: make_shared_cell(Vec::default()),
        }
    }
}

impl TrackTriggerModel {
    pub fn pattern_length(&self) -> usize {
        self.pattern_length
    }

    pub fn pattern_step_beats(&self) -> f64 {
        self.pattern_step_beats
    }

    pub fn num_triggers(&self) -> usize {
        self.triggers.get().len()
    }

    pub fn find_step<'a>(
        &self,
        triggers: &'a Shared<Vec<Trigger>>,
        step: usize,
    ) -> Option<&'a Trigger> {
        triggers
            .iter()
            .find(|trigger| trigger.position.step.get() == step)
    }

    pub fn add_lock(&self, position_beats: usize, parameter_id: ParameterId, value: f32) {
        let triggers = self.triggers.get();
        let mut triggers: Vec<Trigger> = (*triggers).clone();
        if let Some(trigger) = triggers
            .iter_mut()
            .find(|trigger| trigger.position.step.get() == position_beats)
        {
            trigger.add_lock(parameter_id, value);
        }
        self.triggers.set(make_shared(triggers));
    }

    pub fn remove_lock(&self, position_beats: usize, parameter_id: ParameterId) {
        let triggers = self.triggers.get();
        let mut triggers: Vec<Trigger> = (*triggers).clone();
        if let Some(trigger) = triggers
            .iter_mut()
            .find(|trigger| trigger.position.step.get() == position_beats)
        {
            trigger.remove_lock(parameter_id);
        }
        self.triggers.set(make_shared(triggers));
    }

    pub fn toggle_trigger(&self, position_step: usize) {
        let triggers = self.triggers.get();
        let mut triggers: Vec<Trigger> = (*triggers).clone();

        let indexes: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(_idx, trigger)| trigger.position.step.get() == position_step)
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
                step: position_step.into(),
                position: (position_step as f32).into(),
            });
            self.add_trigger(trigger);
        }
    }

    pub fn remove_trigger(&self, position_step: usize) {
        let triggers = self.triggers.get();
        let mut triggers: Vec<Trigger> = (*triggers).clone();
        let _indexes: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(_idx, trigger)| trigger.position.step.get() == position_step)
            .map(|(idx, _)| idx)
            .collect();
        let indexes: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(_idx, trigger)| trigger.position.step.get() == position_step)
            .map(|(idx, _)| idx)
            .collect();
        if !indexes.is_empty() {
            for index in indexes {
                triggers.remove(index);
            }
            self.triggers.set(make_shared(triggers));
        }
    }

    pub fn add_trigger(&self, trigger: Trigger) {
        let triggers = self.triggers.get();
        let mut triggers: Vec<Trigger> = (*triggers).clone();
        triggers.push(trigger);
        log::info!("Track triggers={:?}", triggers);
        self.triggers.set(make_shared(triggers));
    }

    pub fn add_triggers(&self, triggers: &[Trigger]) {
        let triggers: Vec<Trigger> = Vec::from(triggers);
        self.triggers.set(make_shared(triggers));
    }

    pub fn clear(&self) {
        self.triggers.set(make_shared(vec![]));
    }

    pub fn triggers(&self) -> Shared<Vec<Trigger>> {
        self.triggers.get()
    }
}

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;

    use super::*;

    #[test]
    fn test_triggers_iterator_does_not_allocate() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.toggle_trigger(10);

        assert_no_alloc(|| {
            for trigger in trigger_model.triggers().iter() {
                for (_id, lock) in trigger.locks() {
                    assert!(lock.value > f32::NEG_INFINITY);
                }
            }
        });
    }

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
    fn test_remove_trigger() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_trigger(Trigger::default());
        trigger_model.remove_trigger(0);
        let triggers = trigger_model.triggers();
        assert_eq!(triggers.len(), 0);
    }

    #[test]
    fn test_clear_triggers() {
        let trigger_model = TrackTriggerModel::default();
        trigger_model.add_trigger(Trigger::default());
        trigger_model.clear();
        let triggers = trigger_model.triggers();
        assert_eq!(triggers.len(), 0);
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
