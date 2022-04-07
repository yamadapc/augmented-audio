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
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

use crate::audio_io::audio_thread::error::AudioThreadError;
use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId, AudioThreadOptions};

fn find_cpal_host_by_name(host_name: &str) -> Option<cpal::Host> {
    let maybe_id = cpal::available_hosts()
        .into_iter()
        .find(|host| host.name() == host_name);
    maybe_id.and_then(|host_id| cpal::host_from_id(host_id).ok())
}

fn find_cpal_output_device_by_name(host: &Host, id: &str) -> Option<Device> {
    host.output_devices().ok().and_then(|mut devices| {
        devices.find(|device| {
            let name = device.name();
            match name {
                Ok(name) => name == id,
                Err(_) => false,
            }
        })
    })
}

fn find_cpal_input_device_by_name(host: &Host, id: &str) -> Option<Device> {
    host.input_devices().ok().and_then(|mut devices| {
        devices.find(|device| {
            let name = device.name();
            log::info!("Looking for {} in {:?}", id, &name);
            match name {
                Ok(name) => name == id,
                Err(_) => false,
            }
        })
    })
}

pub fn get_cpal_host(host_id: &AudioHostId) -> cpal::Host {
    match &host_id {
        AudioHostId::Default => cpal::default_host(),
        AudioHostId::Id(id) => find_cpal_host_by_name(id).unwrap_or_else(cpal::default_host),
    }
}

pub fn get_cpal_output_device(
    host: &cpal::Host,
    output_device_id: &AudioDeviceId,
) -> Result<cpal::Device, AudioThreadError> {
    let maybe_device = match &output_device_id {
        AudioDeviceId::Default => host.default_output_device(),
        AudioDeviceId::Id(id) => find_cpal_output_device_by_name(host, id),
    };

    match maybe_device {
        Some(device) => Ok(device),
        None => Err(AudioThreadError::OutputDeviceNotFoundError),
    }
}

pub fn get_output_config(
    options: &AudioThreadOptions,
    output_device: &cpal::Device,
) -> Result<cpal::StreamConfig, AudioThreadError> {
    let output_config = output_device.default_output_config()?;
    let sample_format = output_config.sample_format();
    let mut output_config: cpal::StreamConfig = output_config.into();
    output_config.buffer_size = options.buffer_size.clone().into();
    output_config.channels = options.num_channels as u16;
    output_config.sample_rate = options.sample_rate;

    if sample_format != cpal::SampleFormat::F32 {
        return Err(AudioThreadError::UnsupportedSampleFormat);
    }
    Ok(output_config)
}

pub fn get_cpal_input_device(
    host: &cpal::Host,
    input_device_id: &AudioDeviceId,
) -> Result<cpal::Device, AudioThreadError> {
    let maybe_device = match &input_device_id {
        AudioDeviceId::Default => host.default_input_device(),
        AudioDeviceId::Id(id) => find_cpal_input_device_by_name(host, id),
    };

    match maybe_device {
        Some(device) => Ok(device),
        None => Err(AudioThreadError::OutputDeviceNotFoundError),
    }
}

pub fn get_input_config(
    options: &AudioThreadOptions,
    input_device: &cpal::Device,
) -> Result<cpal::StreamConfig, AudioThreadError> {
    let input_config = input_device.default_input_config()?;
    let sample_format = input_config.sample_format();
    let mut input_config: cpal::StreamConfig = input_config.into();
    input_config.buffer_size = options.buffer_size.clone().into();
    input_config.channels = options.num_channels as u16;
    input_config.sample_rate = options.sample_rate;

    if sample_format != cpal::SampleFormat::F32 {
        return Err(AudioThreadError::UnsupportedSampleFormat);
    }
    Ok(input_config)
}
