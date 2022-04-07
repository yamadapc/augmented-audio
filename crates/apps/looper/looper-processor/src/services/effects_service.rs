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
use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_time::{FreeverbProcessor, MonoDelayProcessor};
use audio_processor_traits::parameters::{
    AudioProcessorHandleProvider, AudioProcessorHandleRef, ParameterSpec,
};
use augmented_dsp_filters::rbj::FilterProcessor;

use crate::audio::multi_track_looper::effects_processor::EffectType;

fn build_parameters_model(ty: EffectType, handle: AudioProcessorHandleRef) -> EffectDefinition {
    let name = handle.name();
    let num_parameters = handle.parameter_count();

    let mut parameters = Vec::with_capacity(num_parameters);
    for id in 0..num_parameters {
        let spec = handle.get_parameter_spec(id);
        let model = EffectParameterModel { id, spec };
        parameters.push(model);
    }

    EffectDefinition {
        name,
        parameters,
        ty,
    }
}

#[derive(Debug, Clone)]
pub struct EffectParameterModel {
    pub id: usize,
    pub spec: ParameterSpec,
}

#[derive(Debug, Clone)]
pub struct EffectDefinition {
    pub name: String,
    pub parameters: Vec<EffectParameterModel>,
    pub ty: EffectType,
}

#[derive(Default)]
pub struct EffectsService {}

impl EffectsService {
    pub fn get_effects() -> Vec<EffectDefinition> {
        [
            (
                EffectType::EffectTypeReverb,
                FreeverbProcessor::default().generic_handle(),
            ),
            (
                EffectType::EffectTypeDelay,
                MonoDelayProcessor::<f32>::default().generic_handle(),
            ),
            (
                EffectType::EffectTypeBitCrusher,
                BitCrusherProcessor::default().generic_handle(),
            ),
            (
                EffectType::EffectTypeFilter,
                FilterProcessor::default().generic_handle(),
            ),
        ]
        .map(|(ty, handle)| build_parameters_model(ty, handle))
        .to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_effects() {
        let effects = EffectsService::get_effects();
        assert_eq!(effects[0].name, "Reverb");
        assert_eq!(effects[1].name, "Delay");
        assert_eq!(effects[2].name, "Bit-crusher");
        assert_eq!(effects[3].name, "Filter");
    }

    #[test]
    fn test_get_effect_definition() {
        let effects = EffectsService::get_effects();
        let reverb = effects.get(0).unwrap();
        assert_eq!(reverb.parameters.len(), 4);
        assert_eq!(reverb.parameters[0].spec.name(), "Dry");
        assert_eq!(reverb.parameters[1].spec.name(), "Room size");
        assert_eq!(reverb.parameters[2].spec.name(), "Damp");
        assert_eq!(reverb.parameters[3].spec.name(), "Wet");
    }
}
