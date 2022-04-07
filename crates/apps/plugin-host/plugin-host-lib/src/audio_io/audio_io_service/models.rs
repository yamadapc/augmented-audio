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
use cpal::traits::DeviceTrait;
use cpal::Device;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AudioHost = String;

#[derive(Clone, Serialize, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleRate(pub u32);

#[derive(Clone, Serialize, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferSize(pub u32);

#[derive(Error, Debug)]
pub enum BuildAudioDeviceError {
    #[error("Failed to get device name")]
    Name(#[from] cpal::DeviceNameError),
    #[error("Failed to get device input config")]
    SupportedInputConfigs(#[from] cpal::SupportedStreamConfigsError),
    #[error("Failed to get supported sample rates")]
    NoSampleRates,
}

#[derive(Clone, Serialize, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
    pub name: String,
    pub sample_rate_range: (SampleRate, SampleRate),
    pub buffer_size_range: Option<(BufferSize, BufferSize)>,
}

impl AudioDevice {
    pub fn from_device(device: Device) -> Result<Self, BuildAudioDeviceError> {
        let name = device.name()?;
        let configs = device.supported_input_configs()?;
        let sample_rate_range = configs
            .map(|config| (config.min_sample_rate().0, config.max_sample_rate().0))
            .sorted()
            .take(1)
            .map(|(min, max)| (SampleRate(min), SampleRate(max)))
            .find(|_| true)
            .unwrap_or((SampleRate(0), SampleRate(0)));
        let configs = device.supported_input_configs()?;
        let buffer_size_range = configs
            .filter_map(|config| match config.buffer_size() {
                cpal::SupportedBufferSize::Range { min, max } => Some((*min, *max)),
                cpal::SupportedBufferSize::Unknown => None,
            })
            .sorted()
            .take(1)
            .map(|(min, max)| (BufferSize(min), BufferSize(max)))
            .find(|_| true);

        Ok(AudioDevice {
            name,
            sample_rate_range,
            buffer_size_range,
        })
    }

    pub fn sample_rates(&self) -> Vec<SampleRate> {
        let mut result = vec![];
        let mut current_rate = self.sample_rate_range.0 .0;
        while current_rate <= self.sample_rate_range.1 .0 {
            result.push(SampleRate(current_rate));
            current_rate *= 2;
        }
        result
    }
}

#[derive(Serialize, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesList {
    pub input_devices: Vec<AudioDevice>,
    pub output_devices: Vec<AudioDevice>,
}

impl DevicesList {
    pub fn new(input_devices: Vec<AudioDevice>, output_devices: Vec<AudioDevice>) -> Self {
        DevicesList {
            input_devices,
            output_devices,
        }
    }
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioIOState {
    pub host: AudioHost,
    pub input_device: Option<AudioDevice>,
    pub output_device: Option<AudioDevice>,
}
