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

//! Option negotiation for CPAL streams.
//!
//! This module configures and creates CPAL devices following input options.

use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, DefaultStreamConfigError, Device, DevicesError, Host, SampleRate, StreamConfig,
    SupportedStreamConfig,
};

use crate::standalone_processor::StandaloneOptions;

#[derive(Clone, Copy)]
pub enum AudioIOMode {
    Input,
    Output,
}

fn list_devices<Host: HostTrait>(
    host: &Host,
    mode: AudioIOMode,
) -> Result<impl Iterator<Item = Host::Device>, DevicesError> {
    match mode {
        AudioIOMode::Input => host.input_devices(),
        AudioIOMode::Output => host.output_devices(),
    }
}

fn supported_configs(
    device: &impl DeviceTrait,
    mode: AudioIOMode,
) -> Result<Vec<cpal::SupportedStreamConfigRange>, cpal::SupportedStreamConfigsError> {
    match mode {
        AudioIOMode::Input => device.supported_input_configs().map(|i| i.collect()),
        AudioIOMode::Output => device.supported_output_configs().map(|i| i.collect()),
    }
}

fn device_name(options: &StandaloneOptions, mode: AudioIOMode) -> Option<&String> {
    match mode {
        AudioIOMode::Input => options.input_device.as_ref(),
        AudioIOMode::Output => options.output_device.as_ref(),
    }
}

fn default_device(host: &Host, mode: AudioIOMode) -> Option<Device> {
    match mode {
        AudioIOMode::Input => host.default_input_device(),
        AudioIOMode::Output => host.default_output_device(),
    }
}

fn default_config(
    device: &Device,
    mode: AudioIOMode,
) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
    match mode {
        AudioIOMode::Input => device.default_input_config(),
        AudioIOMode::Output => device.default_output_config(),
    }
}

fn configure_device(
    host: &Host,
    options: &StandaloneOptions,
    mode: AudioIOMode,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let device_name = device_name(&options, mode);
    let device = device_name
        .map(|device_name| {
            let mut devices = list_devices(host, mode).unwrap();
            devices.find(|device| matches!(device.name(), Ok(name) if &name == device_name))
        })
        .flatten()
        .unwrap_or_else(|| default_device(host, mode).unwrap());
    let supported_configs = supported_configs(&device, mode).unwrap();
    let mut supports_stereo = false;
    for config in supported_configs {
        log::debug!("  Supported config: {:?}", config);
        if config.channels() > 1 {
            supports_stereo = true;
        }
    }

    let config = default_config(&device, mode).unwrap();
    let mut config: StreamConfig = config.into();
    config.channels = if supports_stereo { 2 } else { 1 };
    config.sample_rate = SampleRate(sample_rate as u32);
    config.buffer_size = BufferSize::Fixed(buffer_size as u32);

    #[cfg(target_os = "ios")]
    {
        config.buffer_size = BufferSize::Default;
    }

    (device, config)
}

pub fn configure_input_device(
    host: &Host,
    options: &StandaloneOptions,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let (input_device, input_config) =
        configure_device(host, options, AudioIOMode::Input, buffer_size, sample_rate);
    log::info!(
        "Using input name={} sample_rate={} buffer_size={:?}",
        input_device.name().unwrap(),
        sample_rate,
        input_config.buffer_size
    );
    (input_device, input_config)
}

pub fn configure_output_device(
    host: Host,
    options: &StandaloneOptions,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let (output_device, output_config) = configure_device(
        &host,
        options,
        AudioIOMode::Output,
        buffer_size,
        sample_rate,
    );
    log::info!(
        "Using output name={} sample_rate={} buffer_size={:?}",
        output_device.name().unwrap(),
        sample_rate,
        output_config.buffer_size
    );
    (output_device, output_config)
}

#[cfg(test)]
mod test {
    use crate::standalone_cpal::mock_cpal::vec_iterator::VecIterator;
    use crate::standalone_cpal::mock_cpal::*;

    use super::*;

    #[test]
    fn test_none_device_name() {
        let options = StandaloneOptions::default();
        assert!(device_name(&options, AudioIOMode::Input).is_none());
        assert!(device_name(&options, AudioIOMode::Output).is_none());
    }

    #[test]
    fn test_device_name_for_input() {
        let options = StandaloneOptions {
            input_device: Some("input-name".to_string()),
            ..StandaloneOptions::default()
        };
        let name = device_name(&options, AudioIOMode::Input);
        assert_eq!(name.unwrap(), "input-name");
    }

    #[test]
    fn test_device_name_for_output() {
        let options = StandaloneOptions {
            output_device: Some("output-name".to_string()),
            ..StandaloneOptions::default()
        };
        let name = device_name(&options, AudioIOMode::Output);
        assert_eq!(name.unwrap(), "output-name");
    }

    #[test]
    fn test_list_devices_calls_host_input_devices() {
        let mut host = MockHost::default();
        host.expect_input_devices().returning(|| {
            let mock_devices = vec![MockDevice::default()];

            Ok(VecIterator::from(mock_devices).filter(|_| true))
        });
        let result = list_devices(&host, AudioIOMode::Input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_devices_calls_host_output_devices() {
        let mut host = MockHost::default();
        host.expect_output_devices().returning(|| {
            let mock_devices = vec![MockDevice::default()];

            Ok(VecIterator::from(mock_devices).filter(|_| true))
        });
        let result = list_devices(&host, AudioIOMode::Output);
        assert!(result.is_ok());
    }

    #[test]
    fn supported_configs_works_for_input_devices() {
        let mut device = MockDevice::default();
        device
            .expect_supported_input_configs()
            .returning(|| Ok(VecIterator::from(vec![])));

        let result = supported_configs(&device, AudioIOMode::Input);
        assert!(result.is_ok());
    }

    #[test]
    fn supported_configs_works_for_output_devices() {
        let mut device = MockDevice::default();
        device
            .expect_supported_output_configs()
            .returning(|| Ok(VecIterator::from(vec![])));

        let result = supported_configs(&device, AudioIOMode::Output);
        assert!(result.is_ok());
    }
}
