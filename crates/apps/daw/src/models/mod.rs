// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TracksList {
    pub tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub audio_input_id: String,
    pub audio_effects: Vec<AudioEffectInstance>,
    pub pan: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AudioEffectInstance {
    pub id: String,
}
