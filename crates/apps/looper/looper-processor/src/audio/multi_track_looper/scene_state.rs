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
use serde::{Deserialize, Serialize};

use augmented_atomics::AtomicF32;

use crate::audio::multi_track_looper::parameters::{ParameterId, ParameterValue};
use crate::audio::multi_track_looper::parameters_map::ParametersMap;
use crate::LooperId;

/// All scenes state. Contains the slider position & a list of scene states.
///
/// Each scene state contains a parameter map for each voice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneHandle {
    scene_value: AtomicF32,
    scenes: Vec<SceneState>,
}

/// State for a single scene, this is a set of parameter maps for each voice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneState {
    scene_parameters: Vec<ParametersMap>,
}

impl SceneState {
    fn new(num_loopers: usize) -> Self {
        Self {
            scene_parameters: (0..num_loopers).map(|_| ParametersMap::new()).collect(),
        }
    }

    pub fn scene_parameters(&self) -> &Vec<ParametersMap> {
        &self.scene_parameters
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

    #[inline]
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

    pub fn unset(&self, scene_id: usize, looper_id: LooperId, id: impl Into<ParameterId>) {
        self.scenes[scene_id].scene_parameters[looper_id.0].unset(id)
    }

    #[inline]
    pub fn get_left(
        &self,
        looper_id: LooperId,
        id: impl Into<ParameterId>,
    ) -> Option<&ParameterValue> {
        let id = id.into();
        let map = &self.scenes[0].scene_parameters[looper_id.0];
        map.get_option(id)
    }

    #[inline]
    pub fn get_right(
        &self,
        looper_id: LooperId,
        id: impl Into<ParameterId>,
    ) -> Option<&ParameterValue> {
        let id = id.into();
        let map = &self.scenes[1].scene_parameters[looper_id.0];
        map.get_option(id)
    }

    pub fn scenes(&self) -> &Vec<SceneState> {
        &self.scenes
    }
}
