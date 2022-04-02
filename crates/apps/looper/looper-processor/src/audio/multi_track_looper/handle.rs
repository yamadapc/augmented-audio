use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use atomic_refcell::AtomicRefCell;
use basedrop::SharedCell;
use num::ToPrimitive;

use super::looper_voice::LooperVoice;
use super::metrics::audio_processor_metrics::{AudioProcessorMetrics, AudioProcessorMetricsHandle};
use super::parameters::{
    CQuantizeMode, EnvelopeParameter, LFOParameter, LooperId, ParameterId, ParameterValue,
    QuantizationParameter, SceneId, SourceParameter, TempoControl,
};

use super::slice_worker::{SliceResult, SliceWorker};
use super::tempo_estimation::estimate_tempo;

use audio_garbage_collector::{make_shared, make_shared_cell, Shared};

use audio_processor_traits::{AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::{AtomicF32, AtomicValue};

use metronome::MetronomeProcessorHandle;

use crate::audio::multi_track_looper::midi_store::MidiStoreHandle;
use crate::audio::multi_track_looper::scene_state::SceneHandle;
use crate::audio::processor::handle::{LooperHandleThread, LooperState, ToggleRecordingResult};

use crate::{QuantizeMode, TimeInfoProvider, TimeInfoProviderImpl};

pub struct MultiTrackLooperHandle {
    voices: Vec<LooperVoice>,
    time_info_provider: Shared<TimeInfoProviderImpl>,
    scene_handle: SceneHandle,
    metronome_handle: Shared<MetronomeProcessorHandle>,
    slice_worker: SliceWorker,
    settings: SharedCell<AudioProcessorSettings>,
    metrics_handle: Shared<AudioProcessorMetricsHandle>,
    midi_store: Shared<MidiStoreHandle>,
    active_looper: AtomicUsize,
}

impl MultiTrackLooperHandle {
    pub fn new(
        time_info_provider: Shared<TimeInfoProviderImpl>,
        metronome_handle: Shared<MetronomeProcessorHandle>,
        metrics: &AudioProcessorMetrics,
        voices: Vec<LooperVoice>,
    ) -> Self {
        MultiTrackLooperHandle {
            voices,
            time_info_provider,
            scene_handle: SceneHandle::new(8, 2),
            metronome_handle,
            settings: make_shared_cell(AudioProcessorSettings::default()),
            slice_worker: SliceWorker::new(),
            metrics_handle: metrics.handle(),
            midi_store: make_shared(super::midi_store::MidiStoreHandle::default()),
            active_looper: AtomicUsize::new(0),
        }
    }

    pub fn get_parameter(&self, looper_id: LooperId, id: &ParameterId) -> Option<ParameterValue> {
        self.voices
            .get(looper_id.0)
            .map(|voice| voice.user_parameters().get(id.clone()).clone())
    }

    pub fn set_parameter(&self, looper_id: LooperId, id: ParameterId, value: ParameterValue) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            Self::update_parameter_table(voice, id, value);
        }
    }

    pub fn start_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper().start_recording();
        }
    }

    pub fn set_scene_value(&self, value: f32) {
        self.scene_handle.set_slider(value);
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
                    let settings = self.settings.get();
                    let estimated_tempo = estimate_tempo(
                        Default::default(),
                        settings.sample_rate(),
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
    pub fn update_handle_int(&self, voice: &LooperVoice, parameter_id: ParameterId, value: i32) {
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

    pub fn update_handle_float(&self, voice: &LooperVoice, parameter_id: ParameterId, value: f32) {
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
        let all_scene_parameters = &self.scene_handle;
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
        let all_scene_parameters = &self.scene_handle;
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

    pub fn all_loopers_empty_other_than(&self, looper_id: LooperId) -> bool {
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

    pub(crate) fn scene_handle(&self) -> &SceneHandle {
        &self.scene_handle
    }

    pub fn settings(&self) -> Shared<AudioProcessorSettings> {
        self.settings.get()
    }

    pub fn set_settings(&self, settings: Shared<AudioProcessorSettings>) {
        self.settings.set(settings)
    }
}
