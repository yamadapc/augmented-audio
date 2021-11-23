pub struct Project {
    pub id: String,
    pub title: String,
}

pub struct TracksList {
    pub tracks: Vec<Track>,
}

pub struct Track {
    pub id: String,
    pub title: String,
    pub audio_input_id: String,
    pub audio_effects: Vec<AudioEffectInstance>,
    pub pan: f32,
}

pub struct AudioEffectInstance {
    pub id: String,
}
