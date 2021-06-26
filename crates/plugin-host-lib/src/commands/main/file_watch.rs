use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use notify::DebouncedEvent;

use crate::commands::options::RunOptions;
use crate::TestPluginHost;

pub fn run_file_watch_loop(
    rx: Receiver<DebouncedEvent>,
    run_options: RunOptions,
    host: Arc<Mutex<TestPluginHost>>,
) -> ! {
    let inner = || -> Result<(), std::io::Error> {
        let mut current_hash = get_file_hash(run_options.plugin_path().as_ref())?;
        loop {
            match rx.recv() {
                Ok(_) => {
                    let mut host = host.lock().unwrap();
                    let new_hash = get_file_hash(run_options.plugin_path().as_ref())?;
                    if new_hash == current_hash {
                        log::warn!("Ignoring event due to same plugin hash");
                        continue;
                    } else {
                        log::info!(
                            "Received file change event. Plug-in will be reloaded content_hash={}",
                            new_hash
                        );
                        current_hash = new_hash;
                    }

                    match (*host).load_plugin(Path::new(run_options.plugin_path())) {
                        Ok(_) => {
                            log::info!("Reloaded plugin");
                        }
                        Err(err) => {
                            log::error!("Failed to reload plugin: {}", err);
                        }
                    }
                }
                Err(err) => log::error!("File watch error: {}", err),
            }
        }
    };

    loop {
        if let Err(err) = inner() {
            log::error!("Error in file watcher thread: {}", err);
        }
    }
}

fn get_file_hash(path: &Path) -> Result<String, std::io::Error> {
    let file_contents = std::fs::read(path)?;
    let digest = md5::compute(file_contents);
    Ok(format!("{:x}", digest))
}
