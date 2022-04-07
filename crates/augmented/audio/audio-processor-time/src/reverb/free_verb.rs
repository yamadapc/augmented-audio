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
use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandle, AudioProcessorHandleProvider, AudioProcessorHandleRef,
    FloatType, ParameterSpec, ParameterType, ParameterValue,
};
use audio_processor_traits::{AtomicF32, AudioProcessorSettings, SimpleAudioProcessor};

use crate::reverb::all_pass::AllPass;
use crate::reverb::lowpass_feedback_comb_filter::LowpassFeedbackCombFilter;
use crate::reverb::tuning::*;

struct FreeverbProcessorHandle {
    width: AtomicF32,
    gain: AtomicF32,
    dry: AtomicF32,
    wet: AtomicF32,
    damp: AtomicF32,
    roomsize: AtomicF32,
}

impl FreeverbProcessorHandle {
    pub fn set_dry(&self, value: f32) {
        self.dry.set(value * SCALEDRY);
    }

    pub fn set_roomsize(&self, value: f32) {
        self.roomsize.set(value * SCALEROOM + OFFSETROOM);
    }

    pub fn set_damp(&self, value: f32) {
        self.damp.set(value * SCALEDAMP);
    }

    pub fn set_wet(&self, value: f32) {
        self.wet.set(value * SCALEWET);
    }
}

struct GenericHandle(Shared<FreeverbProcessorHandle>);

impl AudioProcessorHandle for GenericHandle {
    fn parameter_count(&self) -> usize {
        4
    }

    fn get_parameter_spec(&self, index: usize) -> ParameterSpec {
        let specs: [ParameterSpec; 4] = [
            ParameterSpec::new(
                "Dry".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
            ParameterSpec::new(
                "Room size".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
            ParameterSpec::new(
                "Damp".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
            ParameterSpec::new(
                "Wet".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
        ];
        specs[index].clone()
    }

    fn get_parameter(&self, index: usize) -> Option<ParameterValue> {
        match index {
            0 => Some(self.0.dry.get().into()),
            1 => Some(self.0.roomsize.get().into()),
            2 => Some(self.0.damp.get().into()),
            3 => Some(self.0.wet.get().into()),
            _ => None,
        }
    }

    fn set_parameter(&self, index: usize, request: ParameterValue) {
        if let Ok(value) = request.try_into() {
            match index {
                0 => self.0.set_dry(value),
                1 => self.0.set_roomsize(value),
                2 => self.0.set_damp(value),
                3 => self.0.set_wet(value),
                _ => {}
            }
        }
    }
}

pub struct FreeverbProcessor {
    comb_filters_left: [LowpassFeedbackCombFilter; 8],
    all_pass_left: [AllPass; 4],
    comb_filters_right: [LowpassFeedbackCombFilter; 8],
    all_pass_right: [AllPass; 4],

    handle: Shared<FreeverbProcessorHandle>,

    wet1: f32,
    wet2: f32,
    damp1: f32,
    roomsize1: f32,
}

impl AudioProcessorHandleProvider for FreeverbProcessor {
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(GenericHandle(self.handle.clone()))
    }
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

            handle: make_shared(FreeverbProcessorHandle {
                width: INITIALWIDTH.into(),
                gain: FIXED_GAIN.into(),
                dry: INITIALDRY.into(),
                wet: INITIALWET.into(),
                damp: INITIALDAMP.into(),
                roomsize: 0.0.into(),
            }),

            wet1: 0.0,
            wet2: 0.0,
            damp1: 0.0,
            roomsize1: 0.0,
        }
    }
}

impl FreeverbProcessor {
    fn update(&mut self) {
        self.wet1 = self.handle.wet.get() * (self.handle.width.get() / 2.0 + 0.5);
        self.wet2 = self.handle.wet.get() * ((1.0 - self.handle.width.get()) / 2.0);

        self.roomsize1 = self.handle.roomsize.get();
        self.damp1 = self.handle.damp.get();

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

        self.handle.set_wet(INITIALWET);
        self.handle.set_damp(INITIALDAMP);
        self.handle.set_roomsize(INITIALROOM);
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        // TODO - no need to update on every frame
        self.update();

        let input_left = frame[0];
        let input_right = frame[1];

        let input = (input_left + input_right) * self.handle.gain.get();

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

        let dry = self.handle.dry.get();
        frame[0] = output_left * self.wet1 + output_right * self.wet2 + input_left * dry;
        frame[1] = output_right * self.wet1 + output_left * self.wet2 + input_right * dry;
    }
}
