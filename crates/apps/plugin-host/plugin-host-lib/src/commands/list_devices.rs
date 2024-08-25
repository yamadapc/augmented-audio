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
use std::error::Error;

use cpal::traits::{DeviceTrait, HostTrait};

pub fn run_list_devices() {
    let hosts = cpal::available_hosts();
    hosts.iter().for_each(|host_id| {
        if print_host_devices(host_id).is_err() {
            log::error!("Error listing devices for host {}", host_id.name());
        }
    });
}

fn print_host_devices(host_id: &cpal::HostId) -> Result<(), Box<dyn Error>> {
    let host = cpal::host_from_id(*host_id)?;

    for device in host.input_devices()? {
        let name = device.name()?;
        let config = device.default_input_config()?;
        let num_channels = config.channels();
        log::info!(
            "{} (INPUT): \"{}\" - channels={}",
            host.id().name(),
            name,
            num_channels
        );
    }

    for device in host.output_devices()? {
        let name = device.name()?;
        let config = device.default_output_config()?;
        let num_channels = config.channels();
        log::info!(
            "{} (OUTPUT): \"{}\" - channels={}",
            host.id().name(),
            name,
            num_channels
        );
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::commands::run_list_devices;

    #[skip]
    #[test]
    fn test_list_devices_does_not_panic() {
        run_list_devices();
    }
}
