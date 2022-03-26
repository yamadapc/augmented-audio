use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
    SimpleAudioProcessor,
};
use handle::LooperHandle;

use crate::sequencer::LoopSequencerProcessor;
use crate::{LoopSequencerProcessorHandle, TimeInfoProviderImpl};

pub mod handle;

pub struct LooperProcessor {
    handle: Shared<handle::LooperHandle>,
    sequencer: LoopSequencerProcessor,
}

impl Default for LooperProcessor {
    fn default() -> Self {
        Self::from_options(Default::default())
    }
}

impl LooperProcessor {
    pub fn new(
        options: handle::LooperOptions,
        time_info_provider: Shared<TimeInfoProviderImpl>,
    ) -> Self {
        let handle = make_shared(LooperHandle::new(options, time_info_provider));
        Self {
            handle: handle.clone(),
            sequencer: LoopSequencerProcessor::new(handle),
        }
    }

    pub fn from_options(options: handle::LooperOptions) -> Self {
        let handle = make_shared(LooperHandle::from_options(options));
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

        if self.handle.is_playing_back() {
            self.sequencer.process(data);
        }
    }
}

impl SimpleAudioProcessor for LooperProcessor {
    type SampleType = f32;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        log::debug!("Prepare looper {:?}", settings);
        self.prepare(settings);
    }

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = self.handle.process(channel, *sample);
        }
        self.handle.after_process();

        if self.handle.is_playing_back() {
            self.sequencer.s_process_frame(frame);
        }
    }
}

impl MidiEventHandler for LooperProcessor {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_testing_helpers::test_level_equivalence;
    use audio_processor_testing_helpers::{assert_f_eq, rms_level};

    use audio_processor_traits::{
        audio_buffer, AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer,
        OwnedAudioBuffer, VecAudioBuffer,
    };
    use handle::{LooperState, QuantizeMode};

    use crate::{TimeInfoProvider, MAX_LOOP_LENGTH_SECS};
    use assert_no_alloc::assert_no_alloc;
    use itertools::Itertools;

    use super::*;

    fn test_settings() -> AudioProcessorSettings {
        AudioProcessorSettings::new(44100.0, 1, 1, 512)
    }

    #[test]
    fn test_create_looper() {
        let _looper = LooperProcessor::default();
    }

    #[test]
    fn test_looper_buffer_has_recording_contents() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        let test_buffer_vec = vec![1.0, 2.0, 3.0, 4.0];
        let mut test_buffer = VecAudioBuffer::from(test_buffer_vec.clone());

        looper.handle.start_recording();
        looper.process(&mut test_buffer);
        looper.handle.stop_recording_allocating_loop();
        looper.process(&mut test_buffer);

        let looper_clip = looper.handle.looper_clip();
        let looper_clip = looper_clip.borrow();
        let looper_clip = looper_clip.slice().iter().map(|f| f.get()).collect_vec();
        assert_eq!(test_buffer_vec, looper_clip);
    }

    #[test]
    fn test_looper_buffer_can_be_set() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        let test_buffer_vec = vec![1.0, 2.0, 3.0, 4.0];
        let mut test_buffer = VecAudioBuffer::from(test_buffer_vec.clone());
        looper.handle.set_looper_buffer(&test_buffer.interleaved());

        let looper_clip = looper.handle.looper_clip();
        let looper_clip = looper_clip.borrow();
        let looper_clip = looper_clip.slice().iter().map(|f| f.get()).collect_vec();
        assert_eq!(test_buffer_vec, looper_clip);
    }

    #[test]
    fn test_looper_buffer_will_playback_if_programmatically_set() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        let test_buffer_vec = vec![1.0, 2.0, 3.0, 4.0];
        let mut test_buffer = VecAudioBuffer::from(test_buffer_vec.clone());
        looper.handle.set_looper_buffer(&test_buffer.interleaved());
        looper.handle.play();

        let mut output_buffer = VecAudioBuffer::new();
        output_buffer.resize(1, 8, 0.0);
        looper.process(&mut output_buffer);

        let output = output_buffer.slice().to_vec();
        assert_eq!(output, vec![1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_looper_buffer_will_start_silent_after_being_set() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        let test_buffer_vec = vec![1.0, 2.0, 3.0, 4.0];
        let mut test_buffer = VecAudioBuffer::from(test_buffer_vec.clone());
        looper.handle.set_looper_buffer(&test_buffer.interleaved());

        let mut output_buffer = VecAudioBuffer::new();
        output_buffer.resize(1, 8, 0.0);
        looper.process(&mut output_buffer);

        let output = output_buffer.slice().to_vec();
        assert_eq!(output, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
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
        assert_ne!(rms_level(&sine_buffer), 0.0);

        let mut audio_buffer = sine_buffer;
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.handle().set_dry_volume(0.0);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }

    fn test_looper_record_and_playback(looper: &mut LooperProcessor) {
        let mut sample_buffer: Vec<f32> = (0..10).map(|i| i as f32).collect();

        let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);

        looper.handle.start_recording();
        looper.process(&mut sample_buffer);
        looper.handle.stop_recording_audio_thread_only();

        // While recording, the output is MUTED
        let empty_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "While recording the looper wasn't silent"
        );

        let mut output_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
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
        looper.handle.pause();
        looper.process(&mut sample_buffer);
        let empty_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
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
        assert_no_alloc(|| {
            looper.process(&mut output);
        });
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
    fn test_looper_overdub() {
        let mut looper = LooperProcessor::default();
        let settings = test_settings();
        looper.prepare(settings);

        // Record and test output is good
        {
            let mut buffer: Vec<f32> = (0..10).map(|_i| 1.0).collect();
            let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer);
            looper.handle.start_recording();
            assert_no_alloc(|| {
                looper.process(&mut buffer);
            });
            looper.handle.stop_recording_allocating_loop();

            let mut buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
            let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer);
            assert_no_alloc(|| {
                looper.process(&mut buffer);
            });
            let output_vec = buffer.slice().to_vec();
            let sample_vec: Vec<f32> = (0..10).map(|_i| 1.0).collect();
            assert_eq!(output_vec, sample_vec, "Recording didn't work");
        }

        // Run overdubbing
        {
            let mut buffer: Vec<f32> = (0..10).map(|_i| 1.0).collect();
            let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer);
            looper.handle.start_recording();
            assert_no_alloc(|| {
                looper.process(&mut buffer);
            });
            looper.handle.stop_recording_allocating_loop();
        }

        // Test output is summed
        let mut buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
        let mut buffer = InterleavedAudioBuffer::new(1, &mut buffer);

        assert_no_alloc(|| {
            looper.process(&mut buffer);
        });
        let output_vec = buffer.slice().to_vec();
        let sample_vec: Vec<f32> = (0..10).map(|_i| 2.0).collect();
        assert_eq!(output_vec, sample_vec, "Overdub didn't work");
    }

    #[test]
    fn test_looper_samples_at_edge() {
        let _collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::default();
        let settings = AudioProcessorSettings::new(10.0, 1, 1, 512);
        looper.prepare(settings);

        let num_samples = (MAX_LOOP_LENGTH_SECS * settings.sample_rate) as usize - 30;
        let mut sample_buffer: Vec<f32> = (0..num_samples).map(|i| i as f32).collect();
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
        assert_no_alloc(|| {
            looper.process(&mut sample_buffer);
        });

        test_looper_is_silent(&settings, &mut looper);

        let mut sample_buffer: Vec<f32> = (0..10).map(|i| i as f32).collect();

        let input_buffer = VecAudioBuffer::new_with(sample_buffer.clone(), 1, sample_buffer.len());
        let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);

        looper.handle.start_recording();
        assert_no_alloc(|| {
            looper.process(&mut sample_buffer);
        });
        looper.handle.stop_recording_audio_thread_only();

        // While recording, the output is MUTED
        let empty_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "While recording the looper wasn't silent"
        );

        let mut output_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
        let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);

        assert_no_alloc(|| {
            looper.process(&mut output_buffer);
        });
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();
        assert_eq!(
            output_vec, sample_vec,
            "After recording the looper didn't playback - or played back a different input"
        );

        audio_buffer::clear(&mut output_buffer);

        assert_no_alloc(|| {
            looper.process(&mut output_buffer);
        });
        let output_vec = output_buffer.slice().to_vec();
        let sample_vec = input_buffer.slice().to_vec();
        assert_eq!(
            output_vec, sample_vec,
            "The looper didn't playback its recording twice"
        );

        // Stop looper
        looper.handle().pause();
        assert_no_alloc(|| {
            looper.process(&mut sample_buffer);
        });
        let empty_buffer: Vec<f32> = (0..10).map(|_i| 0.0).collect();
        let initial_output = sample_buffer.slice().to_vec();
        assert_eq!(
            empty_buffer, initial_output,
            "The looper wasn't silent after stopped"
        );

        looper.handle().clear();
        test_looper_is_silent(&settings, &mut looper);
    }

    #[test]
    fn test_looper_with_quantization_will_wait_until_a_beat() {
        wisual_logger::init_from_env();
        let settings = AudioProcessorSettings::new(100.0, 1, 1, 512);
        let mut looper = LooperProcessor::default();
        looper.prepare(settings);
        looper.handle.tick_time.store(true, Ordering::Relaxed);

        // Setup tempo & quantization
        looper.handle.set_tempo(60.0);
        looper.handle.time_info_provider().play();
        looper
            .handle
            .quantize_options()
            .set_mode(QuantizeMode::SnapClosest);
        let position_beats = get_position_beats(&mut looper);
        assert_f_eq!(position_beats, 0.0);

        {
            let mut sample_buffer: Vec<f32> = (0..100).map(|i| i as f32).collect();
            let mut sample_buffer = InterleavedAudioBuffer::new(1, &mut sample_buffer);
            // We process 1s of audio; which is 1 beat
            assert_no_alloc(|| {
                looper.process(&mut sample_buffer);
            });
        }
        let position_beats = get_position_beats(&mut looper);
        assert_f_eq!(position_beats, 1.0);

        looper.handle.start_recording();
        assert_eq!(looper.handle.state(), LooperState::RecordingScheduled);

        // We process 3 more beats of audio
        test_looper_is_silent_for(&mut looper, 300);
        // Now we're on beat 0, we should be recording
        let position_beats = get_position_beats(&mut looper);
        assert_f_eq!(position_beats, 4.0);
        assert_eq!(looper.handle.state(), LooperState::Recording);

        // We record some audio in
        let mut recorded_buffer: Vec<f32> = (0..400).map(|i| i as f32).collect();
        let mut recorded_buffer = InterleavedAudioBuffer::new(1, &mut recorded_buffer);
        assert_no_alloc(|| {
            looper.process(&mut recorded_buffer);
        });
        let position_beats = get_position_beats(&mut looper);
        assert!((position_beats - 8.0).abs() < 0.0001);
        looper.handle.stop_recording_allocating_loop();
        assert_eq!(looper.handle.state(), LooperState::Playing);

        // We expect audio to be played back now
        let mut output_buffer: Vec<f32> = (0..200).map(|_i| 0.0).collect();
        let mut output_buffer = InterleavedAudioBuffer::new(1, &mut output_buffer);
        assert_no_alloc(|| {
            looper.process(&mut output_buffer);
        });
        assert_eq!(looper.handle.state(), LooperState::Playing);

        let output_vec = output_buffer.slice().to_vec();
        assert_eq!(
            output_vec,
            (0..200).map(|i| i as f32).collect::<Vec<f32>>(),
            "After recording the looper didn't playback"
        );
    }

    fn get_position_beats(looper: &mut LooperProcessor) -> f64 {
        looper
            .handle
            .time_info_provider()
            .get_time_info()
            .position_beats()
            .unwrap()
    }
}
