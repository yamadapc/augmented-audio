use std::collections::HashMap;

use assert_no_alloc::assert_no_alloc;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use augmented_atomics::AtomicValue;
use augmented_oscillator::Oscillator;
use metronome::MetronomeProcessor;

use crate::audio::time_info_provider::TimeInfoMetronomePlayhead;
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

        let metronome = metronome::MetronomeProcessor::new(TimeInfoMetronomePlayhead(
            time_info_provider.clone(),
        ));
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);
        metronome_handle.set_volume(0.7);

        let (processors, voices) = Self::build_voices(&options, num_voices, &time_info_provider);

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

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }

    fn build_voices(
        options: &LooperOptions,
        num_voices: usize,
        time_info_provider: &Shared<TimeInfoProviderImpl>,
    ) -> (Vec<VoiceProcessors>, Vec<LooperVoice>) {
        let processors: Vec<VoiceProcessors> = (0..num_voices)
            .map(|_| looper_voice::build_voice_processor(&options, &time_info_provider))
            .collect();

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
                &mut [(lfo1, &voice.lfo1()), (lfo2, &voice.lfo2())],
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
            let freq_idx = ParameterId::ParameterIdLFO(lfo_index, LFOParameter::Frequency);
            let amount_idx = ParameterId::ParameterIdLFO(lfo_index, LFOParameter::Amount);

            let scratch = &mut parameters_scratch[voice.id];

            let freq = scratch[parameter_scratch_indexes[&freq_idx]].as_float();
            lfo_osc.set_frequency(freq);
            let global_amount = scratch[parameter_scratch_indexes[&amount_idx]].as_float();

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

    fn tick_lfos(&mut self) {
        for (l1, l2) in self.lfos.iter_mut() {
            l1.tick();
            l2.tick();
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

            for parameter_id in voice.parameter_ids() {
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
                    (other_value, _) => {
                        self.parameters_scratch[voice.id]
                            [self.parameter_scratch_indexes[parameter_id]] = other_value.clone();
                    }
                }
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

            for _sample in data.frames() {
                self.handle.time_info_provider().tick();
                self.tick_lfos();
            }

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
