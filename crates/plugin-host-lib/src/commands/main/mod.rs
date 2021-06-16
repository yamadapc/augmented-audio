use std::path::Path;
use std::process::exit;

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
    let path = Path::new(run_options.plugin_path());
    log::info!("Loading VST from: {}...", path.to_str().unwrap());
    if let Err(err) = host.load_plugin(path) {
        log::error!("Failed to load plugin {}", err);
        exit(1);
    }
    log::info!("Initializing audio thread");
    host.start();

    if run_options.open_editor() {
        let instance = host.plugin_instance();
        log::info!("Starting GUI");
        start_gui(instance);
    }

    if let Err(err) = host.wait() {
        log::error!("Failed to join audio thread. Error: {:?}", err);
    }
    log::info!("Closing instance...");
}
