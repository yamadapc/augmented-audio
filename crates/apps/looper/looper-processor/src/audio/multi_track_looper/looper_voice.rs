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
use basedrop::Shared;

use audio_processor_pitch_shifter::{
    MultiChannelPitchShifterProcessor, MultiChannelPitchShifterProcessorHandle,
};

use crate::audio::processor::handle::LooperHandle as LooperProcessorHandle;
use crate::{
    LoopShufflerProcessorHandle, LooperOptions, LooperProcessor, QuantizeMode, TimeInfoProviderImpl,
};

use super::effects_processor::{EffectsProcessor, EffectsProcessorHandle};
use super::envelope_processor::{EnvelopeHandle, EnvelopeProcessor};
use super::lfo_processor::LFOHandle;
use super::parameters::ParameterId;
use super::parameters_map::ParametersMap;
use super::trigger_model::TrackTriggerModel;

pub type ParameterValues = ParametersMap;

pub struct LooperVoice {
    pub id: usize,
    parameter_values: ParameterValues,
    parameter_ids: Vec<ParameterId>,
    triggers: Shared<TrackTriggerModel>,
    looper_handle: Shared<LooperProcessorHandle>,
    sequencer_handle: Shared<LoopShufflerProcessorHandle>,
    pitch_shifter_handle: Shared<MultiChannelPitchShifterProcessorHandle>,
    envelope: Shared<EnvelopeHandle>,
    lfo1_handle: Shared<LFOHandle>,
    lfo2_handle: Shared<LFOHandle>,
    effects_handle: Shared<EffectsProcessorHandle>,
}

impl LooperVoice {
    pub fn trigger_model(&self) -> &Shared<TrackTriggerModel> {
        &self.triggers
    }

    pub fn looper(&self) -> &Shared<LooperProcessorHandle> {
        &self.looper_handle
    }

    pub fn envelope(&self) -> &Shared<EnvelopeHandle> {
        &self.envelope
    }

    pub fn lfo1(&self) -> &Shared<LFOHandle> {
        &self.lfo1_handle
    }

    pub fn lfo2(&self) -> &Shared<LFOHandle> {
        &self.lfo2_handle
    }

    pub fn pitch_shifter(&self) -> &Shared<MultiChannelPitchShifterProcessorHandle> {
        &self.pitch_shifter_handle
    }

    pub fn parameter_ids(&self) -> &[ParameterId] {
        &self.parameter_ids
    }

    /// Parameters as configured in the UI
    pub fn user_parameters(&self) -> &ParameterValues {
        &self.parameter_values
    }

    pub fn sequencer(&self) -> &Shared<LoopShufflerProcessorHandle> {
        &self.sequencer_handle
    }

    pub fn effects(&self) -> &Shared<EffectsProcessorHandle> {
        &self.effects_handle
    }
}

pub struct VoiceProcessors {
    pub looper: LooperProcessor,
    pub pitch_shifter: MultiChannelPitchShifterProcessor,
    pub envelope: EnvelopeProcessor,
    pub effects_processor: EffectsProcessor,
}

pub fn build_voice_handle(id: usize, voice_processors: &VoiceProcessors) -> LooperVoice {
    use audio_garbage_collector::make_shared;

    use super::parameters::build_default_parameters;

    let VoiceProcessors {
        looper,
        pitch_shifter,
        envelope,
        effects_processor,
    } = voice_processors;
    let looper_handle = looper.handle().clone();
    let sequencer_handle = looper.sequencer_handle().clone();
    let triggers = make_shared(TrackTriggerModel::default());
    let (_parameter_values, parameter_ids) = build_default_parameters();

    LooperVoice {
        id,
        parameter_ids,
        parameter_values: ParametersMap::new(),
        looper_handle,
        sequencer_handle,
        triggers,
        pitch_shifter_handle: pitch_shifter.handle().clone(),
        lfo1_handle: make_shared(LFOHandle::default()),
        lfo2_handle: make_shared(LFOHandle::default()),
        envelope: envelope.handle.clone(),
        effects_handle: effects_processor.handle().clone(),
    }
}

pub fn from_handle(handle: &LooperVoice) -> VoiceProcessors {
    let effects_processor = EffectsProcessor::from_handle(handle.effects_handle.clone());
    let looper = LooperProcessor::from_handle(
        handle.looper_handle.clone(),
        handle.sequencer_handle.clone(),
    );
    let envelope = EnvelopeProcessor {
        handle: handle.envelope.clone(),
    };
    let pitch_shifter =
        MultiChannelPitchShifterProcessor::from_handle(handle.pitch_shifter_handle.clone());

    VoiceProcessors {
        looper,
        effects_processor,
        envelope,
        pitch_shifter,
    }
}

pub fn build_voice_processor(
    options: &LooperOptions,
    time_info_provider: &Shared<TimeInfoProviderImpl>,
) -> VoiceProcessors {
    let effects_processor = EffectsProcessor::new();
    let looper = LooperProcessor::new(options.clone(), time_info_provider.clone());
    looper
        .handle()
        .quantize_options()
        .set_mode(QuantizeMode::SnapNext);
    looper.handle().set_tick_time(false);

    let pitch_shifter = MultiChannelPitchShifterProcessor::default();
    let envelope = EnvelopeProcessor::default();

    VoiceProcessors {
        looper,
        pitch_shifter,
        envelope,
        effects_processor,
    }
}
