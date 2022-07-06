use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BuildStreamError, Data, DefaultStreamConfigError, DeviceNameError, DevicesError,
    InputCallbackInfo, OutputCallbackInfo, PauseStreamError, PlayStreamError, SampleFormat,
    SampleRate, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange, SupportedStreamConfigsError,
};

use crate::standalone_cpal::mock_cpal::vec_iterator::VecIterator;

const DEFAULT_SAMPLE_RATE: u32 = 44100;

struct VirtualHost {
    devices: Vec<VirtualHostDevice>,
}

impl Default for VirtualHost {
    fn default() -> Self {
        Self { devices: vec![] }
    }
}

impl HostTrait for VirtualHost {
    type Devices = VecIterator<VirtualHostDevice>;
    type Device = VirtualHostDevice;

    fn is_available() -> bool {
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        Ok(VecIterator::from(self.devices.clone()))
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        self.devices.get(0).cloned()
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        self.devices.get(1).cloned()
    }
}

#[derive(Clone)]
struct VirtualHostDevice {
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
        todo!()
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
        todo!()
    }
}

struct VirtualHostStream {}

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
        let processor = BufferProcessor(NoopAudioProcessor::default());
        let processor = StandaloneAudioOnlyProcessor::new(processor, Default::default());

        let _handles = standalone_start_with(
            processor,
            StandaloneStartOptions {
                ..StandaloneStartOptions::default()
            },
        );
    }
}
