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
    BufferSize, Device, Host, SampleRate, StreamConfig,
};

use crate::standalone_processor::StandaloneOptions;

pub fn configure_input_device(
    host: &Host,
    options: &StandaloneOptions,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let input_device = options
        .input_device
        .as_ref()
        .map(|device_name| {
            let mut input_devices = host.input_devices().unwrap();
            input_devices.find(|device| matches!(device.name(), Ok(name) if &name == device_name))
        })
        .flatten()
        .unwrap_or_else(|| host.default_input_device().unwrap());
    let supported_configs = input_device.supported_input_configs().unwrap();
    let mut supports_stereo = false;
    for config in supported_configs {
        log::info!("  INPUT Supported config: {:?}", config);
        if config.channels() > 1 {
            supports_stereo = true;
        }
    }

    let input_config = input_device.default_input_config().unwrap();
    let mut input_config: StreamConfig = input_config.into();
    input_config.channels = if supports_stereo { 2 } else { 1 };
    input_config.sample_rate = SampleRate(sample_rate as u32);
    input_config.buffer_size = BufferSize::Fixed(buffer_size as u32);

    #[cfg(target_os = "ios")]
    {
        input_config.buffer_size = BufferSize::Default;
    }

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
    let output_device = options
        .input_device
        .as_ref()
        .map(|device_name| {
            let mut output_devices = host.output_devices().unwrap();
            output_devices.find(|device| matches!(device.name(), Ok(name) if &name == device_name))
        })
        .flatten()
        .unwrap_or_else(|| host.default_output_device().unwrap());
    for config in output_device.supported_output_configs().unwrap() {
        log::info!("  OUTPUT Supported config: {:?}", config);
    }
    let output_config = output_device.default_output_config().unwrap();
    let mut output_config: StreamConfig = output_config.into();
    output_config.channels = output_device
        .supported_output_configs()
        .unwrap()
        .map(|config| config.channels())
        .max()
        .unwrap_or(2)
        .min(2);
    output_config.sample_rate = SampleRate(sample_rate as u32);
    output_config.buffer_size = BufferSize::Fixed(buffer_size as u32);

    #[cfg(target_os = "ios")]
    {
        output_config.buffer_size = BufferSize::Default;
    }

    log::info!(
        "Using output name={} sample_rate={} buffer_size={:?}",
        output_device.name().unwrap(),
        sample_rate,
        output_config.buffer_size
    );
    (output_device, output_config)
}
