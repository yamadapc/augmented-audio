use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use assert_no_alloc::assert_no_alloc;
use atomic_refcell::AtomicRefCell;
use basedrop::SharedCell;
use num::ToPrimitive;

use audio_garbage_collector::{make_shared, make_shared_cell, Shared};
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
    VecAudioBuffer,
};
use augmented_atomics::{AtomicF32, AtomicValue};
use augmented_oscillator::Oscillator;
use looper_voice::{LooperVoice, VoiceProcessors};
use metrics::audio_processor_metrics::{AudioProcessorMetrics, AudioProcessorMetricsHandle};
use metronome::{MetronomeProcessor, MetronomeProcessorHandle};
use parameters::{
    CQuantizeMode, EnvelopeParameter, LFOParameter, LooperId, ParameterId, ParameterValue,
    QuantizationParameter, SceneId, SourceParameter, TempoControl,
};
use slice_worker::{SliceResult, SliceWorker};
use tempo_estimation::estimate_tempo;
use trigger_model::step_tracker::StepTracker;
use trigger_model::{find_current_beat_trigger, find_running_beat_trigger};

use crate::multi_track_looper::lfo_processor::LFOHandle;
use crate::multi_track_looper::midi_button::{MIDIButton, MIDIButtonEvent};
use crate::multi_track_looper::midi_store::MidiStoreHandle;
use crate::multi_track_looper::scene_state::SceneHandle;
use crate::processor::handle::{LooperHandleThread, LooperState, ToggleRecordingResult};
use crate::time_info_provider::TimeInfoMetronomePlayhead;
use crate::{LooperOptions, QuantizeMode, TimeInfoProvider, TimeInfoProviderImpl};

pub(crate) mod allocator;
mod copy_paste;
mod envelope_processor;
mod lfo_processor;
mod long_backoff;
mod looper_voice;
pub(crate) mod metrics;
mod midi_button;
pub(crate) mod midi_store;
pub mod parameters;
mod parameters_map;
mod scene_state;
pub(crate) mod slice_worker;
mod tempo_estimation;
mod trigger_model;

pub struct MultiTrackLooperHandle {
    voices: Vec<LooperVoice>,
    time_info_provider: Shared<TimeInfoProviderImpl>,
    sample_rate: AtomicF32,
    scene_parameters: SceneHandle,
    metronome_handle: Shared<MetronomeProcessorHandle>,
    slice_worker: SliceWorker,
    settings: SharedCell<AudioProcessorSettings>,
    metrics_handle: Shared<AudioProcessorMetricsHandle>,
    midi_store: Shared<MidiStoreHandle>,
    active_looper: AtomicUsize,
}

impl MultiTrackLooperHandle {
    pub(crate) fn get_parameter(
        &self,
        looper_id: LooperId,
        id: &ParameterId,
    ) -> Option<ParameterValue> {
        self.voices
            .get(looper_id.0)
            .map(|voice| voice.user_parameters().get(id.clone()).clone())
    }

    pub(crate) fn set_parameter(
        &self,
        looper_id: LooperId,
        id: ParameterId,
        value: ParameterValue,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(voice, id, value);
        }
    }
}

impl MultiTrackLooperHandle {
    pub fn start_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper().start_recording();
        }
    }

    pub fn set_scene_value(&self, value: f32) {
        self.scene_parameters.set_slider(value);
    }

    /// If the active looper is empty, start recording, otherwise start playback
    pub fn on_multi_mode_record_play_pressed(&self) {
        let looper_id = self.active_looper.get();
        let voice = &self.voices[looper_id];
        let looper_id = LooperId(looper_id);
        let state = voice.looper().state();
        if state == LooperState::Empty {
            self.toggle_recording(looper_id, LooperHandleThread::AudioThread);
        } else if state == LooperState::Paused {
            self.toggle_playback(looper_id)
        } else if state == LooperState::Overdubbing
            || state == LooperState::Recording
            || state == LooperState::Playing
        {
            self.toggle_recording(looper_id, LooperHandleThread::AudioThread);
        } // TODO what else
    }

    pub fn stop_active_looper(&self) {
        let looper_id = self.active_looper.get();
        if let Some(voice) = self.voices.get(looper_id) {
            voice.looper().stop();
        }
    }

    pub fn clear_active_looper(&self) {
        let looper_id = self.active_looper.get();
        self.clear(LooperId(looper_id))
    }

    pub fn set_active_looper(&self, looper_id: LooperId) {
        self.active_looper.set(looper_id.0);
    }

    pub fn toggle_recording(&self, looper_id: LooperId, thread: LooperHandleThread) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            let was_empty = self.all_loopers_empty_other_than(looper_id);
            let toggle_recording_result = handle.looper().toggle_recording(thread);
            if let ToggleRecordingResult::StoppedRecording = toggle_recording_result {
                self.slice_worker.add_job(
                    looper_id.0,
                    *self.settings.get(),
                    handle.looper().looper_clip(),
                );

                let parameters = handle.user_parameters();
                let tempo_control = parameters.get(ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterTempoControl,
                ));

                if was_empty
                    && tempo_control.as_enum()
                        == TempoControl::TempoControlSetGlobalTempo.to_usize().unwrap()
                {
                    let estimated_tempo = estimate_tempo(
                        Default::default(),
                        self.sample_rate.get(),
                        handle.looper().num_samples(),
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
                    voice.looper().set_start_offset(value);
                }
                SourceParameter::End => {
                    voice.looper().set_end_offset(value);
                }
                SourceParameter::FadeStart => {
                    voice.looper().set_fade_start(value);
                }
                SourceParameter::FadeEnd => {
                    voice.looper().set_fade_end(value);
                }
                SourceParameter::Pitch => {
                    voice.pitch_shifter().set_ratio(value);
                }
                SourceParameter::Speed => {
                    voice.looper().set_speed(value);
                }
                _ => {}
            }

            let parameter_id = ParameterId::ParameterIdSource(parameter);
            Self::update_parameter_table(voice, parameter_id, ParameterValue::Float(value.into()));
        }
    }

    fn update_parameter_table(
        voice: &LooperVoice,
        parameter_id: ParameterId,
        value: ParameterValue,
    ) {
        let parameter_values = voice.user_parameters();
        let _ = parameter_values.set(parameter_id, value);
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    fn update_handle_int(&self, voice: &LooperVoice, parameter_id: ParameterId, value: i32) {
        match parameter_id {
            ParameterId::ParameterIdSource(parameter) => match parameter {
                SourceParameter::SliceId => {
                    let slice_enabled = voice
                        .user_parameters()
                        .get(SourceParameter::SliceEnabled)
                        .as_bool();

                    if !slice_enabled {
                        return;
                    }

                    if let Some(slice) = self.slice_worker.result(voice.id) {
                        let markers = slice.markers();
                        let num_markers = markers.len();
                        let num_samples = voice.looper().num_samples();

                        if num_markers == 0 || num_samples == 0 {
                            return;
                        }

                        let marker = &markers[(value as usize % num_markers).max(0)];
                        let offset = marker.position_samples as f32 / num_samples as f32;
                        voice.looper().set_start_offset(offset);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn update_handle_float(&self, voice: &LooperVoice, parameter_id: ParameterId, value: f32) {
        match parameter_id {
            ParameterId::ParameterIdSource(parameter) => match parameter {
                SourceParameter::Start => {
                    voice.looper().set_start_offset(value);
                }
                SourceParameter::End => {
                    voice.looper().set_end_offset(value);
                }
                SourceParameter::FadeStart => {
                    voice.looper().set_fade_start(value);
                }
                SourceParameter::FadeEnd => {
                    voice.looper().set_fade_end(value);
                }
                SourceParameter::Pitch => {
                    voice.pitch_shifter().set_ratio(value);
                }
                SourceParameter::Speed => {
                    voice.looper().set_speed(value);
                }
                _ => {}
            },
            ParameterId::ParameterIdEnvelope(parameter) => match parameter {
                EnvelopeParameter::Attack => voice
                    .envelope()
                    .adsr_envelope
                    .set_attack(Duration::from_secs_f32(value)),
                EnvelopeParameter::Decay => voice
                    .envelope()
                    .adsr_envelope
                    .set_decay(Duration::from_secs_f32(value)),
                EnvelopeParameter::Release => voice
                    .envelope()
                    .adsr_envelope
                    .set_release(Duration::from_secs_f32(value)),
                EnvelopeParameter::Sustain => voice.envelope().adsr_envelope.set_sustain(value),
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
                    .envelope()
                    .adsr_envelope
                    .set_attack(Duration::from_secs_f32(value)),
                EnvelopeParameter::Decay => voice
                    .envelope()
                    .adsr_envelope
                    .set_decay(Duration::from_secs_f32(value)),
                EnvelopeParameter::Release => voice
                    .envelope()
                    .adsr_envelope
                    .set_release(Duration::from_secs_f32(value)),
                EnvelopeParameter::Sustain => voice.envelope().adsr_envelope.set_sustain(value),
                _ => {}
            }

            Self::update_parameter_table(
                voice,
                ParameterId::ParameterIdEnvelope(parameter_id),
                ParameterValue::Float(value.into()),
            );
        }
    }

    pub fn set_quantization_mode(&self, looper_id: LooperId, mode: CQuantizeMode) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            voice.looper().quantize_options().set_mode(match mode {
                CQuantizeMode::CQuantizeModeSnapClosest => QuantizeMode::SnapClosest,
                CQuantizeMode::CQuantizeModeSnapNext => QuantizeMode::SnapNext,
                CQuantizeMode::CQuantizeModeNone => QuantizeMode::None,
            });
            Self::update_parameter_table(
                voice,
                ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterQuantizeMode,
                ),
                ParameterValue::Enum(mode.to_usize().unwrap().into()),
            )
        }
    }

    pub fn set_tempo_control(&self, looper_id: LooperId, mode: TempoControl) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(
                voice,
                ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterTempoControl,
                ),
                ParameterValue::Enum(mode.to_usize().unwrap().into()),
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
                    0 => {
                        voice.lfo1().set_frequency(value);
                    }
                    1 => {
                        voice.lfo2().set_frequency(value);
                    }
                    _ => {}
                },
                LFOParameter::Amount => match lfo {
                    0 => {
                        voice.lfo1().set_amount(value);
                    }
                    1 => {
                        voice.lfo2().set_amount(value);
                    }
                    _ => {}
                },
            }

            Self::update_parameter_table(
                voice,
                ParameterId::ParameterIdLFO(lfo, parameter_id),
                ParameterValue::Float(value.into()),
            );
        }
    }

    pub fn get_num_samples(&self, looper_id: LooperId) -> usize {
        if let Some(voice) = self.voices.get(looper_id.0) {
            voice.looper().num_samples()
        } else {
            0
        }
    }

    pub fn get_looper_buffer(
        &self,
        looper_id: LooperId,
    ) -> Option<Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>> {
        self.voices
            .get(looper_id.0)
            .map(|voice| voice.looper().looper_clip())
    }

    pub fn get_looper_state(&self, looper_id: LooperId) -> LooperState {
        self.voices[looper_id.0].looper().state()
    }

    pub fn get_looper_slices(&self, looper_id: LooperId) -> Option<SliceResult> {
        self.slice_worker.result(looper_id.0)
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub fn set_boolean_parameter(
        &self,
        looper_id: LooperId,
        parameter_id: ParameterId,
        value: bool,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match &parameter_id {
                ParameterId::ParameterIdSource(parameter) => match parameter {
                    SourceParameter::LoopEnabled => {
                        voice.looper().set_loop_enabled(value);
                    }
                    _ => {}
                },
                ParameterId::ParameterIdEnvelope(parameter) => match parameter {
                    EnvelopeParameter::EnvelopeEnabled => {
                        voice.envelope().enabled.store(value, Ordering::Relaxed);
                    }
                    _ => {}
                },
                _ => {}
            }

            let parameters = voice.user_parameters();
            let _ = parameters.set(parameter_id, ParameterValue::Bool(value.into()));
        }
    }

    pub fn set_int_parameter(&self, looper_id: LooperId, parameter_id: ParameterId, value: i32) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(voice, parameter_id, ParameterValue::Int(value.into()));
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
        let all_scene_parameters = &self.scene_parameters;
        all_scene_parameters.set(scene_id, looper_id, parameter_id, value);
    }

    pub fn remove_scene_parameter_lock(
        &self,
        scene_id: SceneId,
        looper_id: LooperId,
        parameter_id: ParameterId,
    ) {
        log::info!(
            "Removing scene lock scene={} looper={:?} parameter={:?}",
            scene_id,
            looper_id,
            parameter_id
        );
        let all_scene_parameters = &self.scene_parameters;
        all_scene_parameters.unset(scene_id, looper_id, parameter_id);
    }

    pub fn add_parameter_lock(
        &self,
        looper_id: LooperId,
        position_beats: usize,
        parameter_id: ParameterId,
        value: f32,
    ) {
        self.voices[looper_id.0]
            .trigger_model()
            .add_lock(position_beats, parameter_id, value);
    }

    pub fn remove_parameter_lock(
        &self,
        looper_id: LooperId,
        position_beats: usize,
        parameter_id: ParameterId,
    ) {
        self.voices[looper_id.0]
            .trigger_model()
            .remove_lock(position_beats, parameter_id);
    }

    pub fn toggle_trigger(&self, looper_id: LooperId, position_beats: usize) {
        self.voices[looper_id.0]
            .trigger_model()
            .toggle_trigger(position_beats);
    }

    pub fn get_position_percent(&self, looper_id: LooperId) -> f32 {
        if let Some(voice) = self.voices.get(looper_id.0) {
            let playhead = voice.looper().playhead() as f32;
            let size = voice.looper().num_samples();
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
            i == looper_id.0 || matches!(voice.looper().state(), LooperState::Empty)
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
            // TODO: We might want to stop the playhead
            // Depending on whether there are other loopers playing
            if handle.looper().toggle_playback() {
                self.play();
            }
        }
    }

    pub fn set_metronome_volume(&self, volume: f32) {
        self.metronome_handle.set_volume(volume);
    }

    pub fn set_volume(&self, looper_id: LooperId, volume: f32) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper().set_wet_volume(volume);
        }
    }

    pub fn clear(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper().clear();
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

    pub fn midi(&self) -> &Shared<MidiStoreHandle> {
        &self.midi_store
    }

    pub fn add_lfo_mapping(
        &self,
        looper_id: LooperId,
        lfo_id: usize,
        parameter_id: ParameterId,
        value: f32,
    ) {
        let value = if (value - 0.0).abs() > f32::EPSILON {
            Some(value)
        } else {
            None
        };

        if let Some(voice) = self.voices.get(looper_id.0) {
            let lfo = if lfo_id == 0 {
                voice.lfo1()
            } else {
                voice.lfo2()
            };
            log::info!(
                "Adding mapping to LFO={} parameter={:?} looper={:?}",
                lfo_id,
                parameter_id,
                looper_id
            );
            lfo.set_parameter_map(parameter_id, value);
        }
    }

    pub fn remove_lfo_mapping(
        &self,
        looper_id: LooperId,
        lfo_id: usize,
        parameter_id: ParameterId,
    ) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            let lfo = if lfo_id == 0 {
                voice.lfo1()
            } else {
                voice.lfo2()
            };
            log::info!(
                "Removing mapping to LFO={} parameter={:?} looper={:?}",
                lfo_id,
                parameter_id,
                looper_id
            );
            lfo.set_parameter_map(parameter_id, None);
        }
    }
}

pub struct MultiTrackLooper {
    graph: AudioProcessorGraph,
    handle: Shared<MultiTrackLooperHandle>,
    step_trackers: Vec<StepTracker>,
    lfos: Vec<(Oscillator<f32>, Oscillator<f32>)>,
    metrics: AudioProcessorMetrics,
    parameters_scratch: Vec<Vec<ParameterValue>>,
    parameter_scratch_indexes: HashMap<ParameterId, usize>,
    record_midi_button: MIDIButton,
}

impl Default for MultiTrackLooper {
    fn default() -> Self {
        Self::new(LooperOptions::default(), 8)
    }
}

impl MultiTrackLooper {
    pub fn new(options: LooperOptions, num_voices: usize) -> Self {
        let time_info_provider = make_shared(TimeInfoProviderImpl::new(options.host_callback));
        let processors: Vec<VoiceProcessors> = (0..num_voices)
            .map(|_| looper_voice::build_voice_processor(&options, &time_info_provider))
            .collect();

        let metronome = metronome::MetronomeProcessor::new(TimeInfoMetronomePlayhead(
            time_info_provider.clone(),
        ));
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);
        metronome_handle.set_volume(0.7);

        let voices: Vec<LooperVoice> = processors
            .iter()
            .enumerate()
            .map(|(i, voice_processors)| looper_voice::build_voice_handle(i, voice_processors))
            .collect();

        let parameters_scratch = voices
            .iter()
            .map(|voice| {
                voice
                    .parameter_ids()
                    .iter()
                    .map(|id| voice.user_parameters().get(id.clone()).clone())
                    .collect()
            })
            .collect();
        let parameter_scratch_indexes = voices[0]
            .parameter_ids()
            .iter()
            .enumerate()
            .map(|(idx, id)| (id.clone(), idx))
            .collect();

        let metrics = AudioProcessorMetrics::default();
        let handle = make_shared(MultiTrackLooperHandle {
            voices,
            time_info_provider,
            scene_parameters: SceneHandle::new(8, 2),
            sample_rate: AtomicF32::new(44100.0),
            metronome_handle,
            settings: make_shared_cell(AudioProcessorSettings::default()),
            slice_worker: SliceWorker::new(),
            metrics_handle: metrics.handle(),
            midi_store: make_shared(midi_store::MidiStoreHandle::default()),
            active_looper: AtomicUsize::new(0),
        });

        let step_trackers = processors.iter().map(|_| StepTracker::default()).collect();
        let lfos = processors
            .iter()
            .map(|_| (Oscillator::sine(44100.0), Oscillator::sine(44100.0)))
            .collect();

        let graph = Self::build_audio_graph(processors, metronome);

        Self {
            graph,
            handle,
            step_trackers,
            parameters_scratch,
            parameter_scratch_indexes,
            lfos,
            metrics,
            record_midi_button: MIDIButton::new(),
        }
    }

    fn build_audio_graph(
        processors: Vec<VoiceProcessors>,
        metronome: MetronomeProcessor<TimeInfoMetronomePlayhead>,
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
            let looper_idx = graph.add_node(NodeType::Buffer(Box::new(
                audio_processor_traits::simple_processor::BufferProcessor(looper),
            )));
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
                let triggers = voice.trigger_model();

                // let parameter_values = voice.parameter_values.get();
                // for (parameter_id, parameter_value) in parameter_values.deref().iter() {
                //     self.handle
                //         .update_handle(voice, parameter_id.clone(), parameter_value.value);
                // }

                let triggers_vec = triggers.triggers();
                if let Some(_trigger) =
                    find_current_beat_trigger(triggers, &triggers_vec, step_tracker, position_beats)
                {
                    voice.looper().trigger();
                    voice.envelope().adsr_envelope.note_on();
                }

                let index = voice.id;
                let parameters_scratch = &mut self.parameters_scratch[index];

                let triggers = find_running_beat_trigger(triggers, &triggers_vec, position_beats);
                // let mut has_triggers = false;
                for trigger in triggers {
                    // has_triggers = true;
                    for (parameter_id, lock) in trigger.locks() {
                        let parameter_idx = self.parameter_scratch_indexes[parameter_id];
                        parameters_scratch[parameter_idx] =
                            ParameterValue::Float(lock.value().into());
                    }
                }

                // if !has_triggers {
                //     voice.envelope.adsr_envelope.note_off();
                // }
            }
        }
    }

    fn process_lfos(&mut self) {
        for ((lfo1, lfo2), voice) in self.lfos.iter_mut().zip(self.handle.voices.iter()) {
            if let ParameterValue::Float(lfo_freq_1) = &self.parameters_scratch[voice.id][self
                .parameter_scratch_indexes
                [&ParameterId::ParameterIdLFO(0, LFOParameter::Frequency)]]
            {
                lfo1.set_frequency(lfo_freq_1.get());
            }
            if let ParameterValue::Float(lfo_freq_2) = &self.parameters_scratch[voice.id][self
                .parameter_scratch_indexes
                [&ParameterId::ParameterIdLFO(1, LFOParameter::Frequency)]]
            {
                lfo2.set_frequency(lfo_freq_2.get());
            }
            let lfo1_amount = if let ParameterValue::Float(value) = &self.parameters_scratch
                [voice.id][self.parameter_scratch_indexes
                [&ParameterId::ParameterIdLFO(0, LFOParameter::Amount)]]
            {
                value.get()
            } else {
                1.0
            };
            let lfo2_amount = if let ParameterValue::Float(value) = &self.parameters_scratch
                [voice.id][self.parameter_scratch_indexes
                [&ParameterId::ParameterIdLFO(1, LFOParameter::Amount)]]
            {
                value.get()
            } else {
                1.0
            };

            let lfo1_handle = voice.lfo1();
            let lfo2_handle = voice.lfo2();

            let run_mapping = |parameter: &ParameterId,
                               value: &ParameterValue,
                               lfo_amount: f32,
                               lfo_handle: &Shared<LFOHandle>,
                               lfo: &mut Oscillator<f32>|
             -> Option<f32> {
                let modulation_amount = lfo_handle.modulation_amount(parameter);
                if let ParameterValue::Float(value) = value {
                    let value = lfo.get() * modulation_amount * lfo_amount + value.get();
                    return Some(value);
                }
                None
            };

            for (parameter_idx, parameter) in voice.parameter_ids().iter().enumerate() {
                if let Some(value) = run_mapping(
                    parameter,
                    &self.parameters_scratch[voice.id][parameter_idx],
                    lfo1_amount,
                    lfo1_handle,
                    lfo1,
                ) {
                    self.parameters_scratch[voice.id][parameter_idx] =
                        ParameterValue::Float(value.into());
                }
                if let Some(value) = run_mapping(
                    parameter,
                    &self.parameters_scratch[voice.id][parameter_idx],
                    lfo2_amount,
                    lfo2_handle,
                    lfo2,
                ) {
                    self.parameters_scratch[voice.id][parameter_idx] =
                        ParameterValue::Float(value.into());
                }
            }
        }
    }

    fn process_scenes(&mut self) {
        let scene_value = self.handle.scene_parameters.get_slider();

        for (index, voice) in self.handle.voices.iter().enumerate() {
            let looper_id = LooperId(index);
            let parameter_values = voice.user_parameters();

            for parameter_id in voice.parameter_ids() {
                let parameter_value = parameter_values.get(parameter_id.clone());

                let _key = (looper_id, parameter_id.clone());
                let left_value = self
                    .handle
                    .scene_parameters
                    .get_left(looper_id, parameter_id.clone())
                    .unwrap_or(parameter_value);
                let right_value = self
                    .handle
                    .scene_parameters
                    .get_right(looper_id, parameter_id.clone())
                    .unwrap_or(parameter_value);

                match (left_value, right_value) {
                    (ParameterValue::Float(left_value), ParameterValue::Float(right_value)) => {
                        let value =
                            left_value.get() + scene_value * (right_value.get() - left_value.get());
                        self.parameters_scratch[voice.id]
                            [self.parameter_scratch_indexes[parameter_id]] =
                            ParameterValue::Float(value.into());
                    }
                    (ParameterValue::Int(left_value), ParameterValue::Int(right_value)) => {
                        let value = left_value.get()
                            + (scene_value * (right_value.get() - left_value.get()) as f32) as i32;
                        self.parameters_scratch[voice.id]
                            [self.parameter_scratch_indexes[parameter_id]] =
                            ParameterValue::Int(value.into());
                    }
                    _ => {}
                }
            }
        }
    }

    fn flush_parameters(&mut self) {
        for (voice, values) in self
            .handle
            .voices
            .iter()
            .zip(self.parameters_scratch.iter())
        {
            for (parameter, parameter_value) in voice.parameter_ids().iter().zip(values.iter()) {
                match parameter_value {
                    ParameterValue::Float(value) => {
                        self.handle
                            .update_handle_float(voice, parameter.clone(), value.get())
                    }
                    ParameterValue::Int(value) => {
                        self.handle
                            .update_handle_int(voice, parameter.clone(), value.get())
                    }
                    _ => {}
                }
            }
        }
    }

    fn tick_lfos(&mut self) {
        for (l1, l2) in self.lfos.iter_mut() {
            l1.tick();
            l2.tick();
        }
    }

    fn on_record_button_click_midi(&mut self, value: u8) {
        let event = self.record_midi_button.accept(value);
        self.handle_record_midi_button_event(event);
    }

    fn handle_record_midi_button_event(&mut self, event: Option<MIDIButtonEvent>) {
        if let Some(event) = event {
            match event {
                MIDIButtonEvent::ButtonDown => {
                    self.handle.on_multi_mode_record_play_pressed();
                }
                MIDIButtonEvent::DoubleTap => {
                    self.handle.stop_active_looper();
                }
                MIDIButtonEvent::Hold => {
                    self.handle.clear_active_looper();
                }
                _ => {}
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

        for (o1, o2) in self.lfos.iter_mut() {
            o1.set_sample_rate(settings.sample_rate());
            o2.set_sample_rate(settings.sample_rate());
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        assert_no_alloc(|| {
            self.metrics.on_process_start();

            self.process_scenes();
            self.process_triggers();
            self.process_lfos();
            self.flush_parameters();

            self.graph.process(data);

            for _sample in data.frames() {
                self.handle.time_info_provider.tick();
                self.tick_lfos();
            }

            self.metrics.on_process_end();
        });
    }
}

impl MidiEventHandler for MultiTrackLooper {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        let midi_store = self.handle.midi_store.clone();
        MidiStoreHandle::process_midi_events(&midi_store, midi_messages, self);

        let event = self.record_midi_button.tick();
        self.handle_record_midi_button_event(event);
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::{assert_f_eq, sine_buffer};
    use basedrop::Owned;

    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};
    use audio_processor_traits::InterleavedAudioBuffer;

    use crate::midi_map::MidiControllerNumber;
    use crate::parameters::EntityId;

    use super::*;

    #[test]
    fn test_process_scenes_will_update_parameters_scratch() {
        let mut looper = MultiTrackLooper::default();
        let parameter_id = ParameterId::ParameterIdSource(SourceParameter::Speed);

        looper
            .handle
            .add_scene_parameter_lock(1, LooperId(0), parameter_id.clone(), 2.0);
        let value = looper
            .handle
            .scene_parameters
            .get_right(LooperId(0), parameter_id.clone())
            .unwrap();

        assert_eq!(value.clone(), ParameterValue::Float(2.0.into()));

        looper.handle.set_scene_value(0.5);
        assert_no_alloc(|| {
            looper.process_scenes();
        });
        assert_eq!(
            looper.parameters_scratch[0][looper.parameter_scratch_indexes[&parameter_id]],
            ParameterValue::Float(1.5.into())
        );
    }

    #[test]
    fn test_scenes_dont_allocate() {
        let mut looper = MultiTrackLooper::default();
        let handle = std::thread::spawn(move || {
            assert_no_alloc(|| {
                looper.process_scenes();
                looper.flush_parameters();
            });
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_scenes_are_respected() {
        let mut looper = MultiTrackLooper::default();

        looper.handle.set_scene_value(0.0);
        looper.handle.add_scene_parameter_lock(
            1,
            LooperId(0),
            ParameterId::ParameterIdSource(SourceParameter::Speed),
            2.0,
        );
        assert_no_alloc(|| {
            looper.process_scenes();
            looper.flush_parameters();
        });
        assert_f_eq!(looper.handle.voices[0].looper().speed(), 1.0);
        looper.handle.set_scene_value(0.5);
        assert_no_alloc(|| {
            looper.process_scenes();
        });
        assert_eq!(
            looper.parameters_scratch[0][looper.parameter_scratch_indexes
                [&ParameterId::ParameterIdSource(SourceParameter::Speed)]],
            ParameterValue::Float(1.5.into())
        );
        assert_no_alloc(|| {
            looper.flush_parameters();
        });
        assert_f_eq!(looper.handle.voices[0].looper().speed(), 1.5);
    }

    #[test]
    fn test_build_parameters_table() {
        use itertools::Itertools;
        let (_, parameters) = parameters::build_default_parameters();
        let start_index = parameters
            .iter()
            .cloned()
            .find_position(|id| *id == ParameterId::ParameterIdSource(SourceParameter::Start))
            .unwrap()
            .0;
        let slice_index = parameters
            .iter()
            .cloned()
            .find_position(|id| *id == ParameterId::ParameterIdSource(SourceParameter::SliceId))
            .unwrap()
            .0;
        assert!(slice_index > start_index);
    }

    #[test]
    fn test_process_doesnt_alloc() {
        let mut looper = MultiTrackLooper::new(Default::default(), 8);
        let settings = AudioProcessorSettings::default();
        let mut buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(1.0));

        looper.prepare(settings);
        let num_frames = buffer.len() / settings.block_size();
        for frame in 0..num_frames {
            let start_index = frame * settings.block_size();
            let end_index = start_index + settings.block_size();
            let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer[start_index..end_index]);
            looper.process(&mut buffer);
        }
    }

    #[test]
    fn test_processor_starts_silent() {
        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings::default());
        let mut buffer = VecAudioBuffer::empty_with(1, 4, 0.0);
        processor.process(&mut buffer);
        assert_eq!(buffer.slice(), [0.0, 0.0, 0.0, 0.0])
    }

    #[test]
    fn test_processor_will_playback_set_looper_buffer() {
        let mut processor = MultiTrackLooper::new(Default::default(), 1);
        processor.prepare(AudioProcessorSettings::default());

        let looper = processor.handle().voices()[0].looper().clone();
        let mut looper_buffer = VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]);
        looper.set_looper_buffer(&looper_buffer.interleaved());
        looper.play();

        assert_eq!(looper.state(), LooperState::Playing);
        let mut buffer = VecAudioBuffer::empty_with(1, 4, 0.0);
        processor.process(&mut buffer);
        assert_eq!(looper.state(), LooperState::Playing);
        assert_eq!(buffer.slice(), [1.0, 2.0, 3.0, 4.0])
    }

    #[test]
    fn test_we_can_set_start_on_a_looper() {
        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings::default());
        processor
            .handle()
            .set_source_parameter(LooperId(0), SourceParameter::Start, 0.5);
        let looper = processor.handle().voices()[0].looper();
        looper.set_looper_buffer(&VecAudioBuffer::from(vec![1.0, 2.0, 3.0, 4.0]).interleaved());
        looper.play();
        let value = processor
            .handle()
            .get_parameter(LooperId(0), &SourceParameter::Start.into())
            .unwrap()
            .as_float();
        assert_eq!(value, 0.5);
        let mut buffer = VecAudioBuffer::empty_with(1, 4, 0.0);
        processor.process(&mut buffer);
        assert_eq!(buffer.slice(), [3.0, 4.0, 3.0, 4.0])
    }

    #[test]
    fn test_set_scene_parameter_lock() {
        let mut looper = MultiTrackLooper::new(Default::default(), 8);

        let start_parameter = ParameterId::ParameterIdSource(SourceParameter::Start);
        looper
            .handle
            .set_source_parameter(LooperId(0), SourceParameter::Start, 0.3);
        looper
            .handle
            .add_scene_parameter_lock(0, LooperId(0), start_parameter, 0.8);
        looper.process_scenes();
    }

    #[test]
    fn test_map_lfo_to_pitch_modulation() {
        let settings = AudioProcessorSettings::default();
        let mut looper = MultiTrackLooper::new(Default::default(), 1);
        let looper_voice = looper.handle.voices()[0].looper().clone();

        looper.prepare(settings);

        let buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(10.0));
        let mut buffer = VecAudioBuffer::from(buffer);
        looper_voice.set_looper_buffer(&buffer.interleaved());
        looper_voice.play();

        let pitch_parameter = ParameterId::ParameterIdSource(SourceParameter::Pitch);
        looper
            .handle
            .add_lfo_mapping(LooperId(0), 0, pitch_parameter, 1.0);

        let num_blocks = buffer.num_samples() as usize / settings.block_size();
        let mut output = VecAudioBuffer::empty_with(1, settings.block_size(), 0.0);
        for _i in 0..num_blocks {
            looper.process(&mut output);
        }
    }

    #[test]
    fn test_starts_empty() {
        let looper = MultiTrackLooper::new(Default::default(), 8);
        assert_eq!(
            looper.handle.all_loopers_empty_other_than(LooperId(0)),
            true
        );
    }

    #[test]
    fn test_sending_midi_will_update_mapped_parameters() {
        let mut looper = MultiTrackLooper::default();
        let midi = looper.handle().midi();
        midi.midi_map().add(
            MidiControllerNumber::new(55),
            EntityId::EntityIdLooperParameter(LooperId(0), SourceParameter::Start.into()),
        );
        looper.process_midi_events(&[MidiMessageEntry(Owned::new(
            audio_garbage_collector::handle(),
            MidiMessageWrapper {
                message_data: [0b1011_0001, 55, 64],
                timestamp: 0,
            },
        ))]);
        assert!(
            (looper
                .handle()
                .get_parameter(LooperId(0), &SourceParameter::Start.into())
                .unwrap()
                .as_float())
            .abs()
                - 0.5
                < 0.01 // MIDI has ~0.008 step-size (range: 0-127, resolution: 1 / 127)
        );
    }
}
