mod host;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::env;
use std::path::Path;
use std::process;
use std::sync::{Arc, Mutex};
use vst::host::PluginLoader;
use vst::plugin::Plugin;

/**
 * Start cpal processing thread
 */
fn initialize_main_loop() {
    let cpal_host = cpal::default_host();
    let output_device = cpal_host
        .default_output_device()
        .expect("Expected to find output device");
    let config = output_device
        .default_input_config()
        .expect("Expected default input configuration");
    match config.sample_format() {
        SampleFormat::F32 => run_main_loop(&output_device, &config.into()),
        _ => {
            panic!("What's going on")
        }
    }
}

fn run_main_loop(device: &cpal::Device, config: &cpal::StreamConfig) {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    // let mut sample_clock = 0f32;
    let mut oscillator =
        oscillator::Oscillator::new_with_sample_rate(sample_rate, |phase: f32| phase.sin());
    let mut next_value = move || {
        oscillator.set_frequency(oscillator.get_frequency() * 1.000001);
        oscillator.next()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to play output stream");

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: simple_host path/to/vst");
        process::exit(1);
    }

    let path = Path::new(&args[1]);

    // Create the host
    let host = Arc::new(Mutex::new(host::SampleHost));

    println!("Loading {}...", path.to_str().unwrap());

    // Load the plugin
    let mut loader = PluginLoader::load(path, Arc::clone(&host))
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e));

    // Create an instance of the plugin
    let mut instance = loader.instance().unwrap();

    // Get the plugin information
    let info = instance.get_info();

    println!(
        "Loaded '{}':\n\t\
         Vendor: {}\n\t\
         Presets: {}\n\t\
         Parameters: {}\n\t\
         VST ID: {}\n\t\
         Version: {}\n\t\
         Initial Delay: {} samples",
        info.name,
        info.vendor,
        info.presets,
        info.parameters,
        info.unique_id,
        info.version,
        info.initial_delay
    );

    // Initialize the instance
    instance.init();
    println!("Initialized instance!");

    initialize_main_loop();

    println!("Closing instance...");
    // Close the instance. This is not necessary as the instance is shut down when
    // it is dropped as it goes out of scope.
    // drop(instance);
}
