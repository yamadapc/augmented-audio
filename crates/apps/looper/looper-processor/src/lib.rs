use std::ops::AddAssign;
use std::time::Duration;

use num::FromPrimitive;

use audio_garbage_collector::{Handle, Shared};
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};

use crate::buffer::InternalBuffer;
pub use crate::handle::LooperProcessorHandle;
use crate::handle::ProcessParameters;
use crate::midi_map::{Action, MidiSpec};

mod buffer;
mod handle;
mod midi_map;

enum LoopState {
    Empty,
    Recording { start: usize },
    PlayOrOverdub { start: usize, end: usize },
}

/// Private not thread safe mutable state
struct LooperProcessorState<SampleType: num::Float> {
    loop_state: LoopState,
    looper_cursor: usize,
    num_channels: usize,
    looped_clip: InternalBuffer<SampleType>,
}

impl<SampleType: num::Float> LooperProcessorState<SampleType> {
    pub fn new() -> Self {
        LooperProcessorState {
            num_channels: 2,
            looper_cursor: 0,
            loop_state: LoopState::Empty,
            looped_clip: InternalBuffer::new(),
        }
    }

    pub fn increment_cursor(&mut self) {
        self.looper_cursor += 1;
        if let LoopState::PlayOrOverdub { start, end } = self.loop_state {
            if end > start {
                if self.looper_cursor >= end {
                    self.looper_cursor = start;
                }
            } else {
                // End point is before start
                let loop_length = self.looped_clip.num_samples() - start + end;
                if self.looper_cursor >= start {
                    let cursor_relative_to_start = self.looper_cursor - start;
                    if cursor_relative_to_start >= loop_length {
                        self.looper_cursor = start;
                    }
                } else {
                    let cursor_relative_to_start =
                        self.looper_cursor - end + self.looped_clip.num_samples() - start;
                    if cursor_relative_to_start >= loop_length {
                        self.looper_cursor = start;
                    }
                }
            }
        } else {
            self.looper_cursor %= self.looped_clip.num_samples();
        }
    }

    fn clear(&mut self) {
        self.loop_state = LoopState::Empty;
    }

    fn on_tick(&mut self, is_recording: bool, looper_cursor: usize) {
        match self.loop_state {
            // Loop has ended
            LoopState::Recording { start } if !is_recording => {
                self.loop_state = LoopState::PlayOrOverdub {
                    start,
                    end: looper_cursor,
                };
            }
            // Loop has started
            LoopState::Empty if is_recording => {
                self.loop_state = LoopState::Recording {
                    start: looper_cursor,
                };
            }
            _ => {}
        }

        self.increment_cursor();
    }
}

/// A single stereo looper
pub struct LooperProcessor<SampleType: num::Float> {
    pub id: String,
    state: LooperProcessorState<SampleType>,
    handle: Shared<LooperProcessorHandle<SampleType>>,
}

impl<SampleType: num::Float + 'static> LooperProcessor<SampleType> {
    pub fn new(handle: &Handle) -> Self {
        LooperProcessor {
            id: uuid::Uuid::new_v4().to_string(),
            state: LooperProcessorState::new(),
            handle: Shared::new(handle, LooperProcessorHandle::new(handle)),
        }
    }

    pub fn handle(&self) -> Shared<LooperProcessorHandle<SampleType>> {
        self.handle.clone()
    }
}

impl<SampleType: num::Float + Send + Sync + AddAssign> AudioProcessor
    for LooperProcessor<SampleType>
{
    type SampleType = SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        log::info!("Prepare looper {}", self.id);
        if settings.output_channels() != settings.input_channels() {
            log::error!("Prepare failed. Output/input channels mismatch");
            return;
        }

        let num_channels = settings.input_channels();
        self.state.num_channels = num_channels;
        self.state.looped_clip.resize(
            num_channels,
            settings.sample_rate(),
            Duration::from_secs(10),
        );
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample_index in 0..data.num_samples() {
            let parameters = self.handle.parameters();
            if parameters.should_clear {
                self.handle.set_should_clear(false);
                self.state.clear();
            }

            let looper_cursor = self.state.looper_cursor;
            let mut viz_input = BufferType::SampleType::zero();

            for channel_num in 0..data.num_channels() {
                self.process_sample(&parameters, data, sample_index, channel_num, &mut viz_input)
            }

            self.handle.queue.push(viz_input);
            self.state.on_tick(parameters.is_recording, looper_cursor);
        }
    }
}

impl<SampleType: num::Float + AddAssign> LooperProcessor<SampleType> {
    fn process_sample<BufferType: AudioBuffer<SampleType = SampleType>>(
        &mut self,
        parameters: &ProcessParameters<SampleType>,
        data: &mut BufferType,
        sample_index: usize,
        channel_num: usize,
        viz_input: &mut SampleType,
    ) {
        let looper_cursor: usize = self.state.looper_cursor;

        let ProcessParameters {
            playback_input,
            is_playing_back,
            is_recording,
            loop_volume,
            dry_volume,
            ..
        } = *parameters;

        let loop_channel = self.state.looped_clip.channel(channel_num);
        // INPUT SECTION:
        let input = *data.get(channel_num, sample_index);
        let dry_output = Self::process_input(playback_input, input);
        *viz_input += dry_output;

        // PLAYBACK SECTION:
        let looper_output = loop_channel[looper_cursor];
        let looper_output = if is_playing_back {
            looper_output
        } else {
            SampleType::zero()
        };

        let mixed_output = loop_volume * looper_output + dry_volume * dry_output;
        data.set(channel_num, sample_index, mixed_output);

        // RECORDING SECTION:
        if is_recording {
            // When recording starts we'll store samples in the looper buffer
            loop_channel[looper_cursor] =
                *data.get(channel_num, sample_index) + loop_channel[looper_cursor];
        }
    }

    fn process_input(playback_input: bool, input: SampleType) -> SampleType {
        if !playback_input {
            SampleType::zero()
        } else {
            input
        }
    }

    #[allow(dead_code)]
    fn toggle_recording(&mut self) {
        self.handle.toggle_recording();
    }

    #[allow(dead_code)]
    fn stop(&mut self) {
        self.handle.stop();
    }
}

impl<SampleType: num::Float + Send + Sync + std::ops::AddAssign> MidiEventHandler
    for LooperProcessor<SampleType>
{
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        for message in midi_messages {
            let status = message
                .bytes()
                .map(|bytes| rimd::Status::from_u8(bytes[0]).map(|status| (status, bytes)))
                .flatten();
            if let Some((_status, bytes)) = status {
                if let Some(action) = self
                    .handle
                    .midi_map()
                    .get(&MidiSpec::new(bytes[0], bytes[1]))
                {
                    match action {
                        Action::SetRecording(value) => {
                            if value {
                                self.handle.start_recording()
                            } else {
                                self.handle.stop_recording()
                            }
                        }
                        Action::SetPlayback(value) => {
                            if value {
                                self.handle.play()
                            } else {
                                self.handle.stop()
                            }
                        }
                        Action::Clear => {
                            self.handle.clear();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::rms_level;
    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_testing_helpers::test_level_equivalence;

    use audio_processor_traits::{
        AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer,
    };

    use crate::LooperProcessor;

    fn test_settings() -> AudioProcessorSettings {
        AudioProcessorSettings::new(44100.0, 1, 1, 512)
    }

    #[test]
    fn test_looper_produces_silence_when_started() {
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
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
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
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
        let collector = basedrop::Collector::new();
        let mut looper = LooperProcessor::new(&collector.handle());
        let settings = test_settings();
        looper.prepare(settings);

        let sine_buffer = sine_buffer(settings.sample_rate(), 440.0, Duration::from_secs_f32(0.1));
        assert_ne!(sine_buffer.len(), 0);
        println!("Sine samples: {:?}", sine_buffer);
        println!("Sine RMS: {}", rms_level(&sine_buffer));
        assert_ne!(rms_level(&sine_buffer), 0.0);

        let mut audio_buffer = sine_buffer;
        let mut audio_buffer = InterleavedAudioBuffer::new(1, &mut audio_buffer);
        looper.handle().store_playback_input(false);
        looper.process(&mut audio_buffer);

        assert_eq!(rms_level(audio_buffer.slice()), 0.0);
    }
}
