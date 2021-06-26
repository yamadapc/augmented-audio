use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use notify::{watcher, RecursiveMode, Watcher};
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tao::platform::macos::WindowExtMacOS;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use crate::audio_io::audio_thread::AudioThread;
use crate::audio_io::offline_renderer::OfflineRenderer;
use crate::audio_io::test_plugin_host::TestPluginHost;
use crate::commands::options::RunOptions;
use crate::processors::shared_processor::SharedProcessor;

mod file_watch;

fn start_gui(instance: SharedProcessor<PluginInstance>) {
    let instance_ptr = instance.deref() as *const PluginInstance as *mut PluginInstance;
    let event_loop = tao::event_loop::EventLoop::new();
    let window = tao::window::Window::new(&event_loop).expect("Failed to create editor window");
    unsafe {
        window.set_title(&(*instance_ptr).get_info().name);
    }

    let mut editor = unsafe { instance_ptr.as_mut() }
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
    if run_options.output_audio().is_some() {
        run_offline_rendering(run_options);
        return;
    }

    let mut host = TestPluginHost::default();
    if let Err(err) = host.set_audio_file_path(PathBuf::from(run_options.input_audio())) {
        log::error!("Failed to set input file-path {}", err);
        exit(1);
    }
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

    let instance = host.plugin_instance();

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();

    let host = Arc::new(Mutex::new(host));
    if run_options.watch() {
        let run_options = run_options.clone();
        watcher
            .watch(run_options.plugin_path(), RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        let host = host.clone();
        std::thread::spawn(move || file_watch::run_file_watch_loop(rx, run_options, host));
    }

    if run_options.open_editor() {
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

fn run_offline_rendering(run_options: RunOptions) {
    log::info!("Running offline rendering");
    let output_file_path = run_options.output_audio().clone().unwrap();
    let offline_renderer = OfflineRenderer::new(
        AudioThread::default_settings().expect("Failed to query audio settings"),
        run_options.input_audio(),
        &output_file_path,
        run_options.plugin_path(),
    );
    offline_renderer.run().expect("Failed to render audio");
}
