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

use crate::audio_io::TestPluginHost;
use crate::commands::options::RunOptions;

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
    let mut host = TestPluginHost::default();
    host.set_audio_file_path(PathBuf::from(run_options.input_audio()));
    let path = Path::new(run_options.plugin_path());
    log::info!("Loading VST from: {}...", path.to_str().unwrap());
    if let Err(err) = host.load_plugin(path) {
        log::error!("Failed to load plugin {}", err);
        exit(1);
    }
    log::info!("Initializing audio thread");
    host.start();

    let instance = host.plugin_instance();

    {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(3)).unwrap();
        let run_options = run_options.clone();
        watcher.watch(run_options.plugin_path(), RecursiveMode::NonRecursive);
        std::thread::spawn(move || loop {
            match rx.recv() {
                Ok(_) => match host.load_plugin(Path::new(run_options.plugin_path())) {
                    Ok(_) => {
                        log::info!("Reloaded plugin");
                    }
                    Err(err) => {
                        log::error!("Failed to reload plugin: {}", err);
                    }
                },
                Err(err) => log::error!("File watch error: {}", err),
            }
        });
    }

    if run_options.open_editor() {
        log::info!("Starting GUI");
        start_gui(instance);
    }
    // if let Err(err) = host.wait() {
    //     log::error!("Failed to join audio thread. Error: {:?}", err);
    // }
    // log::info!("Closing instance...");
}
