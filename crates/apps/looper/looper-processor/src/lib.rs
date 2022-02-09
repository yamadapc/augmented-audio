pub use new_processor::handle::LooperHandle as LooperProcessorHandle;
pub use new_processor::handle::LooperOptions;
pub use new_processor::handle::QuantizeMode;
pub use new_processor::handle::QuantizeOptions;
pub use new_processor::LooperProcessor;
pub use sequencer::LoopSequencerParams;
pub use sequencer::LoopSequencerProcessorHandle;
pub use time_info_provider::{TimeInfoProvider, TimeInfoProviderImpl};

// use handle::state::RecordingState;
// use handle::LooperProcessorHandle;

mod handle;
mod loop_quantization;
mod midi_map;
mod new_processor;
mod sequencer;
mod time_info_provider;

const MAX_LOOP_LENGTH_SECS: f32 = 10.0;

// pub type LooperProcessor = RawLooperProcessor<TimeInfoProviderImpl>;
//
// /// A single stereo looper
// pub struct RawLooperProcessor<TimeInfoProviderType: TimeInfoProvider> {
//     id: String,
//     handle: Shared<LooperProcessorHandle>,
//     settings: AudioProcessorSettings,
//     time_info_provider: TimeInfoProviderType,
//     sequencer: sequencer::LoopSequencerProcessor,
// }
//
// impl RawLooperProcessor<TimeInfoProviderImpl> {
//     pub fn new(handle: &Handle, host_callback: Option<HostCallback>) -> Self {
//         let state = LooperProcessorState::new();
//         let time_info_provider = TimeInfoProviderImpl::new(host_callback.clone());
//         let handle = Shared::new(handle, LooperProcessorHandle::new(handle, state));
//         let sequencer = sequencer::LoopSequencerProcessor::new(handle.clone());
//         RawLooperProcessor {
//             id: uuid::Uuid::new_v4().to_string(),
//             handle,
//             settings: AudioProcessorSettings::default(),
//             time_info_provider,
//             sequencer,
//         }
//     }
//
//     pub fn sequencer_handle(&self) -> Shared<LoopSequencerProcessorHandle> {
//         self.sequencer.handle().clone()
//     }
//
//     pub fn handle(&self) -> Shared<LooperProcessorHandle> {
//         self.handle.clone()
//     }
// }
//
// impl<TimeInfoProviderType: TimeInfoProvider> AudioProcessor
//     for RawLooperProcessor<TimeInfoProviderType>
// {
//     type SampleType = f32;
//
//     fn prepare(&mut self, settings: AudioProcessorSettings) {
//         log::info!("Prepare looper {}", self.id);
//         if settings.output_channels() != settings.input_channels() {
//             log::error!("Prepare failed. Output/input channels mismatch");
//             return;
//         }
//
//         let num_channels = settings.input_channels();
//         self.handle
//             .state
//             .num_channels
//             .store(num_channels, Ordering::Relaxed);
//         let mut buffer = VecAudioBuffer::new();
//         buffer.resize(
//             num_channels,
//             (settings.sample_rate() * MAX_LOOP_LENGTH_SECS) as usize,
//             AtomicF32::new(0.0),
//         );
//         self.handle.state.looped_clip.set(make_shared(buffer));
//
//         self.settings = settings;
//     }
//
//     fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
//         &mut self,
//         data: &mut BufferType,
//     ) {
//         let looped_clip = self.handle.state.looped_clip.get();
//
//         for sample_index in 0..data.num_samples() {
//             let parameters = self.handle.parameters();
//             if parameters.should_clear {
//                 self.handle.set_should_clear(false);
//                 self.handle.state.clear();
//             }
//
//             let looper_cursor =
//                 self.handle.state.looper_cursor.get() as usize % looped_clip.num_samples();
//             let flooper_cursor = looper_cursor as f32;
//             for channel_num in 0..data.num_channels() {
//                 process_sample(
//                     flooper_cursor,
//                     looped_clip.deref(),
//                     &parameters,
//                     data,
//                     sample_index,
//                     channel_num,
//                 )
//             }
//
//             let recording_state = self.handle.state.loop_state.recording_state.get();
//             if recording_state.is_empty()
//                 || self.handle.is_playing_back()
//                 || self.handle.is_recording()
//             {
//                 self.handle.state.on_tick(
//                     &self.settings,
//                     &self.time_info_provider,
//                     self.handle.is_recording(),
//                     looper_cursor,
//                 );
//             }
//
//             if self.handle.is_playing_back() || self.handle.is_recording() {
//                 self.time_info_provider.tick();
//                 self.handle.force_stop_if_overflowing(looper_cursor + 1);
//             }
//         }
//
//         self.sequencer.process(data);
//     }
// }
//
// fn process_sample(
//     flooper_cursor: f32,
//     looped_clip: &impl AudioBuffer<SampleType = AtomicF32>,
//     parameters: &ProcessParameters<f32>,
//     data: &mut impl AudioBuffer<SampleType = f32>,
//     sample_index: usize,
//     channel_num: usize,
// ) {
//     let looper_cursor: usize = flooper_cursor as usize;
//
//     let ProcessParameters {
//         playback_input,
//         is_playing_back,
//         is_recording,
//         loop_volume,
//         dry_volume,
//         // state,
//         ..
//     } = *parameters;
//
//     // INPUT SECTION:
//     let input = *data.get(channel_num, sample_index);
//     let dry_output = if !playback_input { 0.0 } else { input };
//
//     // PLAYBACK SECTION:
//     let looper_output = if is_playing_back {
//         let looper_output1 = looped_clip.get(channel_num, looper_cursor).get();
//         let looper_output2 = looped_clip
//             .get(channel_num, (looper_cursor + 1) % looped_clip.num_samples())
//             .get();
//
//         let offset = flooper_cursor - looper_cursor as f32;
//
//         looper_output1 * (1.0 - offset) + looper_output2 * offset
//     } else {
//         0.0
//     };
//
//     // RECORDING SECTION:
//     if !is_playing_back && !is_recording {
//         let sample = looped_clip.get(channel_num, looper_cursor);
//         sample.set(*data.get(channel_num, sample_index));
//     } else if is_recording {
//         let sample = looped_clip.get(channel_num, looper_cursor);
//         sample.set(*data.get(channel_num, sample_index) + sample.get());
//     }
//
//     // OUTPUT SECTION:
//     let mixed_output = loop_volume * looper_output + dry_volume * dry_output;
//     data.set(channel_num, sample_index, mixed_output);
// }
//
// impl<TimeInfoProviderType: TimeInfoProvider> MidiEventHandler
//     for RawLooperProcessor<TimeInfoProviderType>
// {
//     fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
//         for message in midi_messages {
//             let status = message
//                 .bytes()
//                 .map(|bytes| rimd::Status::from_u8(bytes[0]).map(|status| (status, bytes)))
//                 .flatten();
//             if let Some((_status, bytes)) = status {
//                 if let Some(action) = self
//                     .handle
//                     .midi_map()
//                     .get(&MidiSpec::new(bytes[0], bytes[1]))
//                 {
//                     match action {
//                         Action::SetRecording(value) => {
//                             if value {
//                                 self.handle.start_recording()
//                             } else {
//                                 self.handle.stop_recording()
//                             }
//                         }
//                         Action::SetPlayback(value) => {
//                             if value {
//                                 self.handle.play()
//                             } else {
//                                 self.handle.stop()
//                             }
//                         }
//                         Action::Clear => {
//                             self.handle.clear();
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use std::sync::atomic::Ordering;
//     use std::time::Duration;
//
//     use audio_processor_testing_helpers::rms_level;
//     use audio_processor_testing_helpers::sine_buffer;
//     use audio_processor_testing_helpers::test_level_equivalence;
//     use log::Record;
//
//     use audio_garbage_collector::make_shared;
//     use audio_processor_traits::{
//         audio_buffer, AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer,
//         VecAudioBuffer,
//     };
//
//     use crate::sequencer::LoopSequencerProcessor;
//     use crate::{
//         LooperProcessorHandle, LooperProcessorState, RawLooperProcessor, RecordingState,
//         TimeInfoProvider, TimeInfoProviderImpl, MAX_LOOP_LENGTH_SECS,
//     };
//
//     fn test_settings() -> AudioProcessorSettings {
//         AudioProcessorSettings::new(44100.0, 1, 1, 512)
//     }
//
//     #[test]
//     fn test_looper_produces_silence_when_started() {
//         let collector = basedrop::Collector::new();
//         let mut looper = RawLooperProcessor::new(&collector.handle(), None);
//         let settings = test_settings();
//
//         looper.prepare(settings);
//
//         let mut silence_buffer = Vec::new();
//         // Produce 0.1 second empty buffer
//         silence_buffer.resize((0.1 * settings.sample_rate()) as usize, 0.0);
//
//         let mut audio_buffer = silence_buffer.clone();
//         let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
//         looper.process(&mut audio_buffer);
//         assert_eq!(rms_level(audio_buffer.slice()), 0.0);
//     }
//
//     #[test]
//     fn test_looper_plays_its_input_back() {
//         let collector = basedrop::Collector::new();
//         let mut looper = RawLooperProcessor::new(&collector.handle(), None);
//         looper.handle.set_dry_volume(1.0);
//         let settings = test_settings();
//         looper.prepare(settings);
//
//         let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
//         let mut audio_buffer = sine_buffer.clone();
//         let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
//         looper.process(&mut audio_buffer);
//
//         test_level_equivalence(&sine_buffer, audio_buffer.slice(), 1, 1, 0.001);
//     }
//
//     #[test]
//     fn test_looper_does_not_play_back_input_if_specified() {
//         let collector = basedrop::Collector::new();
//         let mut looper = RawLooperProcessor::new(&collector.handle(), None);
//         let settings = test_settings();
//         looper.prepare(settings);
//
//         let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
//         assert_ne!(sine_buffer.len(), 0);
//         println!("Sine samples: {:?}", sine_buffer);
//         println!("Sine RMS: {}", rms_level(&sine_buffer));
//         assert_ne!(rms_level(&sine_buffer), 0.0);
//
//         let mut audio_buffer = sine_buffer;
//         let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
//         looper.handle().store_playback_input(false);
//         looper.process(&mut audio_buffer);
//
//         assert_eq!(rms_level(audio_buffer.slice()), 0.0);
//     }
//
//     fn test_looper_record_and_playback<TimeInfoProviderType: TimeInfoProvider>(
//         looper: &mut RawLooperProcessor<TimeInfoProviderType>,
//     ) {
//         let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();
//
//         let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
//         let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
//
//         looper.handle.start_recording();
//         looper.process(&mut sample_buffer);
//         looper.handle.stop_recording();
//         // The looper will drop the next sample, that's expected behaviour
//         skip_one_sample(looper);
//
//         // While recording, the output is muted
//         let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let initial_output = sample_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             empty_buffer, initial_output,
//             "While recording the looper wasn't silent"
//         );
//
//         let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);
//
//         looper.process(&mut output_buffer);
//         let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             output_vec, sample_vec,
//             "After recording the looper didn't playback - or played back a different input"
//         );
//
//         audio_buffer::clear(&mut output_buffer);
//
//         looper.process(&mut output_buffer);
//         let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             output_vec, sample_vec,
//             "The looper didn't playback its recording twice"
//         );
//
//         // Stop looper
//         looper.handle.stop();
//         looper.process(&mut sample_buffer);
//         let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let initial_output = sample_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             empty_buffer, initial_output,
//             "The looper wasn't silent after stopped"
//         );
//     }
//
//     fn skip_one_sample<TimeInfoProviderType: TimeInfoProvider>(
//         looper: &mut RawLooperProcessor<TimeInfoProviderType>,
//     ) {
//         let mut one_sample_buffer = VecAudioBuffer::new_with(vec![0.0], 1, 1);
//         looper.process(&mut one_sample_buffer);
//     }
//
//     fn make_silent_buffer(num_samples: usize) -> VecAudioBuffer<f32> {
//         let silent_buffer: Vec<f32> = (0..num_samples).map(|_i| 0.0).collect();
//         VecAudioBuffer::new_with(silent_buffer, 1, num_samples)
//     }
//
//     fn test_looper_is_silent<TimeInfoProviderType: TimeInfoProvider>(
//         settings: &AudioProcessorSettings,
//         looper: &mut RawLooperProcessor<TimeInfoProviderType>,
//     ) {
//         let num_samples = (MAX_LOOP_LENGTH_SECS * settings.sample_rate()) as usize;
//         test_looper_is_silent_for(looper, num_samples);
//     }
//
//     fn test_looper_is_silent_for<TimeInfoProviderType: TimeInfoProvider>(
//         looper: &mut RawLooperProcessor<TimeInfoProviderType>,
//         num_samples: usize,
//     ) {
//         let mut output = make_silent_buffer(num_samples);
//         looper.process(&mut output);
//         let silent_buffer = make_silent_buffer(num_samples);
//         assert_eq!(output, silent_buffer, "Looper was not silent");
//     }
//
//     #[test]
//     fn test_looper_samples_at_start() {
//         let collector = basedrop::Collector::new();
//         let mut looper = RawLooperProcessor::new(&collector.handle(), None);
//         let settings = test_settings();
//         looper.prepare(settings);
//
//         test_looper_record_and_playback(&mut looper);
//         looper.handle.clear();
//         test_looper_is_silent(&settings, &mut looper);
//     }
//
//     #[test]
//     fn test_looper_samples_at_edge() {
//         let collector = basedrop::Collector::new();
//         let mut looper = RawLooperProcessor::new(&collector.handle(), None);
//         let settings = AudioProcessorSettings::new(100.0, 1, 1, 512);
//         looper.prepare(settings);
//
//         let num_samples = (MAX_LOOP_LENGTH_SECS * settings.sample_rate) as usize - 30;
//         let mut sample_buffer: Vec<f32> = (0..num_samples).map(|i| i as f32).collect();
//         let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
//         looper.process(&mut sample_buffer);
//
//         test_looper_is_silent(&settings, &mut looper);
//
//         let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();
//
//         let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
//         let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
//
//         looper.handle.start_recording();
//         looper.process(&mut sample_buffer);
//         looper.handle.stop_recording();
//         // The looper will drop the next sample, that's expected behaviour
//         skip_one_sample(&mut looper);
//         assert_eq!(
//             looper.handle.state.loop_state.start.load(Ordering::Relaxed),
//             970
//         );
//         assert_eq!(
//             looper.handle.state.loop_state.end.load(Ordering::Relaxed),
//             70
//         );
//         assert_eq!(looper.handle.num_samples(), 100);
//         assert_eq!(
//             looper.handle.state.loop_state.recording_state.get(),
//             RecordingState::Playing
//         );
//
//         // While recording, the output is muted
//         let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let initial_output = sample_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             empty_buffer, initial_output,
//             "While recording the looper wasn't silent"
//         );
//
//         let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);
//
//         looper.process(&mut output_buffer);
//         let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             output_vec, sample_vec,
//             "After recording the looper didn't playback - or played back a different input"
//         );
//
//         audio_buffer::clear(&mut output_buffer);
//
//         looper.process(&mut output_buffer);
//         let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             output_vec, sample_vec,
//             "The looper didn't playback its recording twice"
//         );
//
//         // Stop looper
//         looper.handle.stop();
//         looper.process(&mut sample_buffer);
//         let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let initial_output = sample_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             empty_buffer, initial_output,
//             "The looper wasn't silent after stopped"
//         );
//
//         looper.handle.clear();
//         test_looper_is_silent(&settings, &mut looper);
//     }
//
//     #[test]
//     fn test_looper_with_quantization() {
//         let collector = basedrop::Collector::new();
//         let mut time_info_provider = TimeInfoProviderImpl::new(None);
//         let settings = AudioProcessorSettings::new(100.0, 1, 1, 512);
//         time_info_provider.set_sample_rate(settings.sample_rate);
//         time_info_provider.set_tempo(60);
//
//         let handle = make_shared(LooperProcessorHandle::new(
//             audio_garbage_collector::handle(),
//             LooperProcessorState::new(),
//         ));
//         let mut looper = RawLooperProcessor {
//             id: "".to_string(),
//             handle: handle.clone(),
//             sequencer: LoopSequencerProcessor::new(handle),
//             settings: Default::default(),
//             time_info_provider,
//         };
//         looper.prepare(settings);
//
//         let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();
//         let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
//         let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
//
//         looper.handle.start_recording();
//         assert_eq!(
//             looper.handle.state.loop_state.start.load(Ordering::Relaxed),
//             0
//         );
//
//         looper.process(&mut sample_buffer);
//         looper.handle.stop_recording();
//         skip_one_sample(&mut looper);
//         assert_eq!(
//             looper.handle.state.loop_state.start.load(Ordering::Relaxed),
//             0
//         );
//         assert_eq!(
//             looper.handle.state.loop_state.end.load(Ordering::Relaxed),
//             400
//         );
//
//         assert_eq!(
//             looper.handle.state.loop_state.recording_state.get(),
//             RecordingState::Recording
//         );
//         test_looper_is_silent_for(&mut looper, 299);
//         assert_eq!(
//             looper.handle.state.loop_state.recording_state.get(),
//             RecordingState::Playing
//         );
//
//         let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
//         let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);
//
//         looper.process(&mut output_buffer);
//         let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
//         assert_eq!(
//             output_vec, sample_vec,
//             "After recording the looper didn't playback"
//         );
//     }
// }
