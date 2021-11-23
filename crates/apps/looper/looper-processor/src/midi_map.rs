use std::collections::HashMap;
use std::ops::Deref;

use audio_garbage_collector::{Handle, Shared, SharedCell};

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum Action {
    #[allow(dead_code)]
    SetRecording(bool),
    #[allow(dead_code)]
    SetPlayback(bool),
    #[allow(dead_code)]
    Clear,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MidiSpec {
    status: u8,
    number: u8,
}

impl MidiSpec {
    pub fn new(status: u8, number: u8) -> Self {
        MidiSpec { status, number }
    }
}

#[cfg(test)]
mod test_midi_spec {
    use super::*;

    #[test]
    fn test_create_midi_spec() {
        let spec = MidiSpec::new(0xB0, 88);
        assert_eq!(spec.status, 0xB0);
        assert_eq!(spec.number, 88);
    }
}

pub struct MidiMap {
    store: SharedCell<HashMap<MidiSpec, Action>>,
    #[allow(dead_code)]
    handle: Handle,
}

impl MidiMap {
    #[allow(dead_code)]
    pub fn new(handle: &Handle, store: SharedCell<HashMap<MidiSpec, Action>>) -> Self {
        MidiMap {
            handle: handle.clone(),
            store,
        }
    }

    pub fn new_with_handle(handle: &Handle) -> Self {
        MidiMap {
            handle: handle.clone(),
            store: SharedCell::new(Shared::new(handle, Default::default())),
        }
    }

    #[allow(dead_code)]
    pub fn add(&self, spec: MidiSpec, action: Action) {
        let mut current = (*self.store.get()).clone();
        current.insert(spec, action);
        self.store.set(Shared::new(&self.handle, current));
    }

    pub fn get(&self, spec: &MidiSpec) -> Option<Action> {
        self.store.get().deref().get(spec).cloned()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.store.get().deref().is_empty()
    }
}

#[cfg(test)]
mod test_midi_map {
    use audio_garbage_collector::Shared;

    use super::*;

    #[test]
    fn test_create_midi_map() {
        let gc = audio_garbage_collector::GarbageCollector::default();
        let store = SharedCell::new(Shared::new(gc.handle(), Default::default()));
        let midi_map = MidiMap::new(gc.handle(), store);
        assert!(midi_map.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let gc = audio_garbage_collector::GarbageCollector::default();
        let store = SharedCell::new(Shared::new(gc.handle(), Default::default()));
        let midi_map = MidiMap::new(gc.handle(), store);

        assert!(midi_map.is_empty());
        let spec = MidiSpec::new(0xB0, 88);
        midi_map.add(spec, Action::Clear);
        assert!(!midi_map.is_empty());
        assert!(midi_map.get(&spec).is_some());
        assert_eq!(midi_map.get(&spec).unwrap(), Action::Clear);
    }
}
