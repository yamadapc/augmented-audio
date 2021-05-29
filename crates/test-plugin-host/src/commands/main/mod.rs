mod audio_settings;
mod cpal_vst_buffer_handler;
mod processor;

use crate::commands::options::RunOptions;
use crate::host;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use processor::TestHostProcessor;
use std::default::Default;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, ProbeResult};
use vst::host::{HostBuffer, PluginInstance, PluginLoader};
use vst::plugin::Plugin;

/// Audio thread
unsafe fn initialize_audio_thread(plugin_instance: PluginInstance, audio_file: ProbeResult) {
    let cpal_host = cpal::default_host();
    println!("Using host: {}", cpal_host.id().name());
    let output_device = cpal_host
        .default_output_device()
        .expect("Expected to find output device");
    println!("Using device: {}", output_device.name().unwrap());
    let input_config = output_device
        .default_input_config()
        .expect("Expected default input configuration");
    let sample_format = input_config.sample_format();
    let mut input_config: StreamConfig = input_config.into();
    input_config.buffer_size = BufferSize::Fixed(512);

    match sample_format {
        SampleFormat::F32 => {
            run_main_loop(plugin_instance, &output_device, &input_config, audio_file)
        }
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
    input_config: &cpal::StreamConfig,
    mut audio_file: ProbeResult,
) {
    let buffer_size = match input_config.buffer_size {
        BufferSize::Default => panic!("Using default buffer size will cause reliability issues"),
        BufferSize::Fixed(buffer_size) => buffer_size,
    };
    let audio_file_stream = audio_file
        .format
        .default_stream()
        .expect("Failed to open audio file stream");
    let mut decoder = symphonia::default::get_codecs()
        .make(&audio_file_stream.codec_params, &Default::default())
        .expect("Failed to get input file codec");

    // let mut next_audio_file_buffer = move || {
    //     let packet = audio_file.format.next_packet().ok()?;
    //     if packet.stream_id() != audio_file_stream.id {
    //         return None;
    //     }
    //
    //     let decoded = decoder.decode(&packet).ok()?;
    //     None
    // };
    // let mut current_buffer = None;
    // let mut current_buffer_position = 0;
    // let next_audio_file_sample = move || {
    //     // if current_buffer.is_none() {
    //     //     current_buffer = next_audio_file_buffer();
    //     // }
    //
    //     if let Some(AudioBufferRef::F32(buffer)) = current_buffer {
    //         let left_channel = buffer.chan(0);
    //         let sample = left_channel[current_buffer_position];
    //         current_buffer_position += 1;
    //         return sample;
    //     }
    //
    //     0.0
    // };

    let sample_rate = input_config.sample_rate.0 as f32;
    let channels = input_config.channels as usize;

    plugin_instance.suspend();
    plugin_instance.set_sample_rate(sample_rate);
    plugin_instance.resume();

    println!("Buffer size {:?}", buffer_size);
    let mut processor = TestHostProcessor::new(plugin_instance, sample_rate, channels, buffer_size);

    // let mut oscillator = oscillator::Oscillator::sine(sample_rate);
    // oscillator.set_frequency(440.0);
    // let mut next_value = move || oscillator.next_sample();
    // let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = output_device
        .build_output_stream(
            input_config,
            move |data: &mut [f32], output_info: &cpal::OutputCallbackInfo| {
                processor.cpal_process(data, output_info);
            },
            move |err| TestHostProcessor::cpal_error(err),
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

fn start_gui() {}

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

    let mut hint = Hint::new();
    let media_source = {
        let audio_input_path = Path::new(run_options.input_audio());
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }
        }
        Box::new(File::open(path).unwrap())
    };
    let audio_file = MediaSourceStream::new(media_source, Default::default());
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let audio_file = match symphonia::default::get_probe().format(
        &hint,
        audio_file,
        &format_opts,
        &metadata_opts,
    ) {
        Ok(mut probed) => probed,
        Err(err) => {
            eprintln!("ERROR: Input file not supported: {}", err);
            exit(1);
        }
    };

    unsafe {
        initialize_audio_thread(instance, audio_file);
    }

    start_gui();

    println!("Closing instance...");
}
