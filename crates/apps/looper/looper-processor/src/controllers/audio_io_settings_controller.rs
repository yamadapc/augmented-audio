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
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;

pub struct AudioDevice {
    pub name: String,
}

#[derive(Default)]
pub struct AudioIOSettingsController {}

impl AudioIOSettingsController {
    pub fn list_input_devices(&self) -> Result<Vec<AudioDevice>> {
        let host = cpal::default_host();
        let devices = host.input_devices()?;
        Self::build_domain_model(devices)
    }

    pub fn list_output_devices(&self) -> Result<Vec<AudioDevice>> {
        let host = cpal::default_host();
        let devices = host.output_devices()?;
        Self::build_domain_model(devices)
    }

    fn build_domain_model(devices: impl Iterator<Item = Device>) -> Result<Vec<AudioDevice>> {
        let mut result = vec![];

        for device in devices {
            let name = device.name()?;
            result.push(AudioDevice { name });
        }

        Ok(result)
    }
}
