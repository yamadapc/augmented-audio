use atomic_refcell::{AtomicRef, AtomicRefCell};
use num::iter::range_step_from;
use std::sync::atomic::Ordering;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_graph::{AudioProcessorGraph, NodeType};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
    VecAudioBuffer,
};
use augmented_atomics::AtomicF32;
use metronome::MetronomeProcessorHandle;

use crate::processor::handle::{LooperState, ToggleRecordingResult};
use crate::tempo_estimation::estimate_tempo;
use crate::trigger_model::step_tracker::StepTracker;
use crate::trigger_model::{find_current_beat_trigger, TrackTriggerModel, Trigger};
use crate::{
    LoopSequencerProcessorHandle, LooperOptions, LooperProcessor, LooperProcessorHandle,
    QuantizeMode, TimeInfoProvider, TimeInfoProviderImpl,
};

pub struct LooperId(pub usize);

#[repr(C)]
#[no_mangle]
pub enum SourceParameter {
    Start = 0,
    End = 1,
    FadeStart = 2,
    FadeEnd = 3,
    Pitch = 4,
    Speed = 5,
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
    triggers: Shared<TrackTriggerModel>,
    looper_handle: Shared<LooperProcessorHandle>,
    sequencer_handle: Shared<LoopSequencerProcessorHandle>,
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
    metronome_handle: Shared<MetronomeProcessorHandle>,
}

impl MultiTrackLooperHandle {
    pub fn start_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            handle.looper_handle.start_recording();
        }
    }

    pub fn toggle_recording(&self, looper_id: LooperId) {
        if let Some(handle) = self.voices.get(looper_id.0) {
            let was_empty = self.all_loopers_empty_other_than(looper_id);
            match handle.looper_handle.toggle_recording() {
                ToggleRecordingResult::StoppedRecording => {
                    if was_empty {
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
                SourceParameter::Pitch => {}
                SourceParameter::Speed => {
                    voice.looper_handle.set_speed(value);
                }
            }
        }
    }

    pub fn set_lfo_frequency(&self, looper_id: LooperId, lfo: usize, value: f32) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match lfo {
                1 => {
                    voice.lfo1_handle.frequency.set(value);
                }
                2 => {
                    voice.lfo2_handle.frequency.set(value);
                }
                _ => {}
            }
        }
    }

    pub fn set_lfo_amount(&self, looper_id: LooperId, lfo: usize, value: f32) {
        if let Some(voice) = self.voices.get(looper_id.0) {
            match lfo {
                1 => {
                    voice.lfo1_handle.amount.set(value);
                }
                2 => {
                    voice.lfo2_handle.amount.set(value);
                }
                _ => {}
            }
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
}

pub struct MultiTrackLooper {
    graph: AudioProcessorGraph,
    handle: Shared<MultiTrackLooperHandle>,
    step_trackers: Vec<StepTracker>,
}

impl MultiTrackLooper {
    pub fn new(options: LooperOptions, num_voices: usize) -> Self {
        let time_info_provider = make_shared(TimeInfoProviderImpl::new(options.host_callback));
        let voices: Vec<LooperProcessor> = (0..num_voices)
            .map(|_| {
                let voice = LooperProcessor::new(options.clone(), time_info_provider.clone());
                voice
                    .handle()
                    .quantize_options()
                    .set_mode(QuantizeMode::SnapNext);
                voice.handle().tick_time.store(false, Ordering::Relaxed);
                voice
            })
            .collect();

        let metronome = metronome::MetronomeProcessor::new();
        let metronome_handle = metronome.handle().clone();
        metronome_handle.set_is_playing(false);

        let handle = make_shared(MultiTrackLooperHandle {
            voices: voices
                .iter()
                .map(|voice| {
                    let looper_handle = voice.handle().clone();
                    let sequencer_handle = voice.sequencer_handle().clone();
                    let triggers = make_shared(TrackTriggerModel::default());

                    LooperVoice {
                        looper_handle,
                        sequencer_handle,
                        triggers,
                        lfo1_handle: make_shared(LFOHandle::default()),
                        lfo2_handle: make_shared(LFOHandle::default()),
                    }
                })
                .collect(),
            time_info_provider,
            sample_rate: AtomicF32::new(44100.0),
            metronome_handle,
        });

        let mut graph = AudioProcessorGraph::default();
        let input_node = graph.input();
        let output_node = graph.output();

        let metronome_idx = graph.add_node(NodeType::Buffer(Box::new(metronome)));
        graph.add_connection(input_node, metronome_idx);
        graph.add_connection(metronome_idx, output_node);

        let step_trackers = voices.iter().map(|_| StepTracker::default()).collect();

        for voice in voices {
            let voice_idx = graph.add_node(NodeType::Simple(Box::new(voice)));
            graph.add_connection(input_node, voice_idx);
            graph.add_connection(voice_idx, output_node);
        }

        Self {
            graph,
            handle,
            step_trackers,
        }
    }

    pub fn handle(&self) -> &Shared<MultiTrackLooperHandle> {
        &self.handle
    }
}

impl AudioProcessor for MultiTrackLooper {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.graph.prepare(settings);
        self.handle.sample_rate.set(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if let Some(position_beats) = self
            .handle
            .time_info_provider
            .get_time_info()
            .position_beats()
        {
            for (voice, step_tracker) in self.handle.voices.iter().zip(&mut self.step_trackers) {
                let triggers = voice.triggers();

                if let Some(_trigger) =
                    find_current_beat_trigger(triggers, step_tracker, position_beats)
                {
                    voice.looper_handle.trigger();
                }
            }
        }

        self.graph.process(data);

        for _sample in data.frames() {
            self.handle.time_info_provider.tick();
        }
    }
}

impl MidiEventHandler for MultiTrackLooper {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_starts_empty() {
        let looper = MultiTrackLooper::new(Default::default(), 8);
        assert_eq!(
            looper.handle.all_loopers_empty_other_than(LooperId(0)),
            true
        );
    }
}
