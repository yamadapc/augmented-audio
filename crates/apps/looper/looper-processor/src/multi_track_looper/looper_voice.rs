use basedrop::{Shared, SharedCell};

use audio_processor_pitch_shifter::{
    MultiChannelPitchShifterProcessor, MultiChannelPitchShifterProcessorHandle,
};

use crate::multi_track_looper::envelope_processor::{EnvelopeHandle, EnvelopeProcessor};
use crate::multi_track_looper::lfo_processor::LFOHandle;
use crate::multi_track_looper::parameters::{ParameterId, ParameterValue};
use crate::processor::handle::LooperHandle as LooperProcessorHandle;
use crate::{LoopSequencerProcessorHandle, LooperProcessor};

use super::trigger_model::TrackTriggerModel;

type ParameterValues = SharedCell<im::HashMap<ParameterId, ParameterValue>>;

pub struct LooperVoice {
    pub id: usize,
    parameter_values: ParameterValues,
    triggers: Shared<TrackTriggerModel>,
    looper_handle: Shared<LooperProcessorHandle>,
    sequencer_handle: Shared<LoopSequencerProcessorHandle>,
    pitch_shifter_handle: Shared<MultiChannelPitchShifterProcessorHandle>,
    envelope: Shared<EnvelopeHandle>,
    lfo1_handle: Shared<LFOHandle>,
    lfo2_handle: Shared<LFOHandle>,
}

impl LooperVoice {
    pub fn triggers(&self) -> &Shared<TrackTriggerModel> {
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

    pub fn parameters(&self) -> &ParameterValues {
        &self.parameter_values
    }

    pub fn sequencer(&self) -> &Shared<LoopSequencerProcessorHandle> {
        &self.sequencer_handle
    }
}

pub struct VoiceProcessors {
    pub looper: LooperProcessor,
    pub pitch_shifter: MultiChannelPitchShifterProcessor,
    pub envelope: EnvelopeProcessor,
}

pub fn build_voice_handle(id: usize, voice_processors: &VoiceProcessors) -> LooperVoice {
    use audio_garbage_collector::{make_shared, make_shared_cell};

    use super::parameters::build_default_parameters;

    let VoiceProcessors {
        looper,
        pitch_shifter,
        envelope,
    } = voice_processors;
    let looper_handle = looper.handle().clone();
    let sequencer_handle = looper.sequencer_handle().clone();
    let triggers = make_shared(TrackTriggerModel::default());
    let parameter_values = build_default_parameters();

    LooperVoice {
        id,
        parameter_values: make_shared_cell(parameter_values),
        looper_handle,
        sequencer_handle,
        triggers,
        pitch_shifter_handle: pitch_shifter.handle().clone(),
        lfo1_handle: make_shared(LFOHandle::default()),
        lfo2_handle: make_shared(LFOHandle::default()),
        envelope: envelope.handle.clone(),
    }
}
