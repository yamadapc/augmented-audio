use cpal::traits::{DeviceTrait, HostTrait};
use std::error::Error;

pub fn run_list_devices() {
    let hosts = cpal::available_hosts();
    hosts
        .iter()
        .for_each(|host_id| match print_host_devices(host_id) {
            Err(_) => {
                println!("Error listing devices for host {}", host_id.name());
            }
            _ => {}
        });
}

fn print_host_devices(host_id: &cpal::HostId) -> Result<(), Box<dyn Error>> {
    let host = cpal::host_from_id(*host_id)?;

    for device in host.input_devices()? {
        let name = device.name()?;
        let config = device.default_input_config()?;
        let num_channels = config.channels();
        println!(
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
        println!(
            "{} (OUTPUT): \"{}\" - channels={}",
            host.id().name(),
            name,
            num_channels
        );
    }

    Ok(())
}
