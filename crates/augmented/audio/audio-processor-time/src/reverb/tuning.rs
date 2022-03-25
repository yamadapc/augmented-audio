pub const STEREO_SPREAD: usize = 23;

// pub const MUTED: f32 = 0.0;
pub const FIXED_GAIN: f32 = 0.015;
pub const SCALEWET: f32 = 3.0;
pub const SCALEDRY: f32 = 2.0;
pub const SCALEDAMP: f32 = 0.4;
pub const SCALEROOM: f32 = 0.28;
pub const OFFSETROOM: f32 = 0.7;
pub const INITIALROOM: f32 = 0.5;
pub const INITIALDAMP: f32 = 0.5;
pub const INITIALWET: f32 = 1.0 / SCALEWET;
pub const INITIALDRY: f32 = 0.0;
pub const INITIALWIDTH: f32 = 1.0;

// pub const INITIALMODE: f32 = 0.0;
// pub const FREEZEMODE: f32 = 0.5;

pub const ALLPASSTUNING_L1: usize = 556;
pub const ALLPASSTUNING_R1: usize = 556 + STEREO_SPREAD;
pub const ALLPASSTUNING_L2: usize = 441;
pub const ALLPASSTUNING_R2: usize = 441 + STEREO_SPREAD;
pub const ALLPASSTUNING_L3: usize = 341;
pub const ALLPASSTUNING_R3: usize = 341 + STEREO_SPREAD;
pub const ALLPASSTUNING_L4: usize = 225;
pub const ALLPASSTUNING_R4: usize = 225 + STEREO_SPREAD;

pub const COMBTUNING_L1: usize = 1116;
pub const COMBTUNING_R1: usize = 1116 + STEREO_SPREAD;
pub const COMBTUNING_L2: usize = 1188;
pub const COMBTUNING_R2: usize = 1188 + STEREO_SPREAD;
pub const COMBTUNING_L3: usize = 1277;
pub const COMBTUNING_R3: usize = 1277 + STEREO_SPREAD;
pub const COMBTUNING_L4: usize = 1356;
pub const COMBTUNING_R4: usize = 1356 + STEREO_SPREAD;
pub const COMBTUNING_L5: usize = 1422;
pub const COMBTUNING_R5: usize = 1422 + STEREO_SPREAD;
pub const COMBTUNING_L6: usize = 1491;
pub const COMBTUNING_R6: usize = 1491 + STEREO_SPREAD;
pub const COMBTUNING_L7: usize = 1557;
pub const COMBTUNING_R7: usize = 1557 + STEREO_SPREAD;
pub const COMBTUNING_L8: usize = 1617;
pub const COMBTUNING_R8: usize = 1617 + STEREO_SPREAD;