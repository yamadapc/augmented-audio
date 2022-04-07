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
use std::collections::HashMap;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use audio_garbage_collector::{Handle, Shared, SharedCell};

use crate::parameters::EntityId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MidiControllerNumber {
    controller_number: u8,
}

impl MidiControllerNumber {
    #[allow(dead_code)]
    pub fn new(number: u8) -> Self {
        MidiControllerNumber {
            controller_number: number,
        }
    }
}

#[cfg(test)]
mod test_midi_spec {
    use super::*;

    #[test]
    fn test_create_midi_spec() {
        let spec = MidiControllerNumber::new(88);
        assert_eq!(spec.controller_number, 88);
    }
}

pub type MidiMapStorePersist = HashMap<MidiControllerNumber, EntityId>;
pub type MidiMapStore = SharedCell<MidiMapStorePersist>;

pub struct MidiMap {
    #[allow(dead_code)]
    store: MidiMapStore,
    handle: Handle,
}

impl Default for MidiMap {
    fn default() -> Self {
        Self::new_with_handle(audio_garbage_collector::handle())
    }
}

impl MidiMap {
    #[allow(dead_code)]
    pub fn new(
        handle: &Handle,
        store: SharedCell<HashMap<MidiControllerNumber, EntityId>>,
    ) -> Self {
        MidiMap {
            handle: handle.clone(),
            store,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_handle(handle: &Handle) -> Self {
        MidiMap {
            handle: handle.clone(),
            store: SharedCell::new(Shared::new(handle, Default::default())),
        }
    }

    #[allow(dead_code)]
    pub fn add(&self, spec: MidiControllerNumber, action: EntityId) {
        let mut current = (*self.store.get()).clone();
        current.insert(spec, action);
        self.store.set(Shared::new(&self.handle, current));
    }

    #[allow(dead_code)]
    pub fn get(&self, spec: &MidiControllerNumber) -> Option<EntityId> {
        self.store.get().deref().get(spec).cloned()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.store.get().deref().is_empty()
    }

    pub fn store(&self) -> &MidiMapStore {
        &self.store
    }
}

#[cfg(test)]
mod test_midi_map {
    use audio_garbage_collector::Shared;

    use crate::parameters::SourceParameter;
    use crate::LooperId;

    use super::*;

    #[test]
    fn test_create_midi_map_with_handle() {
        let midi_map = MidiMap::new_with_handle(audio_garbage_collector::handle());
        assert!(midi_map.is_empty());
    }

    #[test]
    fn test_create_midi_map() {
        let store = SharedCell::new(Shared::new(
            audio_garbage_collector::handle(),
            Default::default(),
        ));
        let midi_map = MidiMap::new(audio_garbage_collector::handle(), store);
        assert!(midi_map.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let store = SharedCell::new(Shared::new(
            audio_garbage_collector::handle(),
            Default::default(),
        ));
        let midi_map = MidiMap::new(audio_garbage_collector::handle(), store);

        assert!(midi_map.is_empty());
        let spec = MidiControllerNumber::new(88);
        midi_map.add(
            spec,
            EntityId::EntityIdLooperParameter(LooperId(0), SourceParameter::Start.into()),
        );
        assert!(!midi_map.is_empty());
        assert!(midi_map.get(&spec).is_some());
        assert_eq!(
            midi_map.get(&spec).unwrap(),
            EntityId::EntityIdLooperParameter(LooperId(0), SourceParameter::Start.into())
        );
    }
}
