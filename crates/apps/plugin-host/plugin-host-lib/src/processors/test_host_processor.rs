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

use std::ops::Deref;
use std::time::Duration;

use vst::host::PluginInstance;
use vst::plugin::Plugin;

use audio_garbage_collector::{Handle, Shared};
use audio_processor_standalone_midi::host::MidiMessageEntry;
use audio_processor_standalone_midi::vst::MidiVSTConverter;
use audio_processor_traits::{
    AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings, BufferProcessor,
};

use crate::audio_io::cpal_vst_buffer_handler::CpalVstBufferHandler;
use crate::audio_io::processor_handle_registry::ProcessorHandleRegistry;
use crate::processors::audio_file_processor::{AudioFileProcessor, InMemoryAudioFile};
use crate::processors::running_rms_processor::{RunningRMSProcessor, RunningRMSProcessorHandle};
use crate::processors::shared_processor::SharedProcessor;
use crate::processors::volume_meter_processor::{VolumeMeterProcessor, VolumeMeterProcessorHandle};

pub type TestHostProcessor = TestHostProcessorImpl<PluginInstance>;

pub struct TestHostProcessorHandle {
    volume: AtomicF32,
}

impl TestHostProcessorHandle {
    pub fn set_volume(&self, volume: f32) {
        self.volume.set(volume);
    }
}

/// The app's main processor
pub struct TestHostProcessorImpl<PluginInstanceT: Plugin + 'static> {
    id: String,
    handle: Shared<TestHostProcessorHandle>,
    plugin_instance: SharedProcessor<PluginInstanceT>,
    audio_settings: AudioProcessorSettings,
    buffer_handler: CpalVstBufferHandler,
    maybe_audio_file_processor: Option<AudioFileProcessor>,
    volume_meter_processor: VolumeMeterProcessor,
    running_rms_processor: BufferProcessor<RunningRMSProcessor>,
    midi_converter: MidiVSTConverter,
    mono_input: Option<usize>,
}

unsafe impl<P: Plugin + 'static> Send for TestHostProcessorImpl<P> {}
unsafe impl<P: Plugin + 'static> Sync for TestHostProcessorImpl<P> {}

impl<PluginInstanceT: Plugin> TestHostProcessorImpl<PluginInstanceT> {
    pub fn new(
        handle: &Handle,
        maybe_audio_file_settings: Option<InMemoryAudioFile>,
        plugin_instance: SharedProcessor<PluginInstanceT>,
        sample_rate: f32,
        channels: usize,
        buffer_size: usize,
        mono_input: Option<usize>,
    ) -> Self {
        let audio_settings =
            AudioProcessorSettings::new(sample_rate, channels, channels, buffer_size);
        let volume_meter_processor = VolumeMeterProcessor::new(handle);

        let host_processor_handle = Shared::new(
            handle,
            TestHostProcessorHandle {
                volume: AtomicF32::new(1.0),
            },
        );

        ProcessorHandleRegistry::current()
            .register("test-host-processor", host_processor_handle.clone());

        let running_rms_processor = BufferProcessor(RunningRMSProcessor::new_with_duration(
            handle,
            Duration::from_millis(300),
        ));
        ProcessorHandleRegistry::current()
            .register("rms-processor", running_rms_processor.0.handle().clone());

        let maybe_audio_file_processor = maybe_audio_file_settings.map(|audio_file_settings| {
            AudioFileProcessor::new(handle, audio_file_settings, audio_settings)
        });

        if let Some(audio_file_processor) = &maybe_audio_file_processor {
            ProcessorHandleRegistry::current()
                .register("audio-file", audio_file_processor.handle().clone());
        }

        TestHostProcessorImpl {
            id: uuid::Uuid::new_v4().to_string(),
            handle: host_processor_handle,
            plugin_instance,
            audio_settings,
            buffer_handler: CpalVstBufferHandler::new(audio_settings),
            maybe_audio_file_processor,
            volume_meter_processor,
            running_rms_processor,
            midi_converter: MidiVSTConverter::default(),
            mono_input,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Resume playback
    pub fn play(&self) {
        if let Some(audio_file_processor) = &self.maybe_audio_file_processor {
            audio_file_processor.play();
        }
    }

    /// Pause playback
    pub fn pause(&self) {
        if let Some(audio_file_processor) = &self.maybe_audio_file_processor {
            audio_file_processor.pause();
        }
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        if let Some(audio_file_processor) = &self.maybe_audio_file_processor {
            audio_file_processor.stop();
        }
    }

    /// Whether the file is being played back
    pub fn is_playing(&self) -> bool {
        if let Some(audio_file_processor) = &self.maybe_audio_file_processor {
            audio_file_processor.is_playing()
        } else {
            false
        }
    }

    pub fn handle(&self) -> &Shared<TestHostProcessorHandle> {
        &self.handle
    }

    pub fn volume_handle(&self) -> &Shared<VolumeMeterProcessorHandle> {
        self.volume_meter_processor.handle()
    }

    pub fn current_output_volume(&self) -> (f32, f32) {
        self.volume_meter_processor.current_volume()
    }

    pub fn running_rms_processor_handle(&self) -> &Shared<RunningRMSProcessorHandle> {
        self.running_rms_processor.0.handle()
    }

    pub fn set_volume(&self, volume: f32) {
        self.handle.set_volume(volume);
    }
}

impl<PluginInstanceT: Plugin> TestHostProcessorImpl<PluginInstanceT> {
    /// Will eventually evolve onto a "MidiEventsProcessor" trait.
    pub fn process_midi(&mut self, midi_message_buffer: &[MidiMessageEntry]) {
        let events = self.midi_converter.accept(midi_message_buffer);
        self.plugin_instance.process_events(events);
    }
}

impl<PluginInstanceT: Plugin> AudioProcessor for TestHostProcessorImpl<PluginInstanceT> {
    type SampleType = f32;

    fn prepare(&mut self, audio_settings: AudioProcessorSettings) {
        log::info!("Prepared TestHostProcessor id={}", self.id);
        self.plugin_instance
            .set_block_size(audio_settings.block_size() as i64);
        self.plugin_instance
            .set_sample_rate(audio_settings.sample_rate() as f32);
        self.audio_settings = audio_settings;
        self.buffer_handler.prepare(&audio_settings);
        if let Some(audio_file_processor) = &mut self.maybe_audio_file_processor {
            audio_file_processor.prepare(audio_settings);
        }
        self.volume_meter_processor.prepare(audio_settings);
        self.running_rms_processor.prepare(audio_settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        output: &mut BufferType,
    ) {
        let num_channels = self.audio_settings.input_channels();

        // Mono the input source
        mono_input_source(self.mono_input, output);

        // Input generation section
        if let Some(audio_file_processor) = &mut self.maybe_audio_file_processor {
            audio_file_processor.process(output);
        }

        // VST processing section
        self.buffer_handler.process(output);
        let mut audio_buffer = self.buffer_handler.get_audio_buffer();
        unsafe {
            let instance =
                self.plugin_instance.deref() as *const PluginInstanceT as *mut PluginInstanceT;
            (*instance).process(&mut audio_buffer);
        }
        flush_vst_output(num_channels, &mut audio_buffer, output);

        let volume = self.handle.volume.get();
        for frame in output.frames_mut() {
            for sample in frame {
                *sample *= volume;
            }
        }

        // Volume meter
        self.volume_meter_processor.process(output);
        self.running_rms_processor.process(output);
    }
}

impl<PluginInstanceT: Plugin> Drop for TestHostProcessorImpl<PluginInstanceT> {
    fn drop(&mut self) {
        log::warn!("Dropping test host processor {}", self.id);
    }
}

/// Flush VST output in `audio_buffer` into `output`
#[allow(clippy::needless_range_loop)]
pub fn flush_vst_output<BufferType: AudioBuffer<SampleType = f32>>(
    _num_channels: usize,
    audio_buffer: &mut vst::buffer::AudioBuffer<f32>,
    output: &mut BufferType,
) {
    let (_, plugin_output) = audio_buffer.split();
    for channel in 0..output.num_channels() {
        let plugin_channel = plugin_output.get(channel);
        for (frame, plugin_frame) in output.frames_mut().zip(plugin_channel) {
            frame[channel] = *plugin_frame;
        }
    }
}

impl<P: Plugin> TestHostProcessorImpl<P> {}

fn mono_input_source<BufferType: AudioBuffer<SampleType = f32>>(
    mono_input: Option<usize>,
    output: &mut BufferType,
) {
    if let Some(mono_input_channel) = mono_input {
        if mono_input_channel >= output.num_channels() {
            return;
        }

        for sample_index in 0..output.num_samples() {
            let source_sample = *output.get(mono_input_channel, sample_index);
            for channel_index in 0..output.num_channels() {
                if channel_index == mono_input_channel {
                    continue;
                }

                output.set(channel_index, sample_index, source_sample);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use std::ops::DerefMut;
    use std::ptr::null;
    use std::sync::atomic::Ordering;

    use basedrop::Owned;
    use mockall::mock;
    use vst::plugin::Plugin;

    use audio_processor_standalone_midi::host::MidiMessageWrapper;
    use audio_processor_traits::{OwnedAudioBuffer, VecAudioBuffer};

    use crate::processors::shared_processor::SharedProcessor;

    use super::*;

    #[cfg(test)]
    mock! {
        PluginInstance {}

        impl Plugin for PluginInstance {
            fn new(callback: vst::plugin::HostCallback) -> Self;
            fn get_info(&self) -> vst::plugin::Info;
            fn set_sample_rate(&mut self, rate: f32);
            fn set_block_size(&mut self, size: i64);
            fn process_events(&mut self, events: &vst::api::Events);
        }
    }

    #[test]
    fn test_create_host() {
        let gc_handle = audio_garbage_collector::handle();
        let plugin_instance = MockPluginInstance::default();
        let plugin = SharedProcessor::new(gc_handle, plugin_instance);
        let sample_rate = 44100.0;
        let host = TestHostProcessorImpl::new(gc_handle, None, plugin, sample_rate, 2, 512, None);
        assert!(!host.id().is_empty());
        assert!(!host.is_playing());
    }

    #[test]
    fn test_set_volume_sets_volume_on_handle() {
        let gc_handle = audio_garbage_collector::handle();
        let plugin_instance = MockPluginInstance::default();
        let plugin = SharedProcessor::new(gc_handle, plugin_instance);
        let sample_rate = 44100.0;
        let host = TestHostProcessorImpl::new(gc_handle, None, plugin, sample_rate, 2, 512, None);

        host.set_volume(0.3);
        assert_f_eq!(host.handle().volume.load(Ordering::Relaxed), 0.3);
        host.set_volume(0.7);
        assert_f_eq!(host.handle().volume.load(Ordering::Relaxed), 0.7);
    }

    #[test]
    fn test_host_prepares_plugin() {
        let gc_handle = audio_garbage_collector::handle();
        let mut plugin_instance = MockPluginInstance::default();

        plugin_instance
            .expect_set_sample_rate()
            .with(mockall::predicate::eq(1000.0))
            .returning(|_| ());
        plugin_instance
            .expect_set_block_size()
            .with(mockall::predicate::eq(64))
            .returning(|_| ());

        let plugin = SharedProcessor::new(gc_handle, plugin_instance);
        let sample_rate = 44100.0;

        let mut host =
            TestHostProcessorImpl::new(gc_handle, None, plugin, sample_rate, 2, 512, None);

        let mut settings = AudioProcessorSettings::default();
        settings.sample_rate = 1000.0;
        settings.block_size = 64;
        host.prepare(settings);
    }

    #[test]
    fn test_host_forwards_midi_events() {
        let gc_handle = audio_garbage_collector::handle();
        let plugin_instance = MockPluginInstance::default();
        let mut plugin = SharedProcessor::new(gc_handle, plugin_instance);
        let sample_rate = 44100.0;

        plugin.deref_mut().expect_process_events().returning(|_| ());

        let mut host =
            TestHostProcessorImpl::new(gc_handle, None, plugin.clone(), sample_rate, 2, 512, None);
        host.process_midi(&[MidiMessageEntry(Owned::new(
            gc_handle,
            MidiMessageWrapper {
                message_data: [0, 1, 2],
                timestamp: 10,
            },
        ))]);
    }

    #[test]
    fn test_flush_vst_output() {
        let inputs: *const *const f32 = null();
        let chan1 = &mut [10.0_f32, 20.0, 30.0] as *mut f32;
        let chan2 = &mut [1.0_f32, 2.0, 3.0] as *mut f32;
        let outputs: *mut *mut f32 = &mut [chan1, chan2] as *mut _;
        let mut source_buffer =
            unsafe { vst::buffer::AudioBuffer::from_raw(2, 2, inputs, outputs, 3) };
        let mut dest_buffer = VecAudioBuffer::new();

        dest_buffer.resize(2, 3, 0.0);
        flush_vst_output(2, &mut source_buffer, &mut dest_buffer);

        assert_f_eq!(*dest_buffer.get(0, 0), 10.0);
        assert_f_eq!(*dest_buffer.get(0, 1), 20.0);
        assert_f_eq!(*dest_buffer.get(0, 2), 30.0);
        assert_f_eq!(*dest_buffer.get(1, 0), 1.0);
        assert_f_eq!(*dest_buffer.get(1, 1), 2.0);
        assert_f_eq!(*dest_buffer.get(1, 2), 3.0);
    }

    #[test]
    fn test_mono_input_source() {
        let mut buffer = VecAudioBuffer::empty_with(2, 3, 0.0);
        buffer.set(0, 0, 1.0);
        buffer.set(1, 0, 10.0);
        buffer.set(0, 1, 2.0);
        buffer.set(1, 1, 20.0);
        buffer.set(0, 2, 3.0);
        buffer.set(1, 2, 30.0);

        mono_input_source(Some(1), &mut buffer);
        assert_f_eq!(*buffer.get(0, 0), 10.0);
        assert_f_eq!(*buffer.get(0, 1), 20.0);
        assert_f_eq!(*buffer.get(0, 2), 30.0);

        assert_f_eq!(*buffer.get(1, 0), 10.0);
        assert_f_eq!(*buffer.get(1, 1), 20.0);
        assert_f_eq!(*buffer.get(1, 2), 30.0);
    }
}
