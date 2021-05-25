use crate::commands::options::RunOptions;
use crate::host;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::path::Path;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use vst::buffer::AudioBuffer;
use vst::host::{HostBuffer, PluginInstance, PluginLoader};
use vst::plugin::Plugin;

/// Audio thread
unsafe fn initialize_main_loop(plugin_instance: PluginInstance) {
    let cpal_host = cpal::default_host();
    println!("Using host: {}", cpal_host.id().name());
    let output_device = cpal_host
        .default_output_device()
        .expect("Expected to find output device");
    println!("Using device: {}", output_device.name().unwrap());
    let config = output_device
        .default_input_config()
        .expect("Expected default input configuration");

    match config.sample_format() {
        SampleFormat::F32 => run_main_loop(plugin_instance, &output_device, &config.into()),
        _ => {
            panic!("Unsupported sample format from device.")
        }
    }
}

struct UnsafePluginRef(Arc<Mutex<PluginInstance>>);

unsafe impl Send for UnsafePluginRef {}
unsafe impl Sync for UnsafePluginRef {}

unsafe fn run_main_loop(
    mut plugin_instance: PluginInstance,
    output_device: &cpal::Device,
    config: &cpal::StreamConfig,
) {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    plugin_instance.set_sample_rate(sample_rate);
    plugin_instance.start_process();
    let mut plugin_ref = UnsafePluginRef(Arc::new(Mutex::new(plugin_instance)));

    let mut oscillator = oscillator::Oscillator::sine(sample_rate);
    oscillator.set_frequency(440.0);
    let mut next_value = move || oscillator.next();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = output_device
        .build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value, &mut plugin_ref)
            },
            err_fn,
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to play output stream");

    std::thread::sleep(std::time::Duration::from_millis(50000));
}

unsafe fn write_data<'a>(
    output: &mut [f32],
    channels: usize,
    next_sample: &mut dyn FnMut() -> f32,
    plugin_ref: &mut UnsafePluginRef,
) {
    let mut input_buffer = Vec::new();
    let mut output_buffer = Vec::new();

    for _channel in 0..channels {
        input_buffer.push(Vec::new());
        output_buffer.push(Vec::new());
    }

    for frame in output.chunks_mut(channels) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let value = next_sample();
            *sample = value;
            input_buffer.get_unchecked_mut(channel).push(*sample);
            output_buffer.get_unchecked_mut(channel).push(*sample);
        }
    }

    let mut buffer = HostBuffer::new(channels, channels);
    let mut audio_buffer = buffer.bind(&input_buffer, &mut output_buffer);
    plugin_ref.0.lock().unwrap().process(&mut audio_buffer);

    let (_, plugin_output) = audio_buffer.split();
    for (sample_index, frame) in output.chunks_mut(channels).enumerate() {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let channel_out = plugin_output.get(channel);
            let value = channel_out.get(sample_index).unwrap();
            *sample = *value;
        }
    }
}

pub fn run_test(run_options: RunOptions) {
    let host = Arc::new(Mutex::new(host::AudioTestHost));

    let path = Path::new(run_options.plugin_path());
    println!("Loading VST from: {}...", path.to_str().unwrap());
    let mut loader = PluginLoader::load(path, Arc::clone(&host))
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e));

    println!("Creating plugin instance...");
    let mut instance = loader.instance().unwrap();
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

    unsafe {
        initialize_main_loop(instance);
    }

    println!("Closing instance...");
    // Close the instance. This is not necessary as the instance is shut down when
    // it is dropped as it goes out of scope.
    // drop(instance);
}
