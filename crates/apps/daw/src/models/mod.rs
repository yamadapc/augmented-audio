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
