use crate::LooperId;
use augmented_atomics::AtomicF32;

use crate::multi_track_looper::parameters_map::ParametersMap;
use crate::parameters::{ParameterId, ParameterValue};

pub struct SceneHandle {
    scene_value: AtomicF32,
    scenes: Vec<SceneState>,
}

struct SceneState {
    scene_parameters: Vec<ParametersMap>,
}

impl SceneState {
    fn new(num_loopers: usize) -> Self {
        Self {
            scene_parameters: (0..num_loopers).map(|_| ParametersMap::new()).collect(),
        }
    }
}

impl SceneHandle {
    pub fn new(num_loopers: usize, num_scenes: usize) -> Self {
        Self {
            scene_value: 0.0.into(),
            scenes: (0..num_scenes)
                .map(|_| SceneState::new(num_loopers))
                .collect(),
        }
    }

    pub fn get_slider(&self) -> f32 {
        self.scene_value.get()
    }

    pub fn set_slider(&self, value: f32) {
        self.scene_value.set(value);
    }

    pub fn set(
        &self,
        scene_id: usize,
        looper_id: LooperId,
        id: impl Into<ParameterId>,
        value: impl Into<ParameterValue>,
    ) {
        self.scenes[scene_id].scene_parameters[looper_id.0].set(id, value)
    }

    pub fn get_left(
        &self,
        looper_id: LooperId,
        id: impl Into<ParameterId>,
    ) -> Option<&ParameterValue> {
        let id = id.into();
        let map = &self.scenes[0].scene_parameters[looper_id.0];
        if map.has_value(id.clone()) {
            Some(&map.get(id))
        } else {
            None
        }
    }

    pub fn get_right(
        &self,
        looper_id: LooperId,
        id: impl Into<ParameterId>,
    ) -> Option<&ParameterValue> {
        let id = id.into();
        let map = &self.scenes[1].scene_parameters[looper_id.0];
        if map.has_value(id.clone()) {
            Some(&map.get(id))
        } else {
            None
        }
    }
}