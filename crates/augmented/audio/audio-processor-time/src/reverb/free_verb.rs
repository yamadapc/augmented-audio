use audio_processor_traits::{AudioProcessorSettings, SimpleAudioProcessor};

use crate::reverb::all_pass::AllPass;
use crate::reverb::lowpass_feedback_comb_filter::LowpassFeedbackCombFilter;
use crate::reverb::tuning::*;

pub struct FreeverbProcessor {
    comb_filters_left: [LowpassFeedbackCombFilter; 8],
    all_pass_left: [AllPass; 4],
    comb_filters_right: [LowpassFeedbackCombFilter; 8],
    all_pass_right: [AllPass; 4],

    width: f32,
    gain: f32,
    dry: f32,
    wet: f32,
    wet1: f32,
    wet2: f32,
    damp: f32,
    damp1: f32,
    roomsize: f32,
    roomsize1: f32,
}

impl Default for FreeverbProcessor {
    fn default() -> Self {
        Self {
            comb_filters_left: [
                LowpassFeedbackCombFilter::new(COMBTUNING_L1),
                LowpassFeedbackCombFilter::new(COMBTUNING_L2),
                LowpassFeedbackCombFilter::new(COMBTUNING_L3),
                LowpassFeedbackCombFilter::new(COMBTUNING_L4),
                LowpassFeedbackCombFilter::new(COMBTUNING_L5),
                LowpassFeedbackCombFilter::new(COMBTUNING_L6),
                LowpassFeedbackCombFilter::new(COMBTUNING_L7),
                LowpassFeedbackCombFilter::new(COMBTUNING_L8),
            ],
            all_pass_left: [
                AllPass::new(ALLPASSTUNING_L1),
                AllPass::new(ALLPASSTUNING_L2),
                AllPass::new(ALLPASSTUNING_L3),
                AllPass::new(ALLPASSTUNING_L4),
            ],
            comb_filters_right: [
                LowpassFeedbackCombFilter::new(COMBTUNING_R1),
                LowpassFeedbackCombFilter::new(COMBTUNING_R2),
                LowpassFeedbackCombFilter::new(COMBTUNING_R3),
                LowpassFeedbackCombFilter::new(COMBTUNING_R4),
                LowpassFeedbackCombFilter::new(COMBTUNING_R5),
                LowpassFeedbackCombFilter::new(COMBTUNING_R6),
                LowpassFeedbackCombFilter::new(COMBTUNING_R7),
                LowpassFeedbackCombFilter::new(COMBTUNING_R8),
            ],
            all_pass_right: [
                AllPass::new(ALLPASSTUNING_R1),
                AllPass::new(ALLPASSTUNING_R2),
                AllPass::new(ALLPASSTUNING_R3),
                AllPass::new(ALLPASSTUNING_R4),
            ],

            width: INITIALWIDTH,
            gain: FIXED_GAIN,
            dry: INITIALDRY,
            wet: INITIALWET,
            wet1: 0.0,
            wet2: 0.0,
            damp: INITIALDAMP,
            damp1: 0.0,
            roomsize: 0.0,
            roomsize1: 0.0,
        }
    }
}

impl FreeverbProcessor {
    fn update(&mut self) {
        self.wet1 = self.wet * (self.width / 2.0 + 0.5);
        self.wet2 = self.wet * ((1.0 - self.width) / 2.0);

        self.roomsize1 = self.roomsize;
        self.damp1 = self.damp;

        for (comb_left, comb_right) in self
            .comb_filters_left
            .iter_mut()
            .zip(self.comb_filters_right.iter_mut())
        {
            comb_left.set_feedback(self.roomsize1);
            comb_right.set_feedback(self.roomsize1);
            comb_left.set_damp(self.damp1);
            comb_right.set_damp(self.damp1);
        }
    }

    pub fn set_dry(&mut self, value: f32) {
        self.dry = value * SCALEDRY;
    }

    pub fn set_roomsize(&mut self, value: f32) {
        self.roomsize = value * SCALEROOM + OFFSETROOM;
        self.update();
    }

    pub fn set_damp(&mut self, value: f32) {
        self.damp = value * SCALEDAMP;
        self.update();
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALEWET;
        self.update();
    }
}

impl SimpleAudioProcessor for FreeverbProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, _settings: AudioProcessorSettings) {
        for (allpass_left, allpass_right) in self
            .all_pass_left
            .iter_mut()
            .zip(self.all_pass_right.iter_mut())
        {
            allpass_left.set_feedback(0.5);
            allpass_right.set_feedback(0.5);
        }

        self.set_wet(INITIALWET);
        self.set_damp(INITIALDAMP);
        self.set_roomsize(INITIALROOM);
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        let input_left = frame[0];
        let input_right = frame[1];

        let input = (input_left + input_right) * self.gain;

        let mut output_left = 0.0;
        let mut output_right = 0.0;
        for (comb_left, comb_right) in self
            .comb_filters_left
            .iter_mut()
            .zip(self.comb_filters_right.iter_mut())
        {
            output_left += comb_left.process(input);
            output_right += comb_right.process(input);
        }

        for (allpass_left, allpass_right) in self
            .all_pass_left
            .iter_mut()
            .zip(self.all_pass_right.iter_mut())
        {
            output_left = allpass_left.process(output_left);
            output_right = allpass_right.process(output_right);
        }

        frame[0] = output_left * self.wet1 + output_right * self.wet2 + input_left * self.dry;
        frame[1] = output_right * self.wet1 + output_left * self.wet2 + input_right * self.dry;
    }
}
