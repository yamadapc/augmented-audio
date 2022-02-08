use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};

use crate::new_processor::handle::LooperHandle;
use crate::sequencer::LoopSequencerProcessor;
use crate::LoopSequencerProcessorHandle;

pub mod handle;
mod scratch_pad;

pub struct LooperProcessor {
    handle: Shared<handle::LooperHandle>,
    sequencer: LoopSequencerProcessor,
}

impl Default for LooperProcessor {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl LooperProcessor {
    pub fn new(options: handle::LooperOptions) -> Self {
        let handle = make_shared(LooperHandle::new(options));
        Self {
            handle: handle.clone(),
            sequencer: LoopSequencerProcessor::new(handle),
        }
    }

    pub fn handle(&self) -> &Shared<handle::LooperHandle> {
        &self.handle
    }

    pub fn sequencer_handle(&self) -> &Shared<LoopSequencerProcessorHandle> {
        self.sequencer.handle()
    }
}

impl AudioProcessor for LooperProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.handle.prepare(settings);
        self.sequencer.prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            for (channel, sample) in frame.iter_mut().enumerate() {
                *sample = self.handle.process(channel, *sample);
            }
            self.handle.after_process();
        }

        self.sequencer.process(data);
    }
}

impl MidiEventHandler for LooperProcessor {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use audio_processor_testing_helpers::rms_level;
    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_testing_helpers::test_level_equivalence;

    use audio_processor_traits::{
        audio_buffer, AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer,
        VecAudioBuffer,
    };

    use crate::MAX_LOOP_LENGTH_SECS;

    use super::LooperProcessor;

    fn test_settings() -> AudioProcessorSettings {
        AudioProcessorSettings::new(44100.0, 1, 1, 512)
    }

    #[test]
    fn test_looper_produces_silence_when_started() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();

        looper.prepare(settings);

        let mut silence_buffer = Vec::new();
        // Produce 0.1 second empty buffer
        silence_buffer.resize((0.1 * settings.sample_rate()) as usize, 0.0);

        let mut audio_buffer = silence_buffer.clone();
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.process(&mut audio_buffer);
        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }

    #[test]
    fn test_looper_plays_its_input_back() {
        let mut looper = LooperProcessor::default();
        looper.handle.set_dry_volume(1.0);
        let settings = test_settings();
        looper.prepare(settings);

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        let mut audio_buffer = sine_buffer.clone();
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.process(&mut audio_buffer);

        test_level_equivalence(&sine_buffer, audio_buffer.slice(), 1, 1, 0.001);
    }

    #[test]
    fn test_looper_does_not_play_back_input_if_specified() {
        let _collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        assert_ne!(sine_buffer.len(), 0);
        println!("Sine samples: {:?}", sine_buffer);
        println!("Sine RMS: {}", rms_level(&sine_buffer));
        assert_ne!(rms_level(&sine_buffer), 0.0);

        let mut audio_buffer = sine_buffer;
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.handle().set_dry_volume(0.0);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }

    fn test_looper_record_and_playback(looper: &mut LooperProcessor) {
        let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();

        let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);

        looper.handle.start_recording();
        looper.process(&mut sample_buffer);
        looper.handle.stop_recording_audio_thread_only();

        // While recording, the output is muted
        let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "While recording the looper wasn't silent"
        );

        let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);

        looper.process(&mut output_buffer);
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();

        looper.handle().debug();
        assert_eq!(
            output_vec, sample_vec,
            "After recording the looper didn't playback - or played back a different input"
        );

        audio_buffer::clear(&mut output_buffer);

        looper.process(&mut output_buffer);
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();
        assert_eq!(
            output_vec, sample_vec,
            "The looper didn't playback its recording twice"
        );

        // Stop looper
        looper.handle.pause();
        looper.process(&mut sample_buffer);
        let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "The looper wasn't silent after stopped"
        );
    }

    fn make_silent_buffer(num_samples: usize) -> VecAudioBuffer<f32> {
        let silent_buffer: Vec<f32> = (0..num_samples).map(|_i| 0.0).collect();
        VecAudioBuffer::new_with(silent_buffer, 1, num_samples)
    }

    fn test_looper_is_silent(settings: &AudioProcessorSettings, looper: &mut LooperProcessor) {
        let num_samples = (MAX_LOOP_LENGTH_SECS * settings.sample_rate()) as usize;
        test_looper_is_silent_for(looper, num_samples);
    }

    fn test_looper_is_silent_for(looper: &mut LooperProcessor, num_samples: usize) {
        let mut output = make_silent_buffer(num_samples);
        looper.process(&mut output);
        let silent_buffer = make_silent_buffer(num_samples);
        assert_eq!(output, silent_buffer, "Looper was not silent");
    }

    #[test]
    fn test_looper_samples_at_start() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        test_looper_record_and_playback(&mut looper);
        looper.handle.clear();
        test_looper_is_silent(&settings, &mut looper);
    }

    #[test]
    fn test_looper_samples_at_edge() {
        let _collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::default();
        let settings = AudioProcessorSettings::new(100.0, 1, 1, 512);
        looper.prepare(settings);

        let num_samples = (MAX_LOOP_LENGTH_SECS * settings.sample_rate) as usize - 30;
        let mut sample_buffer: Vec<f32> = (0..num_samples).map(|i| i as f32).collect();
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
        looper.process(&mut sample_buffer);

        test_looper_is_silent(&settings, &mut looper);

        let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();

        let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);

        looper.handle.start_recording();
        looper.process(&mut sample_buffer);
        looper.handle.stop_recording_audio_thread_only();

        // While recording, the output is muted
        let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "While recording the looper wasn't silent"
        );

        let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);

        looper.process(&mut output_buffer);
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();
        assert_eq!(
            output_vec, sample_vec,
            "After recording the looper didn't playback - or played back a different input"
        );

        audio_buffer::clear(&mut output_buffer);

        looper.process(&mut output_buffer);
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();
        assert_eq!(
            output_vec, sample_vec,
            "The looper didn't playback its recording twice"
        );

        // Stop looper
        looper.handle().pause();
        looper.process(&mut sample_buffer);
        let empty_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "The looper wasn't silent after stopped"
        );

        looper.handle().clear();
        test_looper_is_silent(&settings, &mut looper);
    }

    // #[test]
    // fn test_looper_with_quantization() {
    //     let collector = basedrop::Collector::new();
    //     let mut time_info_provider = TimeInfoProviderImpl::new(None);
    //     let settings = AudioProcessorSettings::new(100.0, 1, 1, 512);
    //     time_info_provider.set_sample_rate(settings.sample_rate);
    //     time_info_provider.set_tempo(60);
    //
    //     let handle = make_shared(LooperProcessorHandle::new(
    //         audio_garbage_collector::handle(),
    //         LooperProcessorState::new(),
    //     ));
    //     let mut looper = LooperProcessor {
    //         id: "".to_string(),
    //         handle: handle.clone(),
    //         sequencer: LoopSequencerProcessor::new(handle),
    //         settings: Default::default(),
    //         time_info_provider,
    //     };
    //     looper.prepare(settings);
    //
    //     let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();
    //     let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
    //     let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
    //
    //     looper.handle.start_recording();
    //     looper.process(&mut sample_buffer);
    //     looper.handle.stop_recording();
    //     test_looper_is_silent_for(&mut looper, 299);
    //
    //     let mut output_buffer: Vec<f32> = (0..100).map(|_i| 0.0).collect();
    //     let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);
    //
    //     looper.process(&mut output_buffer);
    //     let output_vec = output_buffer.slice().iter().cloned().collect::<Vec<f32>>();
    //     let sample_vec = input_buffer.slice().iter().cloned().collect::<Vec<f32>>();
    //     assert_eq!(
    //         output_vec, sample_vec,
    //         "After recording the looper didn't playback"
    //     );
    // }
}
