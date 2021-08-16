use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use notify::{watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use crate::audio_io::audio_thread::options::{
    AudioDeviceId, AudioHostId, AudioThreadOptions, BufferSize,
};
use crate::audio_io::audio_thread::AudioThread;
use crate::audio_io::offline_renderer::OfflineRenderer;
use crate::audio_io::test_plugin_host::TestPluginHost;
use crate::commands::options::RunOptions;
use crate::processors::shared_processor::SharedProcessor;
use audio_processor_traits::AudioProcessorSettings;
use std::thread::JoinHandle;

mod file_watch;

/// Entry-point for the run plug-in command. Mostly kicks-off other work:
///
/// * Parses options
/// * Creates the host, audio and other threads
/// * Loads the audio-file (blocking before starting the plug-in)
/// * Loads the audio-plugin
/// * Creates a window for the plug-in & blocks on it (if specified)
/// * Otherwise parks the current thread forever
pub fn run_test(run_options: RunOptions) {
    if run_options.output_audio().is_some() {
        run_offline_rendering(run_options);
        return;
    }

    let (audio_settings, audio_thread_options) = get_audio_options(&run_options);
    let mut host = TestPluginHost::new(audio_settings, audio_thread_options, false);
    host.set_mono_input(run_options.use_mono_input());
    run_load_audio_file(&run_options, &mut host);
    run_initialize_plugin(&run_options, &mut host);

    let host = Arc::new(Mutex::new(host));
    // This needs to be kept around otherwise the watcher will stop when dropped
    let _maybe_watcher = run_initialize_file_watch_thread(&run_options, &host);

    if run_options.open_editor() {
        let instance = host.lock().unwrap().plugin_instance();
        if let Some(instance) = instance {
            log::info!("Starting GUI");
            start_gui(instance);
        }
    } else {
        thread::park();
    }

    {
        let mut host = host.lock().unwrap();
        if let Err(err) = host.wait() {
            log::error!("Failed to stop audio. Error: {:?}", err);
        }
        log::info!("Closing...");
    }
}

#[cfg(not(target_os = "macos"))]
fn start_gui(_instance: SharedProcessor<PluginInstance>) {
    log::warn!("GUI is unsupported on this OS")
}

#[cfg(target_os = "macos")]
fn start_gui(instance: SharedProcessor<PluginInstance>) {
    let instance_ptr = instance.deref() as *const PluginInstance as *mut PluginInstance;
    let event_loop = tao::event_loop::EventLoop::new();

    let mut editor = unsafe { instance_ptr.as_mut() }
        .unwrap()
        .get_editor()
        .expect("Plugin has no editor");
    let (width, height) = editor.size();

    let window = tao::window::WindowBuilder::new()
        .with_inner_size(tao::dpi::Size::Logical(tao::dpi::LogicalSize::new(
            width as f64,
            height as f64,
        )))
        .build(&event_loop)
        .expect("Failed to create editor window");

    unsafe {
        window.set_title(&(*instance_ptr).get_info().name);
    }
    editor.open(window.ns_view());

    window.request_redraw();

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

fn get_audio_options(run_options: &RunOptions) -> (AudioProcessorSettings, AudioThreadOptions) {
    let mut audio_settings = AudioThread::default_settings().unwrap();
    audio_settings.set_sample_rate(
        run_options
            .sample_rate()
            .map(|s| s as f32)
            .unwrap_or_else(|| audio_settings.sample_rate()),
    );
    audio_settings.set_block_size(
        run_options
            .buffer_size()
            .map(|size| size as usize)
            .unwrap_or_else(|| audio_settings.block_size()),
    );
    let mut audio_thread_options = AudioThreadOptions::default();
    audio_thread_options.buffer_size = run_options
        .buffer_size()
        .map(|s| BufferSize::Fixed(s as usize))
        .unwrap_or(audio_thread_options.buffer_size);
    audio_thread_options.output_device_id = run_options
        .output_device_id()
        .clone()
        .map(AudioDeviceId::Id)
        .unwrap_or(audio_thread_options.output_device_id);
    audio_thread_options.host_id = run_options
        .audio_host_id()
        .clone()
        .map(AudioHostId::Id)
        .unwrap_or(audio_thread_options.host_id);

    if run_options.use_default_input_device() {
        audio_thread_options.input_device_id = Some(AudioDeviceId::Default);
    }
    if let Some(input_device_id) = run_options.input_device_id() {
        audio_thread_options.input_device_id = Some(AudioDeviceId::Id(input_device_id.clone()));
    }

    log::info!(
        "Using audio settings:\n\t\
         Host: {}\n\t\
         Output device ID: {}\n\t\
         Sample rate: {}\n\t\
         Block size: {}\n\t\
         Input channels: {}\n\t\
         Output channels: {}\
        ",
        audio_thread_options.host_id,
        audio_thread_options.output_device_id,
        audio_settings.sample_rate(),
        audio_settings.block_size(),
        audio_settings.input_channels(),
        audio_settings.output_channels()
    );

    (audio_settings, audio_thread_options)
}

/// Load the VST plug-in & exit the process on failure
fn run_initialize_plugin(run_options: &RunOptions, host: &mut TestPluginHost) {
    let path = Path::new(run_options.plugin_path());
    log::info!("Loading VST from: {}...", path.to_str().unwrap());
    if let Err(err) = host.load_plugin(path) {
        log::error!("Failed to load plugin {}", err);
        exit(1);
    }
    log::info!("Initializing audio thread");
    if let Err(err) = host.start() {
        log::error!("Failed to start host: {}", err);
        exit(1);
    }
}

/// Load the audio input file & exit the process on failure
fn run_load_audio_file(run_options: &RunOptions, host: &mut TestPluginHost) {
    if let Some(input_audio) = run_options.input_audio() {
        if let Err(err) = host.set_audio_file_path(PathBuf::from(input_audio)) {
            log::error!("Failed to set input file-path {}", err);
            exit(1);
        }
    }
}

/// Start the file watcher thread
fn run_initialize_file_watch_thread(
    run_options: &RunOptions,
    host: &Arc<Mutex<TestPluginHost>>,
) -> Option<(RecommendedWatcher, JoinHandle<()>)> {
    if run_options.watch() {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();
        let run_options = run_options.clone();
        watcher
            .watch(run_options.plugin_path(), RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        let host = host.clone();
        let handle =
            std::thread::spawn(move || file_watch::run_file_watch_loop(rx, run_options, host));
        Some((watcher, handle))
    } else {
        None
    }
}

/// Start the offline rendering command, rendering to an output file
fn run_offline_rendering(run_options: RunOptions) {
    log::info!("Running offline rendering");
    let output_file_path = run_options.output_audio().clone().unwrap();
    let (audio_settings, _) = get_audio_options(&run_options);
    let offline_renderer = OfflineRenderer::new(
        audio_settings,
        &run_options
            .input_audio()
            .clone()
            .expect("The \"--input\" flag is required for offline rendering"),
        &output_file_path,
        run_options.plugin_path(),
    );
    offline_renderer.run().expect("Failed to render audio");
}
