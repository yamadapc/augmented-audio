use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use symphonia::core::probe::ProbeResult;
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tao::platform::macos::WindowExtMacOS;
use vst::host::{PluginInstance, PluginLoader};
use vst::plugin::Plugin;

use processor::TestHostProcessor;

use crate::commands::main::audio_file_processor::{default_read_audio_file, AudioFileSettings};
use crate::commands::main::audio_settings::AudioSettings;
use crate::commands::options::RunOptions;
use crate::host;

mod audio_file_processor;
mod audio_settings;
mod cpal_vst_buffer_handler;
mod processor;
mod sample_rate_conversion;

struct UnsafePluginRef(*mut PluginInstance);
unsafe impl Send for UnsafePluginRef {}
unsafe impl Sync for UnsafePluginRef {}

/// Audio thread
fn initialize_audio_thread(plugin_instance: *mut PluginInstance, audio_file: ProbeResult) {
    let cpal_host = cpal::default_host();
    log::info!("Using host: {}", cpal_host.id().name());
    let output_device = cpal_host
        .default_output_device()
        .expect("Expected to find output device");
    log::info!("Using device: {}", output_device.name().unwrap());
    let output_config = output_device
        .default_output_config()
        .expect("Expected default output configuration");
    let sample_format = output_config.sample_format();
    let mut output_config: StreamConfig = output_config.into();
    output_config.buffer_size = BufferSize::Fixed(512);

    match sample_format {
        SampleFormat::F32 => unsafe {
            run_main_loop(plugin_instance, &output_device, &output_config, audio_file)
        },
        _ => {
            panic!("Unsupported sample format from device.")
        }
    }
}

unsafe fn run_main_loop(
    plugin_instance: *mut PluginInstance,
    output_device: &cpal::Device,
    output_config: &cpal::StreamConfig,
    audio_file: ProbeResult,
) {
    let buffer_size = match output_config.buffer_size {
        BufferSize::Default => panic!("Using default buffer size will cause reliability issues"),
        BufferSize::Fixed(buffer_size) => buffer_size,
    };

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    let instance = plugin_instance.as_mut().unwrap();
    instance.suspend();
    instance.set_sample_rate(sample_rate);
    instance.resume();

    log::info!("Buffer size {:?}", buffer_size);
    let audio_file_settings = AudioFileSettings::new(audio_file);
    let mut processor = TestHostProcessor::new(
        audio_file_settings,
        plugin_instance,
        sample_rate,
        channels,
        buffer_size,
    );
    let audio_settings = AudioSettings::new(sample_rate, channels, buffer_size);
    processor.prepare(audio_settings);

    let stream = output_device
        .build_output_stream(
            output_config,
            move |data: &mut [f32], output_info: &cpal::OutputCallbackInfo| {
                processor.cpal_process(data, output_info);
            },
            TestHostProcessor::cpal_error,
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to play output stream");

    std::thread::park();
}

fn start_gui(instance: *mut PluginInstance) {
    let event_loop = tao::event_loop::EventLoop::new();
    let window = tao::window::Window::new(&event_loop).expect("Failed to create editor window");
    unsafe {
        window.set_title(&(*instance).get_info().name);
    }

    let mut editor = unsafe { instance.as_mut() }
        .unwrap()
        .get_editor()
        .expect("Plugin has no editor");
    editor.open(window.ns_view());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}

pub fn run_test(run_options: RunOptions) {
    let host = Arc::new(Mutex::new(host::AudioTestHost));

    let path = Path::new(run_options.plugin_path());
    log::info!("Loading VST from: {}...", path.to_str().unwrap());
    let mut loader = PluginLoader::load(path, Arc::clone(&host))
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e));

    log::info!("Creating plugin instance...");
    let mut instance = loader.instance().unwrap();
    let info = instance.get_info();
    log::info!(
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
    log::info!("Initialized instance!");

    log::info!("Initializing audio thread");
    let audio_thread = {
        let instance = UnsafePluginRef(&mut instance as *mut PluginInstance);
        let input_audio_path = run_options.input_audio().to_string();

        thread::spawn(move || {
            let instance = instance.0;
            let audio_file = default_read_audio_file(&input_audio_path);
            initialize_audio_thread(instance, audio_file);
        })
    };

    let instance = &mut instance as *mut PluginInstance;
    if run_options.open_editor() {
        log::info!("Starting GUI");
        start_gui(instance);
    }

    if let Err(err) = audio_thread.join() {
        log::error!(
            "Failed to join audio thread. There may be issues terminating the command. Error: {:?}",
            err
        );
    }
    log::info!("Closing instance...");
}
