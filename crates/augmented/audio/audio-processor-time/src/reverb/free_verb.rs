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
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioContext, AudioProcessor};

use crate::reverb::all_pass::AllPass;
use crate::reverb::lowpass_feedback_comb_filter::LowpassFeedbackCombFilter;
use crate::reverb::tuning::*;

struct FreeverbProcessorHandle {
    width: AtomicF32,
    gain: AtomicF32,
    dry: AtomicF32,
    wet: AtomicF32,
    damp: AtomicF32,
    room_size: AtomicF32,
}

impl FreeverbProcessorHandle {
    pub fn set_dry(&self, value: f32) {
        self.dry.set(value * SCALE_DRY);
    }

    pub fn set_room_size(&self, value: f32) {
        self.room_size.set(value * SCALE_ROOM + OFFSET_ROOM);
    }

    pub fn set_damp(&self, value: f32) {
        self.damp.set(value * SCALE_DAMP);
    }

    pub fn set_wet(&self, value: f32) {
        self.wet.set(value * SCALE_WET);
    }
}

struct GenericHandle(Shared<FreeverbProcessorHandle>);

impl AudioProcessorHandle for GenericHandle {
    fn name(&self) -> String {
        "Reverb".to_string()
    }

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
            1 => Some(self.0.room_size.get().into()),
            2 => Some(self.0.damp.get().into()),
            3 => Some(self.0.wet.get().into()),
            _ => None,
        }
    }

    fn set_parameter(&self, index: usize, request: ParameterValue) {
        if let Ok(value) = request.try_into() {
            match index {
                0 => self.0.set_dry(value),
                1 => self.0.set_room_size(value),
                2 => self.0.set_damp(value),
                3 => self.0.set_wet(value),
                _ => {}
            }
        }
    }
}

pub struct MonoFreeverbProcessor {
    comb_filters: [LowpassFeedbackCombFilter; 8],
    all_pass: [AllPass; 4],
}

impl MonoFreeverbProcessor {
    fn new(comb_filters: [LowpassFeedbackCombFilter; 8], all_pass: [AllPass; 4]) -> Self {
        Self {
            comb_filters,
            all_pass,
        }
    }

    fn prepare(&mut self) {
        for all_pass in &mut self.all_pass {
            all_pass.set_feedback(0.5)
        }
    }

    fn update(&mut self, room_size: f32, damp: f32) {
        for comb in self.comb_filters.iter_mut() {
            comb.set_feedback(room_size);
            comb.set_damp(damp);
        }
    }

    fn process(&mut self, channel: &mut [f32], wet: f32, gain: f32) {
        for input in channel {
            let mut output = 0.0;
            for comb in self.comb_filters.iter_mut() {
                output += comb.process(*input * gain);
            }

            for allpass in self.all_pass.iter_mut() {
                output = allpass.process(output);
            }

            *input = output * wet;
        }
    }
}

pub struct FreeverbProcessor {
    processors: [MonoFreeverbProcessor; 2],
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
            processors: [
                MonoFreeverbProcessor::new(
                    [
                        LowpassFeedbackCombFilter::new(COMBTUNING_L1),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L2),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L3),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L4),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L5),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L6),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L7),
                        LowpassFeedbackCombFilter::new(COMBTUNING_L8),
                    ],
                    [
                        AllPass::new(ALL_PASS_TUNING),
                        AllPass::new(ALLPASSTUNING_L2),
                        AllPass::new(ALLPASSTUNING_L3),
                        AllPass::new(ALLPASSTUNING_L4),
                    ],
                ),
                MonoFreeverbProcessor::new(
                    [
                        LowpassFeedbackCombFilter::new(COMBTUNING_R1),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R2),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R3),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R4),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R5),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R6),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R7),
                        LowpassFeedbackCombFilter::new(COMBTUNING_R8),
                    ],
                    [
                        AllPass::new(ALLPASSTUNING_R1),
                        AllPass::new(ALLPASSTUNING_R2),
                        AllPass::new(ALLPASSTUNING_R3),
                        AllPass::new(ALLPASSTUNING_R4),
                    ],
                ),
            ],
            handle: make_shared(FreeverbProcessorHandle {
                width: INITIAL_WIDTH.into(),
                gain: FIXED_GAIN.into(),
                dry: INITIAL_DRY.into(),
                wet: INITIAL_WET.into(),
                damp: INITIAL_DAMP.into(),
                room_size: 0.0.into(),
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

        self.roomsize1 = self.handle.room_size.get();
        self.damp1 = self.handle.damp.get();

        for processor in self.processors.iter_mut() {
            processor.update(self.roomsize1, self.damp1);
        }
    }
}

impl AudioProcessor for FreeverbProcessor {
    type SampleType = f32;

    fn prepare(&mut self, _context: &mut AudioContext) {
        for processor in self.processors.iter_mut() {
            processor.prepare();
        }

        self.handle.set_wet(INITIAL_WET);
        self.handle.set_damp(INITIAL_DAMP);
        self.handle.set_room_size(INITIAL_ROOM);
    }

    // TODO: I broke dry signal and so on
    fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<Self::SampleType>) {
        // TODO - no need to update on every frame
        self.update();

        for (channel_num, (channel, processor)) in buffer
            .channels_mut()
            .iter_mut()
            .zip(&mut self.processors)
            .enumerate()
        {
            let wet = if channel_num == 1 {
                self.wet1
            } else {
                self.wet2
            };
            processor.process(channel, wet, self.handle.gain.get());
        }

        // let input_left = frame[0];
        // let input_right = frame[1];
        //
        // let input = (input_left + input_right) * self.handle.gain.get();
        //
        // let mut output_left = 0.0;
        // let mut output_right = 0.0;
        // for (comb_left, comb_right) in self
        //     .comb_filters_left
        //     .iter_mut()
        //     .zip(self.comb_filters_right.iter_mut())
        // {
        //     output_left += comb_left.process(input);
        //     output_right += comb_right.process(input);
        // }
        //
        // for (allpass_left, allpass_right) in self
        //     .all_pass_left
        //     .iter_mut()
        //     .zip(self.all_pass_right.iter_mut())
        // {
        //     output_left = allpass_left.process(output_left);
        //     output_right = allpass_right.process(output_right);
        // }
    }
}
