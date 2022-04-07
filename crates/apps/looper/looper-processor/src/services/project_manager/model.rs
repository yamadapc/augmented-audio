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
use std::ops::Deref;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::audio::midi_map::MidiMapStorePersist;
use crate::audio::multi_track_looper::lfo_processor::LFOHandleMap;
use crate::audio::multi_track_looper::looper_voice::{LooperVoice, ParameterValues};
use crate::audio::multi_track_looper::scene_state::SceneHandle;
use crate::audio::multi_track_looper::trigger_model::{TrackTriggerModel, Trigger};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackTriggerModelPersist {
    pub pattern_length: usize,
    pub pattern_step_beats: f64,
    pub triggers: Vec<Trigger>,
}

impl From<&TrackTriggerModel> for TrackTriggerModelPersist {
    fn from(model: &TrackTriggerModel) -> Self {
        Self {
            pattern_length: model.pattern_length(),
            pattern_step_beats: model.pattern_step_beats(),
            triggers: model.triggers().deref().to_vec(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LooperVoicePersist {
    pub id: usize,
    pub parameter_values: ParameterValues,
    pub triggers: TrackTriggerModelPersist,
    pub lfo1: LFOHandleMap,
    pub lfo2: LFOHandleMap,
}

impl From<&LooperVoice> for LooperVoicePersist {
    fn from(voice: &LooperVoice) -> Self {
        Self {
            id: voice.id,
            parameter_values: voice.user_parameters().clone(),
            triggers: TrackTriggerModelPersist::from(voice.trigger_model().deref()),
            lfo1: voice.lfo1().map().clone(),
            lfo2: voice.lfo2().map().clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub voices: Vec<LooperVoicePersist>,
    pub looper_clips: Vec<Option<PathBuf>>,
    pub midi_map: MidiMapStorePersist,
    pub scene_state: SceneHandle,
}
