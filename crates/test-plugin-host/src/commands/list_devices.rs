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
