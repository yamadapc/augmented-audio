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

//! The long-term goal of this module is to provide a full mocked environment
//! which exposes one input device and one output device.
//!
//! In this mocked environment, extra APIs would allow a consumer to push buffers
//! into an input buffer queue and read buffers that have been processed by
//! the output callbacks.
//!
//! This would be used for integration testing.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BuildStreamError, Data, DefaultStreamConfigError, DeviceNameError, DevicesError,
    InputCallbackInfo, OutputCallbackInfo, PauseStreamError, PlayStreamError, SampleFormat,
    SampleRate, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange, SupportedStreamConfigsError,
};

use crate::standalone_cpal::mock_cpal::vec_iterator::VecIterator;

const DEFAULT_SAMPLE_RATE: u32 = 44100;

#[derive(Default)]
pub struct VirtualHost {
    input_device: VirtualHostDevice,
    output_device: VirtualHostDevice,
}

impl HostTrait for VirtualHost {
    type Devices = VecIterator<VirtualHostDevice>;
    type Device = VirtualHostDevice;

    fn is_available() -> bool {
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        Ok(VecIterator::from(vec![
            self.input_device.clone(),
            self.output_device.clone(),
        ]))
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        Some(self.input_device.clone())
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        Some(self.output_device.clone())
    }
}

#[derive(Clone)]
pub struct VirtualHostDevice {
    name: String,
    supported_input_configs: Vec<SupportedStreamConfigRange>,
    supported_output_configs: Vec<SupportedStreamConfigRange>,
    default_input_config: SupportedStreamConfig,
    default_output_config: SupportedStreamConfig,
}

impl Default for VirtualHostDevice {
    fn default() -> Self {
        Self {
            name: "Test device".to_string(),
            supported_input_configs: vec![SupportedStreamConfigRange::new(
                2,
                SampleRate(DEFAULT_SAMPLE_RATE),
                SampleRate(DEFAULT_SAMPLE_RATE),
                SupportedBufferSize::Range { min: 512, max: 512 },
                SampleFormat::F32,
            )],
            supported_output_configs: vec![SupportedStreamConfigRange::new(
                2,
                SampleRate(DEFAULT_SAMPLE_RATE),
                SampleRate(DEFAULT_SAMPLE_RATE),
                SupportedBufferSize::Range { min: 512, max: 512 },
                SampleFormat::F32,
            )],
            default_input_config: SupportedStreamConfig::new(
                2,
                SampleRate(DEFAULT_SAMPLE_RATE),
                SupportedBufferSize::Range { min: 512, max: 512 },
                SampleFormat::F32,
            ),
            default_output_config: SupportedStreamConfig::new(
                2,
                SampleRate(DEFAULT_SAMPLE_RATE),
                SupportedBufferSize::Range { min: 512, max: 512 },
                SampleFormat::F32,
            ),
        }
    }
}

impl DeviceTrait for VirtualHostDevice {
    type SupportedInputConfigs = VecIterator<SupportedStreamConfigRange>;
    type SupportedOutputConfigs = VecIterator<SupportedStreamConfigRange>;
    type Stream = VirtualHostStream;

    fn name(&self) -> Result<String, DeviceNameError> {
        Ok(self.name.clone())
    }

    fn supported_input_configs(
        &self,
    ) -> Result<Self::SupportedInputConfigs, SupportedStreamConfigsError> {
        Ok(VecIterator::from(self.supported_input_configs.clone()))
    }

    fn supported_output_configs(
        &self,
    ) -> Result<Self::SupportedOutputConfigs, SupportedStreamConfigsError> {
        Ok(VecIterator::from(self.supported_output_configs.clone()))
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        Ok(self.default_input_config.clone())
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        Ok(self.default_output_config.clone())
    }

    fn build_input_stream_raw<D, E>(
        &self,
        _config: &StreamConfig,
        _sample_format: SampleFormat,
        _data_callback: D,
        _error_callback: E,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        Ok(VirtualHostStream::default())
    }

    fn build_output_stream_raw<D, E>(
        &self,
        _config: &StreamConfig,
        _sample_format: SampleFormat,
        _data_callback: D,
        _error_callback: E,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        Ok(VirtualHostStream::default())
    }
}

#[derive(Default)]
pub struct VirtualHostStream {}

impl StreamTrait for VirtualHostStream {
    fn play(&self) -> Result<(), PlayStreamError> {
        Ok(())
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use audio_processor_traits::{BufferProcessor, NoopAudioProcessor};

    use crate::{standalone_start_with, StandaloneAudioOnlyProcessor, StandaloneStartOptions};

    use super::*;

    #[test]
    fn test_create_virtual_host() {
        let _host = VirtualHost::default();
    }

    #[test]
    fn test_create_virtual_device() {
        let _device = VirtualHostDevice::default();
    }

    #[test]
    fn test_run_virtual_host_with_standalone_run() {
        let host = VirtualHost::default();
        let processor = BufferProcessor(NoopAudioProcessor::default());
        let processor = StandaloneAudioOnlyProcessor::new(processor, Default::default());

        let _handles = standalone_start_with::<
            StandaloneAudioOnlyProcessor<BufferProcessor<NoopAudioProcessor<f32>>>,
            VirtualHost,
        >(
            processor,
            StandaloneStartOptions {
                host,
                host_name: "VirtualHost".to_string(),
                handle: Some(audio_garbage_collector::handle().clone()),
            },
        );
    }
}
