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

use crate::processor::handle::{LooperState, ToggleRecordingResult};
use crate::{LooperOptions, QuantizeMode, TimeInfoProvider, TimeInfoProviderImpl};

pub(crate) mod allocator;
mod envelope_processor;
mod lfo_processor;
mod looper_voice;
pub(crate) mod metrics;
pub mod parameters;
pub(crate) mod slice_worker;
mod tempo_estimation;
mod trigger_model;

type SceneParametersRef = Shared<
    lockfree::map::Map<SceneId, lockfree::map::Map<(LooperId, ParameterId), ParameterValue>>,
>;

pub struct MultiTrackLooperHandle {
    voices: Vec<LooperVoice>,
    time_info_provider: Shared<TimeInfoProviderImpl>,
    sample_rate: AtomicF32,
    scene_value: AtomicF32,
    scene_parameters: SceneParametersRef,
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
            handle.looper().start_recording();
        }
    }

    pub fn set_scene_value(&self, value: f32) {
        self.scene_value.set(value);
    }

    pub fn toggle_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            let was_empty = self.all_loopers_empty_other_than(looper_id);
            if let ToggleRecordingResult::StoppedRecording = handle.looper().toggle_recording() {
                self.slice_worker.add_job(
                    looper_id.0,
                    *self.settings.get(),
                    handle.looper().looper_clip(),
                );

                let parameters = handle.parameters();
                let tempo_control = parameters.get(&ParameterId::ParameterIdQuantization(
                    QuantizationParameter::QuantizationParameterQuantizeMode,
                ));

                if was_empty
                    && tempo_control
                        .map(|tempo_control| {
                            *tempo_control.val()
                                == ParameterValue::Enum(
                                    TempoControl::TempoControlSetGlobalTempo.to_usize().unwrap(),
                                )
                        })
                        .unwrap_or(false)
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
            Self::update_parameter_table(voice, parameter_id, ParameterValue::Float(value));
        }
    }

    fn update_parameter_table(
        voice: &LooperVoice,
        parameter_id: ParameterId,
        value: ParameterValue,
    ) {
        let parameter_values = voice.parameters();
        let _ = parameter_values.insert(parameter_id, value);
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    fn update_handle_int(&self, voice: &LooperVoice, parameter_id: ParameterId, value: i32) {
        match parameter_id {
            ParameterId::ParameterIdSource(parameter) => match parameter {
                SourceParameter::SliceId => {
                    let slice_enabled = voice
                        .parameters()
                        .get(&ParameterId::ParameterIdSource(
                            SourceParameter::SliceEnabled,
                        ))
                        .map(|entry| {
                            if let ParameterValue::Bool(v) = entry.val() {
                                *v
                            } else {
                                false
                            }
                        })
                        .unwrap_or(false);

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
                ParameterValue::Float(value),
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
                ParameterValue::Enum(mode.to_usize().unwrap()),
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
                        voice.lfo1().frequency.set(value);
                    }
                    2 => {
                        voice.lfo2().frequency.set(value);
                    }
                    _ => {}
                },
                LFOParameter::Amount => match lfo {
                    1 => {
                        voice.lfo1().amount.set(value);
                    }
                    2 => {
                        voice.lfo2().amount.set(value);
                    }
                    _ => {}
                },
            }

            Self::update_parameter_table(
                voice,
                ParameterId::ParameterIdLFO(lfo, parameter_id),
                ParameterValue::Float(value),
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

            let parameters = voice.parameters();
            let _ = parameters.insert(parameter_id.clone(), ParameterValue::Bool(value));
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
        let all_scene_parameters = &self.scene_parameters;
        if let Some(scene_parameters) = all_scene_parameters.get(&scene_id) {
            scene_parameters
                .val()
                .insert((looper_id, parameter_id), ParameterValue::Float(value));
        }
    }

    pub fn add_parameter_lock(
        &self,
        looper_id: LooperId,
        position_beats: usize,
        parameter_id: ParameterId,
        value: f32,
    ) {
        self.voices[looper_id.0]
            .triggers()
            .add_lock(position_beats, parameter_id, value);
    }

    pub fn toggle_trigger(&self, looper_id: LooperId, position_beats: usize) {
        self.voices[looper_id.0]
            .triggers()
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
            handle.looper().toggle_playback();
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
}

pub struct MultiTrackLooper {
    graph: AudioProcessorGraph,
    handle: Shared<MultiTrackLooperHandle>,
    step_trackers: Vec<StepTracker>,
    metrics: AudioProcessorMetrics,
}

impl MultiTrackLooper {
    pub fn new(options: LooperOptions, num_voices: usize) -> Self {
        let time_info_provider = make_shared(TimeInfoProviderImpl::new(options.host_callback));
        let processors: Vec<VoiceProcessors> = (0..num_voices)
            .map(|_| looper_voice::build_voice_processor(&options, &time_info_provider))
            .collect();

        let metronome = metronome::MetronomeProcessor::new();
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);
        metronome_handle.set_volume(0.7);

        let voices = processors
            .iter()
            .enumerate()
            .map(|(i, voice_processors)| looper_voice::build_voice_handle(i, voice_processors))
            .collect();
        let metrics = AudioProcessorMetrics::default();
        let handle = make_shared(MultiTrackLooperHandle {
            voices,
            time_info_provider,
            scene_value: AtomicF32::new(0.0),
            scene_parameters: make_shared(Default::default()),
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
                let triggers = voice.triggers();

                // let parameter_values = voice.parameter_values.get();
                // for (parameter_id, parameter_value) in parameter_values.deref().iter() {
                //     self.handle
                //         .update_handle(voice, parameter_id.clone(), parameter_value.value);
                // }

                if let Some(_trigger) =
                    find_current_beat_trigger(triggers, step_tracker, position_beats)
                {
                    voice.looper().trigger();
                    voice.envelope().adsr_envelope.note_on();
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
        let all_scene_parameters = &self.handle.scene_parameters;
        let left_parameters = all_scene_parameters.get(&self.handle.left_scene_id.get());
        let right_parameters = all_scene_parameters.get(&self.handle.right_scene_id.get());
        let scene_value = self.handle.scene_value.get();

        for (index, voice) in self.handle.voices.iter().enumerate() {
            let looper_id = LooperId(index);
            let parameter_values = voice.parameters();

            for parameter_id in voice.parameter_ids() {
                if let Some(parameter_value) = parameter_values.get(parameter_id) {
                    let parameter_value = parameter_value.val();

                    let key = (looper_id, parameter_id.clone());
                    let left_value = left_parameters
                        .as_ref()
                        .and_then(|ps| ps.val().get(&key).map(|entry| entry.val().clone()))
                        .unwrap_or_else(|| parameter_value.clone());
                    let right_value = right_parameters
                        .as_ref()
                        .and_then(|ps| ps.val().get(&key).map(|entry| entry.val().clone()))
                        .unwrap_or_else(|| parameter_value.clone());

                    match (left_value, right_value) {
                        (ParameterValue::Float(left_value), ParameterValue::Float(right_value)) => {
                            let value = left_value + scene_value * (right_value - left_value);
                            self.handle
                                .update_handle_float(voice, parameter_id.clone(), value);
                        }
                        (ParameterValue::Int(left_value), ParameterValue::Int(right_value)) => {
                            let value = left_value
                                + (scene_value * (right_value - left_value) as f32) as i32;
                            self.handle
                                .update_handle_int(voice, parameter_id.clone(), value);
                        }
                        _ => {}
                    }
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
        assert_no_alloc(|| {
            self.metrics.on_process_start();

            self.process_scenes();
            self.process_triggers();

            self.graph.process(data);

            for _sample in data.frames() {
                self.handle.time_info_provider.tick();
            }

            self.metrics.on_process_end();
        });
    }
}

impl MidiEventHandler for MultiTrackLooper {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_traits::InterleavedAudioBuffer;

    use super::*;

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
    fn test_starts_empty() {
        let looper = MultiTrackLooper::new(Default::default(), 8);
        assert_eq!(
            looper.handle.all_loopers_empty_other_than(LooperId(0)),
            true
        );
    }
}
