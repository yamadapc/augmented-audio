use std::ops::Deref;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use atomic_refcell::AtomicRefCell;
use basedrop::SharedCell;
use num::ToPrimitive;
use strum::{EnumProperty, IntoEnumIterator};

use audio_garbage_collector::{make_shared, make_shared_cell, Shared};
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_pitch_shifter::{
    MultiChannelPitchShifterProcessor, MultiChannelPitchShifterProcessorHandle,
};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
    VecAudioBuffer,
};
use augmented_atomics::{AtomicF32, AtomicValue};
use metronome::{MetronomeProcessor, MetronomeProcessorHandle};
use parameters::{
    CQuantizeMode, EnvelopeParameter, LFOParameter, LooperId, ParameterId, ParameterValue,
    QuantizationParameter, SceneId, SourceParameter, TempoControl,
};

use crate::audio_processor_metrics::{AudioProcessorMetrics, AudioProcessorMetricsHandle};
use crate::processor::handle::{LooperState, ToggleRecordingResult};
use crate::slice_worker::{SliceResult, SliceWorker};
use crate::tempo_estimation::estimate_tempo;
use crate::trigger_model::step_tracker::StepTracker;
use crate::trigger_model::{
    find_current_beat_trigger, find_running_beat_trigger, TrackTriggerModel,
};
use crate::{
    LoopSequencerProcessorHandle, LooperOptions, LooperProcessor, LooperProcessorHandle,
    QuantizeMode, TimeInfoProvider, TimeInfoProviderImpl,
};

pub mod parameters;

struct EnvelopeHandle {
    adsr_envelope: augmented_adsr_envelope::Envelope,
    enabled: AtomicBool,
}

struct EnvelopeProcessor {
    handle: Shared<EnvelopeHandle>,
}

impl Default for EnvelopeProcessor {
    fn default() -> Self {
        let envelope = augmented_adsr_envelope::Envelope::new();
        envelope.set_attack(Duration::from_secs_f32(0.0));
        envelope.set_decay(Duration::from_secs_f32(0.0));
        envelope.set_sustain(1.0);
        envelope.set_release(Duration::from_secs_f32(1_000_000.0));
        Self {
            handle: make_shared(EnvelopeHandle {
                adsr_envelope: envelope,
                enabled: AtomicBool::new(false),
            }),
        }
    }
}

impl AudioProcessor for EnvelopeProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.handle
            .adsr_envelope
            .set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if self.handle.enabled.load(Ordering::Relaxed) {
            for frame in data.frames_mut() {
                let volume = self.handle.adsr_envelope.volume();
                for sample in frame {
                    *sample = *sample * volume;
                }
                self.handle.adsr_envelope.tick();
            }
        }
    }
}

struct LFOHandle {
    amount: AtomicF32,
    frequency: AtomicF32,
}

impl Default for LFOHandle {
    fn default() -> Self {
        LFOHandle {
            amount: 1.0.into(),
            frequency: 1.0.into(),
        }
    }
}

pub struct LooperVoice {
    id: usize,
    parameter_values: SharedCell<im::HashMap<ParameterId, ParameterValue>>,
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

    pub fn sequencer(&self) -> &Shared<LoopSequencerProcessorHandle> {
        &self.sequencer_handle
    }
}

pub struct MultiTrackLooperHandle {
    voices: Vec<LooperVoice>,
    time_info_provider: Shared<TimeInfoProviderImpl>,
    sample_rate: AtomicF32,
    scene_value: AtomicF32,
    scene_parameters:
        SharedCell<im::HashMap<SceneId, im::HashMap<(LooperId, ParameterId), ParameterValue>>>,
    left_scene_id: AtomicUsize,
    right_scene_id: AtomicUsize,
    metronome_handle: Shared<MetronomeProcessorHandle>,
    slice_worker: SliceWorker,
    settings: SharedCell<AudioProcessorSettings>,
    metrics_handle: Shared<AudioProcessorMetricsHandle>,
}

impl MultiTrackLooperHandle {
    pub fn start_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper_handle.start_recording();
        }
    }

    pub fn set_scene_value(&self, value: f32) {
        self.scene_value.set(value);
    }

    pub fn toggle_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            let was_empty = self.all_loopers_empty_other_than(looper_id);
            match handle.looper_handle.toggle_recording() {
                ToggleRecordingResult::StoppedRecording => {
                    self.slice_worker.add_job(
                        looper_id.0,
                        *self.settings.get(),
                        handle.looper_handle.looper_clip(),
                    );

                    let parameters = handle.parameter_values.get();
                    let tempo_control = parameters.get(&ParameterId::ParameterIdQuantization(
                        QuantizationParameter::QuantizationParameterQuantizeMode,
                    ));

                    if was_empty
                        && tempo_control.cloned()
                            == Some(ParameterValue::Enum(
                                TempoControl::TempoControlSetGlobalTempo.to_usize().unwrap(),
                            ))
                    {
                        let estimated_tempo = estimate_tempo(
                            Default::default(),
                            self.sample_rate.get(),
                            handle.looper_handle.num_samples(),
                        )
                        .tempo;
                        if estimated_tempo > 300.0 {
                            log::warn!(
                                "This loop is too short tempo is ignored {}",
                                estimated_tempo
                            );
                            return;
                        }
                        log::info!("Setting global tempo to {}", estimated_tempo);
                        self.time_info_provider.set_tempo(estimated_tempo);
                        self.metronome_handle.set_tempo(estimated_tempo);
                        self.metronome_handle.set_is_playing(true);
                        self.time_info_provider.play();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set_source_parameter(
        &self,
        looper_id: LooperId,
        parameter: SourceParameter,
        value: f32,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match parameter {
                SourceParameter::Start => {
                    voice.looper_handle.set_start_offset(value);
                }
                SourceParameter::End => {
                    voice.looper_handle.set_end_offset(value);
                }
                SourceParameter::FadeStart => {
                    voice.looper_handle.set_fade_start(value);
                }
                SourceParameter::FadeEnd => {
                    voice.looper_handle.set_fade_end(value);
                }
                SourceParameter::Pitch => {
                    voice.pitch_shifter_handle.set_ratio(value);
                }
                SourceParameter::Speed => {
                    voice.looper_handle.set_speed(value);
                }
                _ => {}
            }

            let parameter_id = ParameterId::ParameterIdSource { parameter };
            Self::update_parameter_table(voice, parameter_id, ParameterValue::Float(value));
        }
    }

    fn update_parameter_table(
        voice: &LooperVoice,
        parameter_id: ParameterId,
        value: ParameterValue,
    ) {
        let parameter_values = voice.parameter_values.get();
        let mut parameter_values = parameter_values.deref().clone();
        parameter_values.insert(parameter_id, value);
        voice.parameter_values.set(make_shared(parameter_values));
    }

    fn update_handle_int(&self, voice: &LooperVoice, parameter_id: ParameterId, value: i32) {
        match parameter_id {
            ParameterId::ParameterIdSource { parameter } => match parameter {
                SourceParameter::SliceId => {
                    if let Some(slice) = self.slice_worker.result(voice.id) {
                        let markers = slice.markers();
                        let num_markers = markers.len();
                        let num_samples = voice.looper_handle.num_samples();

                        if num_markers == 0 || num_samples == 0 {
                            return;
                        }

                        let marker = &markers[(value as usize % num_markers).max(0)];
                        let offset = marker.position_samples as f32 / num_samples as f32;
                        voice.looper_handle.set_start_offset(offset);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn update_handle_float(&self, voice: &LooperVoice, parameter_id: ParameterId, value: f32) {
        match parameter_id {
            ParameterId::ParameterIdSource { parameter } => match parameter {
                SourceParameter::Start => {
                    voice.looper_handle.set_start_offset(value);
                }
                SourceParameter::End => {
                    voice.looper_handle.set_end_offset(value);
                }
                SourceParameter::FadeStart => {
                    voice.looper_handle.set_fade_start(value);
                }
                SourceParameter::FadeEnd => {
                    voice.looper_handle.set_fade_end(value);
                }
                SourceParameter::Pitch => {
                    voice.pitch_shifter_handle.set_ratio(value);
                }
                SourceParameter::Speed => {
                    voice.looper_handle.set_speed(value);
                }
                _ => {}
            },
            ParameterId::ParameterIdEnvelope { parameter } => match parameter {
                EnvelopeParameter::Attack => voice
                    .envelope
                    .adsr_envelope
                    .set_attack(Duration::from_secs_f32(value)),
                EnvelopeParameter::Decay => voice
                    .envelope
                    .adsr_envelope
                    .set_decay(Duration::from_secs_f32(value)),
                EnvelopeParameter::Release => voice
                    .envelope
                    .adsr_envelope
                    .set_release(Duration::from_secs_f32(value)),
                EnvelopeParameter::Sustain => voice.envelope.adsr_envelope.set_sustain(value),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn set_tempo(&self, tempo: f32) {
        let time_info_provider = self.time_info_provider();
        time_info_provider.set_tempo(tempo);
        self.metronome_handle.set_tempo(tempo);
    }

    pub fn set_envelope_parameter(
        &self,
        looper_id: LooperId,
        parameter_id: EnvelopeParameter,
        value: f32,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match parameter_id {
                EnvelopeParameter::Attack => voice
                    .envelope
                    .adsr_envelope
                    .set_attack(Duration::from_secs_f32(value)),
                EnvelopeParameter::Decay => voice
                    .envelope
                    .adsr_envelope
                    .set_decay(Duration::from_secs_f32(value)),
                EnvelopeParameter::Release => voice
                    .envelope
                    .adsr_envelope
                    .set_release(Duration::from_secs_f32(value)),
                EnvelopeParameter::Sustain => voice.envelope.adsr_envelope.set_sustain(value),
                _ => {}
            }

            Self::update_parameter_table(
                &voice,
                ParameterId::ParameterIdEnvelope {
                    parameter: parameter_id,
                },
                ParameterValue::Float(value),
            );
        }
    }

    pub fn set_quantization_mode(&self, looper_id: LooperId, mode: CQuantizeMode) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            voice.looper_handle.quantize_options().set_mode(match mode {
                CQuantizeMode::CQuantizeModeSnapClosest => QuantizeMode::SnapClosest,
                CQuantizeMode::CQuantizeModeSnapNext => QuantizeMode::SnapNext,
                CQuantizeMode::CQuantizeModeNone => QuantizeMode::None,
            });
            Self::update_parameter_table(
                &voice,
                ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterQuantizeMode,
                ),
                ParameterValue::Enum(mode.to_usize().unwrap()),
            )
        }
    }

    pub fn set_tempo_control(&self, looper_id: LooperId, mode: TempoControl) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(
                &voice,
                ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterTempoControl,
                ),
                ParameterValue::Enum(mode.to_usize().unwrap()),
            );
        }
    }

    pub fn set_lfo_parameter(
        &self,
        looper_id: LooperId,
        lfo: usize,
        parameter_id: LFOParameter,
        value: f32,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match parameter_id {
                LFOParameter::Frequency => match lfo {
                    1 => {
                        voice.lfo1_handle.frequency.set(value);
                    }
                    2 => {
                        voice.lfo2_handle.frequency.set(value);
                    }
                    _ => {}
                },
                LFOParameter::Amount => match lfo {
                    1 => {
                        voice.lfo1_handle.amount.set(value);
                    }
                    2 => {
                        voice.lfo2_handle.amount.set(value);
                    }
                    _ => {}
                },
            }

            Self::update_parameter_table(
                &voice,
                ParameterId::ParameterIdLFO {
                    lfo,
                    parameter: parameter_id,
                },
                ParameterValue::Float(value),
            );
        }
    }

    pub fn get_num_samples(&self, looper_id: LooperId) -> usize {
        if let Some(voice) = self.voices.get(looper_id.0) {
            voice.looper_handle.num_samples()
        } else {
            0
        }
    }

    pub fn get_looper_buffer(
        &self,
        looper_id: LooperId,
    ) -> Option<Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>> {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Some(voice.looper_handle.looper_clip())
        } else {
            None
        }
    }

    pub fn get_looper_state(&self, looper_id: LooperId) -> LooperState {
        self.voices[looper_id.0].looper().state()
    }

    pub fn get_looper_slices(&self, looper_id: LooperId) -> Option<SliceResult> {
        self.slice_worker.result(looper_id.0)
    }

    pub fn set_boolean_parameter(
        &self,
        looper_id: LooperId,
        parameter_id: ParameterId,
        value: bool,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match parameter_id {
                ParameterId::ParameterIdSource { parameter } => match parameter {
                    SourceParameter::LoopEnabled => {
                        voice.looper_handle.set_loop_enabled(value);
                    }
                    _ => {}
                },
                ParameterId::ParameterIdEnvelope { parameter } => match parameter {
                    EnvelopeParameter::EnvelopeEnabled => {
                        voice.envelope.enabled.store(value, Ordering::Relaxed);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn set_int_parameter(&self, looper_id: LooperId, parameter_id: ParameterId, value: i32) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(voice, parameter_id, ParameterValue::Int(value));
        }
    }

    pub fn add_scene_parameter_lock(
        &self,
        scene_id: SceneId,
        looper_id: LooperId,
        parameter_id: ParameterId,
        value: f32,
    ) {
        log::info!(
            "Scene lock scene={} looper={:?} parameter={:?} value={}",
            scene_id,
            looper_id,
            parameter_id,
            value
        );
        let all_scene_parameters = self.scene_parameters.get();
        let mut all_scene_parameters = all_scene_parameters.deref().clone();
        let mut scene_parameters = all_scene_parameters
            .get(&scene_id)
            .cloned()
            .unwrap_or(Default::default());
        scene_parameters.insert((looper_id, parameter_id), ParameterValue::Float(value));
        all_scene_parameters.insert(scene_id, scene_parameters);
        self.scene_parameters.set(make_shared(all_scene_parameters));
    }

    pub fn add_parameter_lock(
        &self,
        looper_id: LooperId,
        position_beats: usize,
        parameter_id: ParameterId,
        value: f32,
    ) {
        self.voices[looper_id.0]
            .triggers
            .add_lock(position_beats, parameter_id, value);
    }

    pub fn toggle_trigger(&self, looper_id: LooperId, position_beats: usize) {
        self.voices[looper_id.0]
            .triggers
            .toggle_trigger(position_beats);
    }

    pub fn get_position_percent(&self, looper_id: LooperId) -> f32 {
        if let Some(voice) = self.voices.get(looper_id.0) {
            let playhead = voice.looper_handle.playhead() as f32;
            let size = voice.looper_handle.num_samples();
            if size == 0 {
                0.0
            } else {
                playhead / size as f32
            }
        } else {
            0.0
        }
    }

    fn all_loopers_empty_other_than(&self, looper_id: LooperId) -> bool {
        self.voices.iter().enumerate().all(|(i, voice)| {
            i == looper_id.0 || matches!(voice.looper_handle.state(), LooperState::Empty)
        })
    }

    pub fn play(&self) {
        self.time_info_provider.play();
        if self.time_info_provider.get_time_info().tempo().is_some() {
            self.metronome_handle.set_is_playing(true);
        }
    }

    pub fn stop(&self) {
        self.metronome_handle.set_is_playing(false);
        self.time_info_provider.stop();
    }

    pub fn toggle_playback(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper_handle.toggle_playback();
        }
    }

    pub fn set_metronome_volume(&self, volume: f32) {
        self.metronome_handle.set_volume(volume);
    }

    pub fn set_volume(&self, looper_id: LooperId, volume: f32) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper_handle.set_wet_volume(volume);
        }
    }

    pub fn clear(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper_handle.clear();
            if self.all_loopers_empty_other_than(looper_id) {
                self.stop();
            }
        }
    }

    pub fn num_voices(&self) -> usize {
        self.voices.len()
    }

    pub fn voices(&self) -> &Vec<LooperVoice> {
        &self.voices
    }

    pub fn get(&self, looper_id: LooperId) -> Option<&LooperVoice> {
        self.voices.get(looper_id.0)
    }

    pub fn time_info_provider(&self) -> &Shared<TimeInfoProviderImpl> {
        &self.time_info_provider
    }

    pub fn metrics_handle(&self) -> &Shared<AudioProcessorMetricsHandle> {
        &self.metrics_handle
    }
}

pub struct MultiTrackLooper {
    graph: AudioProcessorGraph,
    handle: Shared<MultiTrackLooperHandle>,
    step_trackers: Vec<StepTracker>,
    metrics: AudioProcessorMetrics,
}

struct VoiceProcessors {
    looper: LooperProcessor,
    pitch_shifter: MultiChannelPitchShifterProcessor,
    envelope: EnvelopeProcessor,
}

impl MultiTrackLooper {
    pub fn new(options: LooperOptions, num_voices: usize) -> Self {
        let time_info_provider = make_shared(TimeInfoProviderImpl::new(options.host_callback));
        let processors: Vec<VoiceProcessors> = (0..num_voices)
            .map(|_| Self::build_voice_processor(&options, &time_info_provider))
            .collect();

        let metronome = metronome::MetronomeProcessor::new();
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);
        metronome_handle.set_volume(0.7);

        let voices = processors
            .iter()
            .enumerate()
            .map(|(i, voice_processors)| Self::build_voice_handle(i, voice_processors))
            .collect();
        let metrics = AudioProcessorMetrics::default();
        let handle = make_shared(MultiTrackLooperHandle {
            voices,
            time_info_provider,
            scene_value: AtomicF32::new(0.0),
            scene_parameters: make_shared_cell(Default::default()),
            left_scene_id: AtomicUsize::new(0),
            right_scene_id: AtomicUsize::new(1),
            sample_rate: AtomicF32::new(44100.0),
            metronome_handle,
            settings: make_shared_cell(AudioProcessorSettings::default()),
            slice_worker: SliceWorker::new(),
            metrics_handle: metrics.handle(),
        });

        let step_trackers = processors.iter().map(|_| StepTracker::default()).collect();

        let graph = Self::build_audio_graph(processors, metronome);

        Self {
            graph,
            handle,
            step_trackers,
            metrics,
        }
    }

    fn build_audio_graph(
        processors: Vec<VoiceProcessors>,
        metronome: MetronomeProcessor,
    ) -> AudioProcessorGraph {
        let mut graph = AudioProcessorGraph::default();
        let metronome_idx = graph.add_node(NodeType::Buffer(Box::new(metronome)));
        graph
            .add_connection(graph.input(), metronome_idx)
            .expect("Shouldn't produce loop");
        graph
            .add_connection(metronome_idx, graph.output())
            .expect("Shouldn't produce loop");

        for VoiceProcessors {
            looper,
            pitch_shifter,
            envelope,
        } in processors
        {
            let looper_idx = graph.add_node(NodeType::Simple(Box::new(looper)));
            let pitch_shifter_idx = graph.add_node(NodeType::Buffer(Box::new(pitch_shifter)));
            let envelope_idx = graph.add_node(NodeType::Buffer(Box::new(envelope)));

            graph
                .add_connection(graph.input(), looper_idx)
                .expect("Shouldn't produce loop");
            graph
                .add_connection(looper_idx, pitch_shifter_idx)
                .expect("Shouldn't produce loop");
            graph
                .add_connection(pitch_shifter_idx, envelope_idx)
                .expect("Shouldn't produce loop");
            graph
                .add_connection(envelope_idx, graph.output())
                .expect("Shouldn't produce loop");
        }

        graph
    }

    fn build_voice_handle(id: usize, voice_processors: &VoiceProcessors) -> LooperVoice {
        let VoiceProcessors {
            looper,
            pitch_shifter,
            envelope,
        } = voice_processors;
        let looper_handle = looper.handle().clone();
        let sequencer_handle = looper.sequencer_handle().clone();
        let triggers = make_shared(TrackTriggerModel::default());
        let parameter_values = Self::build_default_parameters();

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

    fn build_default_parameters() -> im::HashMap<ParameterId, ParameterValue> {
        let source_parameters: Vec<ParameterId> = SourceParameter::iter()
            .map(|parameter| ParameterId::ParameterIdSource { parameter })
            .collect();
        let envelope_parameters: Vec<ParameterId> = EnvelopeParameter::iter()
            .map(|parameter| ParameterId::ParameterIdEnvelope { parameter })
            .collect();
        let lfo_parameters: Vec<ParameterId> = LFOParameter::iter()
            .map(|parameter| ParameterId::ParameterIdLFO { lfo: 0, parameter })
            .collect();
        let quantization_parameters: Vec<ParameterId> = QuantizationParameter::iter()
            .map(ParameterId::ParameterIdQuantization)
            .collect();
        let all_parameters = source_parameters
            .iter()
            .chain(envelope_parameters.iter())
            .chain(lfo_parameters.iter())
            .chain(quantization_parameters.iter());

        all_parameters
            .flat_map(|parameter_id| {
                let default_value = match parameter_id.get_str("type").unwrap() {
                    "float" => {
                        let f_str = parameter_id.get_str("default").unwrap();
                        let f = f32::from_str(f_str).unwrap();
                        ParameterValue::Float(f)
                    }
                    "bool" => {
                        let b_str = parameter_id.get_str("default").unwrap();
                        ParameterValue::Bool(b_str == "true")
                    }
                    "enum" => {
                        let e_str = parameter_id.get_str("default").unwrap();
                        let e = usize::from_str(e_str).unwrap();
                        ParameterValue::Enum(e)
                    }
                    "int" => {
                        if let Some(i_str) = parameter_id.get_str("default") {
                            let i = i32::from_str(i_str).unwrap();
                            ParameterValue::Int(i)
                        } else {
                            ParameterValue::None
                        }
                    }
                    _ => panic!("Invalid parameter declaration"),
                };

                if let ParameterId::ParameterIdLFO { parameter, .. } = parameter_id {
                    (0..2)
                        .map(|lfo| {
                            (
                                ParameterId::ParameterIdLFO {
                                    lfo,
                                    parameter: parameter.clone(),
                                },
                                default_value.clone(),
                            )
                        })
                        .collect()
                } else {
                    vec![(parameter_id.clone(), default_value)]
                }
            })
            .collect()
    }

    fn build_voice_processor(
        options: &LooperOptions,
        time_info_provider: &Shared<TimeInfoProviderImpl>,
    ) -> VoiceProcessors {
        let looper = LooperProcessor::new(options.clone(), time_info_provider.clone());
        looper
            .handle()
            .quantize_options()
            .set_mode(QuantizeMode::SnapNext);
        looper.handle().tick_time.store(false, Ordering::Relaxed);

        let pitch_shifter = MultiChannelPitchShifterProcessor::default();
        let envelope = EnvelopeProcessor::default();

        VoiceProcessors {
            looper,
            pitch_shifter,
            envelope,
        }
    }

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }

    fn process_triggers(&mut self) {
        if let Some(position_beats) = self
            .handle
            .time_info_provider
            .get_time_info()
            .position_beats()
        {
            for (voice, step_tracker) in self.handle.voices.iter().zip(&mut self.step_trackers) {
                let triggers = voice.triggers();

                // let parameter_values = voice.parameter_values.get();
                // for (parameter_id, parameter_value) in parameter_values.deref().iter() {
                //     self.handle
                //         .update_handle(voice, parameter_id.clone(), parameter_value.value);
                // }

                if let Some(_trigger) =
                    find_current_beat_trigger(triggers, step_tracker, position_beats)
                {
                    voice.looper_handle.trigger();
                    voice.envelope.adsr_envelope.note_on();
                }

                let triggers_vec = triggers.triggers();
                let triggers = find_running_beat_trigger(triggers, &triggers_vec, position_beats);
                // let mut has_triggers = false;
                for trigger in triggers {
                    // has_triggers = true;
                    for (parameter_id, lock) in trigger.locks() {
                        self.handle
                            .update_handle_float(voice, parameter_id.clone(), lock.value());
                    }
                }

                // if !has_triggers {
                //     voice.envelope.adsr_envelope.note_off();
                // }
            }
        }
    }

    fn process_scenes(&mut self) {
        let all_scene_parameters = self.handle.scene_parameters.get();
        let left_parameters = all_scene_parameters.get(&self.handle.left_scene_id.get());
        let right_parameters = all_scene_parameters.get(&self.handle.right_scene_id.get());
        let scene_value = self.handle.scene_value.get();

        for (index, voice) in self.handle.voices.iter().enumerate() {
            let looper_id = LooperId(index);
            let parameter_values = voice.parameter_values.get();

            for (parameter_id, parameter_value) in parameter_values.iter() {
                let key = (looper_id, parameter_id.clone());
                let left_value = left_parameters
                    .map(|ps| ps.get(&key))
                    .flatten()
                    .cloned()
                    .unwrap_or(parameter_value.clone());
                let right_value = right_parameters
                    .map(|ps| ps.get(&key))
                    .flatten()
                    .cloned()
                    .unwrap_or(parameter_value.clone());

                match (left_value, right_value) {
                    (ParameterValue::Float(left_value), ParameterValue::Float(right_value)) => {
                        let value = left_value + scene_value * (right_value - left_value);
                        self.handle
                            .update_handle_float(voice, parameter_id.clone(), value);
                    }
                    (ParameterValue::Int(left_value), ParameterValue::Int(right_value)) => {
                        let value =
                            left_value + (scene_value * (right_value - left_value) as f32) as i32;
                        self.handle
                            .update_handle_int(voice, parameter_id.clone(), value);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl AudioProcessor for MultiTrackLooper {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.graph.prepare(settings);
        self.handle.sample_rate.set(settings.sample_rate());
        self.handle.metrics_handle.prepare(settings);
        self.handle.settings.set(make_shared(settings));
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.metrics.on_process_start();

        self.process_scenes();
        self.process_triggers();

        self.graph.process(data);

        for _sample in data.frames() {
            self.handle.time_info_provider.tick();
        }

        self.metrics.on_process_end();
    }
}

impl MidiEventHandler for MultiTrackLooper {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_build_parameters_table() {
        let table = MultiTrackLooper::build_default_parameters();
        let parameters: Vec<ParameterId> = table.iter().map(|(id, _)| id).cloned().collect();
        let start_index = parameters
            .iter()
            .cloned()
            .find_position(|id| {
                *id == ParameterId::ParameterIdSource {
                    parameter: SourceParameter::Start,
                }
            })
            .unwrap()
            .0;
        let slice_index = parameters
            .iter()
            .cloned()
            .find_position(|id| {
                *id == ParameterId::ParameterIdSource {
                    parameter: SourceParameter::SliceId,
                }
            })
            .unwrap()
            .0;
        assert!(slice_index > start_index);
    }

    #[test]
    fn test_set_scene_parameter_lock() {
        let mut looper = MultiTrackLooper::new(Default::default(), 8);

        let start_parameter = ParameterId::ParameterIdSource {
            parameter: SourceParameter::Start,
        };
        looper
            .handle
            .set_source_parameter(LooperId(0), SourceParameter::Start, 0.3);
        looper
            .handle
            .add_scene_parameter_lock(0, LooperId(0), start_parameter, 0.8);
        looper.process_scenes();
    }

    #[test]
    fn test_starts_empty() {
        let looper = MultiTrackLooper::new(Default::default(), 8);
        assert_eq!(
            looper.handle.all_loopers_empty_other_than(LooperId(0)),
            true
        );
    }
}
