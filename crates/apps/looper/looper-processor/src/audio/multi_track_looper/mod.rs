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

//! This module provides a `MultiTrackLooperProcessor`, which can sequence and loop 8 looper tracks
//! with shared tempo.
use std::convert::TryFrom;

use assert_no_alloc::assert_no_alloc;
use rustc_hash::FxHashMap as HashMap;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_metronome::MetronomeProcessor;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use augmented_atomics::AtomicValue;
use augmented_oscillator::Oscillator;

use crate::audio::time_info_provider::TimeInfoMetronomePlayhead;
use crate::parameters::LFOMode;
use crate::{LooperOptions, TimeInfoProvider, TimeInfoProviderImpl};

pub use self::handle::MultiTrackLooperHandle;
use self::lfo_processor::LFOHandle;
use self::looper_voice::{LooperVoice, VoiceProcessors};
use self::metrics::audio_processor_metrics::AudioProcessorMetrics;
use self::midi_button::{MIDIButton, MIDIButtonEvent};
use self::midi_store::MidiStoreHandle;
use self::parameters::{LFOParameter, LooperId, ParameterId, ParameterValue};
pub use self::parameters_map::ParametersMap;
use self::trigger_model::step_tracker::StepTracker;
use self::trigger_model::{find_current_beat_trigger, find_running_beat_trigger};

pub(crate) mod allocator;
mod copy_paste;
pub(crate) mod effects_processor;
mod envelope_processor;
mod handle;
pub(crate) mod lfo_processor;
mod long_backoff;
pub(crate) mod looper_voice;
pub(crate) mod metrics;
mod midi_button;
pub(crate) mod midi_store;
pub mod parameters;
mod parameters_map;
pub(crate) mod scene_state;
pub(crate) mod slice_worker;
mod tempo_estimation;
pub(crate) mod trigger_model;

/// During audio-processing, parameters are written into this storage to apply different phases
/// (user-settings, scenes, triggers & LFOs; in this order). This is temporary, per-callback
/// mutable state.
/// TODO: This should be a struct rather than two type-aliases
type ParametersScratch = Vec<Vec<ParameterValue>>;
type ParametersScratchIndexes = HashMap<ParameterId, usize>;

#[cfg_attr(doc, aquamarine::aquamarine)]
///
/// The following is a diagram of how things are connected:
/// ```mermaid
/// graph TD
///    A{Input}
///
///    A-->E[Looper 1]
///     -->G[Pitch-shifter 1]
///     -->F[Envelope 1]
///     -->H[Effects 1]
///     -->X{Gain 1}
///     -->I{Output}
///
///    A -->M[Looper 2]
///     -->K[Pitch-shifter 2]
///     -->J[Envelope 2]
///     -->L[Effects 2]
///     -->Y{Gain 2}
///     -->I{Output}
///    A -->O[Looper ...]
///     -->P[Pitch-shifter ...]
///     -->Q[Envelope ...]
///     -->R[Effects ...]
///     -->S{Gain ...}
///     -->I{Output}
/// ```
pub struct MultiTrackLooper {
    graph: AudioProcessorGraph,
    handle: Shared<MultiTrackLooperHandle>,
    step_trackers: Vec<StepTracker>,
    lfos: Vec<(Oscillator<f32>, Oscillator<f32>)>,
    metrics: AudioProcessorMetrics,
    parameters_scratch: ParametersScratch,
    parameter_scratch_indexes: ParametersScratchIndexes,
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

        let metronome =
            MetronomeProcessor::new(TimeInfoMetronomePlayhead(time_info_provider.clone()));
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);
        metronome_handle.set_volume(0.7);

        let (processors, voices) =
            Self::build_voices(&options, num_voices, &time_info_provider, None);

        let (parameters_scratch, parameter_scratch_indexes) =
            Self::make_parameters_scratch(&voices);

        let metrics = AudioProcessorMetrics::default();
        let handle = make_shared(MultiTrackLooperHandle::new(
            time_info_provider,
            metronome_handle,
            &metrics,
            voices,
        ));

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

    fn make_parameters_scratch(
        voices: &[LooperVoice],
    ) -> (ParametersScratch, ParametersScratchIndexes) {
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
        (parameters_scratch, parameter_scratch_indexes)
    }

    pub fn from_handle(
        options: LooperOptions,
        num_voices: usize,
        handle: Shared<MultiTrackLooperHandle>,
    ) -> Self {
        let metronome = MetronomeProcessor::from_handle(
            TimeInfoMetronomePlayhead(handle.time_info_provider().clone()),
            handle.metronome_handle().clone(),
        );
        let (processors, voices) = Self::build_voices(
            &options,
            num_voices,
            handle.time_info_provider(),
            Some(handle.voices()),
        );
        let (parameters_scratch, parameter_scratch_indexes) =
            Self::make_parameters_scratch(&voices);
        let metrics = AudioProcessorMetrics::from_handle(handle.metrics_handle().clone());
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
            lfos,
            metrics,
            parameters_scratch,
            parameter_scratch_indexes,
            record_midi_button: MIDIButton::new(),
        }
    }

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }

    fn build_voices(
        options: &LooperOptions,
        num_voices: usize,
        time_info_provider: &Shared<TimeInfoProviderImpl>,
        previous_handles: Option<&[LooperVoice]>,
    ) -> (Vec<VoiceProcessors>, Vec<LooperVoice>) {
        let processors: Vec<VoiceProcessors> = match previous_handles {
            None => (0..num_voices)
                .map(|_| looper_voice::build_voice_processor(options, time_info_provider))
                .collect(),
            Some(previous_handles) => previous_handles
                .iter()
                .map(looper_voice::from_handle)
                .collect(),
        };

        let voices: Vec<LooperVoice> = processors
            .iter()
            .enumerate()
            .map(|(i, voice_processors)| looper_voice::build_voice_handle(i, voice_processors))
            .collect();
        (processors, voices)
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
            effects_processor,
        } in processors
        {
            let looper_idx = graph.add_node(NodeType::Buffer(Box::new(looper)));
            let pitch_shifter_idx = graph.add_node(NodeType::Buffer(Box::new(pitch_shifter)));
            let envelope_idx = graph.add_node(NodeType::Buffer(Box::new(envelope)));
            let effects_idx = graph.add_node(NodeType::Buffer(Box::new(effects_processor)));

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
                .add_connection(envelope_idx, effects_idx)
                .expect("Shouldn't produce loop");
            graph
                .add_connection(effects_idx, graph.output())
                .expect("Shouldn't produce loop");
        }

        graph
    }
}

// Sequencer parameter-locks handling
impl MultiTrackLooper {
    fn process_triggers(&mut self) {
        if let Some(position_beats) = self
            .handle
            .time_info_provider()
            .get_time_info()
            .position_beats()
        {
            let step_trackers = &mut self.step_trackers;
            let parameters_scratch = &mut self.parameters_scratch;
            let parameters_scratch_indexes = &self.parameter_scratch_indexes;

            for (voice, step_tracker) in self.handle.voices().iter().zip(step_trackers) {
                Self::process_triggers_for_voice(
                    parameters_scratch_indexes,
                    parameters_scratch,
                    voice,
                    step_tracker,
                    position_beats,
                )
            }
        }
    }

    fn process_triggers_for_voice(
        parameters_scratch_indexes: &ParametersScratchIndexes,
        parameters_scratch: &mut ParametersScratch,
        voice: &LooperVoice,
        step_tracker: &mut StepTracker,
        position_beats: f64,
    ) {
        let triggers = voice.trigger_model();

        let triggers_vec = triggers.triggers();
        if let Some(_trigger) =
            find_current_beat_trigger(triggers, &triggers_vec, step_tracker, position_beats)
        {
            voice.looper().trigger();
            voice.envelope().adsr_envelope.note_on();
        }

        let index = voice.id;
        let parameters_scratch = &mut parameters_scratch[index];

        let triggers = find_running_beat_trigger(triggers, &triggers_vec, position_beats);
        let mut has_triggers = false;
        for trigger in triggers {
            has_triggers = true;
            for (parameter_id, lock) in trigger.locks() {
                let parameter_idx = parameters_scratch_indexes[parameter_id];
                parameters_scratch[parameter_idx] = ParameterValue::Float(lock.value().into());
            }
        }

        if !has_triggers {
            voice.envelope().adsr_envelope.note_off();
        }
    }
}

// LFOs handling
impl MultiTrackLooper {
    fn process_lfos(&mut self) {
        let parameters_scratch = &mut self.parameters_scratch;
        let parameters_scratch_indexes = &self.parameter_scratch_indexes;
        for ((lfo1, lfo2), voice) in self.lfos.iter_mut().zip(self.handle.voices().iter()) {
            Self::process_lfos_for_voice(
                parameters_scratch_indexes,
                parameters_scratch,
                &mut [(lfo1, voice.lfo1()), (lfo2, voice.lfo2())],
                &voice,
            )
        }
    }

    fn process_lfos_for_voice(
        parameter_scratch_indexes: &ParametersScratchIndexes,
        parameters_scratch: &mut ParametersScratch,
        lfos: &mut [(&mut Oscillator<f32>, &LFOHandle)],
        voice: &&LooperVoice,
    ) {
        for (lfo_index, (lfo_osc, lfo_handle)) in lfos.iter_mut().enumerate() {
            let freq_idx =
                ParameterId::ParameterIdLFO(lfo_index, LFOParameter::LFOParameterFrequency);
            let freq_idx = parameter_scratch_indexes[&freq_idx];
            let amount_idx =
                ParameterId::ParameterIdLFO(lfo_index, LFOParameter::LFOParameterAmount);
            let amount_idx = parameter_scratch_indexes[&amount_idx];
            let lfo_mode_idx =
                ParameterId::ParameterIdLFO(lfo_index, LFOParameter::LFOParameterMode);
            let lfo_mode_idx = parameter_scratch_indexes[&lfo_mode_idx];

            let scratch = &mut parameters_scratch[voice.id];

            let freq = scratch[freq_idx].as_float();
            lfo_osc.set_frequency(freq);
            let lfo_mode: LFOMode =
                LFOMode::try_from(scratch[lfo_mode_idx].as_enum()).unwrap_or(LFOMode::LFOModeSine);
            lfo_osc.set_generator(lfo_mode.generator_fn());

            let global_amount = scratch[amount_idx].as_float();

            for (parameter_idx, parameter) in voice.parameter_ids().iter().enumerate() {
                let modulation_amount = lfo_handle.modulation_amount(parameter);
                let value = &scratch[parameter_idx];
                if let ParameterValue::Float(value) = value {
                    let value = lfo_osc.get() * modulation_amount * global_amount + value.get();
                    scratch[parameter_idx] = ParameterValue::Float(value.into());
                }
            }
        }
    }

    /// Not the best implementation as LFOs are ticked only once per callback.
    ///
    /// This means resolution of the LFOs is very low (as they only update on whatever
    /// (buffer-size / sample-rate) minimum. For example, ~10ms for 512 samples buffer.
    fn tick_lfos(&mut self, num_samples: f32) {
        for (l1, l2) in self.lfos.iter_mut() {
            l1.tick_n(num_samples);
            l2.tick_n(num_samples);
        }
    }
}

// Scenes handling
impl MultiTrackLooper {
    // Public for benchmarking
    pub fn process_scenes(&mut self) {
        let scene_value = self.handle.scene_handle().get_slider();

        for (index, voice) in self.handle.voices().iter().enumerate() {
            let looper_id = LooperId(index);
            let parameter_values = voice.user_parameters();

            for (parameter_idx, parameter_id) in voice.parameter_ids().iter().enumerate() {
                let parameter_slot = &mut self.parameters_scratch[voice.id][parameter_idx];
                let parameter_value = parameter_values.get(parameter_id.clone());

                let _key = (looper_id, parameter_id.clone());
                let left_value = self
                    .handle
                    .scene_handle()
                    .get_left(looper_id, parameter_id.clone())
                    .unwrap_or(parameter_value);
                let right_value = self
                    .handle
                    .scene_handle()
                    .get_right(looper_id, parameter_id.clone())
                    .unwrap_or(parameter_value);

                *parameter_slot = match (left_value, right_value) {
                    (ParameterValue::Float(left_value), ParameterValue::Float(right_value)) => {
                        let value =
                            left_value.get() + scene_value * (right_value.get() - left_value.get());

                        ParameterValue::Float(value.into())
                    }
                    (ParameterValue::Int(left_value), ParameterValue::Int(right_value)) => {
                        let value = left_value.get()
                            + (scene_value * (right_value.get() - left_value.get()) as f32) as i32;

                        ParameterValue::Int(value.into())
                    }
                    (other_value, _) => other_value.clone(),
                };
            }
        }
    }
}

// Flush parameters;
// * Each stage (triggers, lfos, scenes, etc) makes changes into a temporary parameters
//   scratch-space
// * After all stages, we flush the scratch space into the processor handles (actually set the
//   parameters)
impl MultiTrackLooper {
    fn flush_parameters(&mut self) {
        for (voice, values) in self
            .handle
            .voices()
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
}

// MIDI record button handling
impl MultiTrackLooper {
    fn on_record_button_click_midi(&mut self, value: u8) {
        let event = self.record_midi_button.accept(value);
        self.handle_record_midi_button_event(event);
    }

    fn handle_record_midi_button_event(&self, event: Option<MIDIButtonEvent>) {
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
        self.handle.metrics_handle().prepare(settings);
        self.handle.set_settings(make_shared(settings));

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

            // Ideally this wouldn't be in bulk. Perhaps this is the wrong approach and we should
            // be just reading time from somewhere
            self.tick_lfos(data.num_samples() as f32);
            self.handle
                .time_info_provider()
                .tick_n(data.num_samples() as u32);

            self.metrics.on_process_end();
        });
    }
}

impl MidiEventHandler for MultiTrackLooper {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        let midi_store = self.handle.midi().clone();
        MidiStoreHandle::process_midi_events(&midi_store, midi_messages, self);

        let event = self.record_midi_button.tick();
        self.handle_record_midi_button_event(event);
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::{assert_f_eq, sine_buffer};
    use basedrop::Owned;

    use audio_processor_standalone_midi::host::{MidiMessageEntry, MidiMessageWrapper};
    use audio_processor_traits::{InterleavedAudioBuffer, VecAudioBuffer};

    use crate::audio::midi_map::MidiControllerNumber;
    use crate::audio::multi_track_looper::parameters::EntityId;
    use crate::audio::processor::handle::LooperState;
    use crate::parameters::SourceParameter;
    use crate::LooperHandleThread;

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
            .scene_handle()
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
    fn test_user_parameters_are_respected() {
        let mut looper = MultiTrackLooper::default();

        looper
            .handle
            .set_source_parameter(LooperId(0), SourceParameter::Speed, 2.0);
        looper.process_scenes();
        looper.process_triggers();
        looper.process_lfos();
        looper.flush_parameters();
        assert_f_eq!(looper.handle.voices()[0].looper().speed(), 2.0);
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
        assert_f_eq!(looper.handle.voices()[0].looper().speed(), 1.0);
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
        assert_f_eq!(looper.handle.voices()[0].looper().speed(), 1.5);
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
        wisual_logger::init_from_env();
        let mut looper = MultiTrackLooper::new(Default::default(), 8);
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 100.0;
        let mut buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(1.0));

        looper.prepare(settings);
        let num_frames = buffer.len() / settings.block_size();
        log::info!("Processing num_frames={}", num_frames);
        for frame in 0..num_frames {
            let start_index = frame * settings.block_size();
            let end_index = start_index + settings.block_size();
            let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer[start_index..end_index]);
            looper.process(&mut buffer);
        }
    }

    #[test]
    fn test_processor_starts_silent() {
        wisual_logger::init_from_env();
        let mut processor = MultiTrackLooper::default();
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 100.0;
        processor.prepare(settings);
        let mut buffer = VecAudioBuffer::empty_with(1, 4, 0.0);
        processor.process(&mut buffer);
        assert_eq!(buffer.slice(), [0.0, 0.0, 0.0, 0.0])
    }

    #[test]
    fn test_processor_will_playback_set_looper_buffer() {
        let mut processor = MultiTrackLooper::new(Default::default(), 1);
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 100.0;
        processor.prepare(settings);

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
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 100.0;
        processor.prepare(settings);
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
    fn test_record_into_single_looper() {
        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings {
            sample_rate: 10.0, // 10 samples per sec
            ..AudioProcessorSettings::default()
        });
        // "4 seconds" of audio;
        let num_samples = 40;

        processor
            .handle()
            .toggle_recording(LooperId(0), LooperHandleThread::OtherThread);
        // Record 0..40 into the first looper
        let mut buffer = range_buffer(num_samples);
        processor.process(&mut buffer);

        // Stop recording and check output
        processor
            .handle()
            .toggle_recording(LooperId(0), LooperHandleThread::OtherThread);
        let mut buffer = VecAudioBuffer::empty_with(1, num_samples, 0.0);
        processor.process(&mut buffer);

        assert_eq!(
            buffer.slice(),
            &(0..num_samples)
                .into_iter()
                .map(|i| i as f32)
                .collect::<Vec<f32>>()
        );
        // Check internal state
        assert_eq!(
            processor
                .handle()
                .time_info_provider()
                .get_time_info()
                .is_playing(),
            true
        );
        assert_eq!(
            processor
                .handle()
                .time_info_provider()
                .get_time_info()
                .tempo(),
            Some(120.0)
        );
    }

    #[test]
    fn test_record_into_two_synced_loopers() {
        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings {
            sample_rate: 10.0, // 10 samples per sec
            ..AudioProcessorSettings::default()
        });
        // "4 seconds" of audio; this is going to be 2 bars at 120 bpm
        // each beat is 5 samples, each bar is 20 samples
        let num_samples = 40;

        processor
            .handle()
            .toggle_recording(LooperId(0), LooperHandleThread::OtherThread);
        // Record 0..40 into the first looper
        let mut buffer = range_buffer(num_samples);
        processor.process(&mut buffer);

        // Stop recording
        processor
            .handle()
            .toggle_recording(LooperId(0), LooperHandleThread::OtherThread);

        // Advance by 10 samples
        let mut buffer = VecAudioBuffer::empty_with(1, 10, 0.0);
        processor.process(&mut buffer);

        // Trigger recording second looper
        processor
            .handle()
            .toggle_recording(LooperId(1), LooperHandleThread::OtherThread);
        assert_eq!(
            processor.handle().get_looper_state(LooperId(1)),
            LooperState::RecordingScheduled
        );

        // Tick 10 samples
        for _i in 0..10 {
            assert_eq!(
                processor.handle().get_looper_state(LooperId(1)),
                LooperState::RecordingScheduled
            );
            let mut buffer = VecAudioBuffer::empty_with(1, 1, 0.0);
            processor.process(&mut buffer);
        }

        assert_eq!(
            processor.handle().get_looper_state(LooperId(1)),
            LooperState::Recording
        );

        // Record 30 samples in
        let mut buffer = range_buffer(30);
        processor.process(&mut buffer);
        processor
            .handle()
            .toggle_recording(LooperId(1), LooperHandleThread::OtherThread);

        // Advance 10 samples
        for i in 0..10 {
            assert_eq!(
                processor.handle().get_looper_state(LooperId(1)),
                LooperState::PlayingScheduled
            );
            let mut buffer = VecAudioBuffer::empty_with(1, 1, 0.0);
            buffer.set(0, 0, i as f32);
            processor.process(&mut buffer);
        }

        assert_eq!(
            processor.handle().get_looper_state(LooperId(1)),
            LooperState::Playing
        );

        // Test output; mute looper 0
        processor.handle().set_volume(LooperId(0), 0.0);
        let mut buffer = VecAudioBuffer::empty_with(1, 40, 0.0);
        processor.process(&mut buffer);
        assert_eq!(
            &buffer
                .slice()
                .iter()
                .map(|f| (f * 10.0) as usize)
                // TODO - This is broken; the clip should be 40 samples long
                .take(39)
                .collect::<Vec<usize>>(),
            &(0..30)
                .into_iter()
                .chain((0..9).into_iter())
                .map(|i| i as f32)
                .map(|f| (f * 10.0) as usize)
                .collect::<Vec<usize>>()
        );
    }

    fn range_buffer(num_samples: usize) -> VecAudioBuffer<f32> {
        let mut buffer = VecAudioBuffer::empty_with(1, num_samples, 0.0);
        for i in 0..num_samples {
            buffer.set(0, i, i as f32);
        }
        buffer
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
    fn test_slicing_sets_offset() {
        let mut looper = MultiTrackLooper::new(Default::default(), 1);
        let slice_parameter = ParameterId::ParameterIdSource(SourceParameter::SliceId);
        looper
            .handle()
            .set_int_parameter(LooperId(0), slice_parameter.clone(), 2);
        let parameter_value = looper
            .handle()
            .get_parameter(LooperId(0), &slice_parameter)
            .unwrap();
        let parameter: i32 = parameter_value.as_int();
        assert_eq!(parameter, 2);

        looper.process_scenes();
        looper.process_lfos();
        looper.process_triggers();
        let parameter = &looper.parameters_scratch[0][looper.parameter_scratch_indexes
            [&ParameterId::ParameterIdSource(SourceParameter::SliceId)]];
        let parameter: i32 = parameter.as_int();
        assert_eq!(parameter, 2);
    }

    #[test]
    fn test_map_lfo_to_pitch_modulation() {
        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 100.0;
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
        assert!(looper.handle.all_loopers_empty_other_than(LooperId(0)));
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
