// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::path::Path;
use std::sync::mpsc::Receiver;

use actix::Addr;
use notify::DebouncedEvent;

use crate::audio_io::test_plugin_host::TestPluginHost;
use crate::audio_io::LoadPluginMessage;
use crate::commands::options::RunOptions;

pub fn run_file_watch_loop(
    rx: Receiver<DebouncedEvent>,
    run_options: RunOptions,
    host: Addr<TestPluginHost>,
) -> ! {
    let inner = || -> Result<(), std::io::Error> {
        let mut current_hash = get_file_hash(run_options.plugin_path().as_ref())?;
        loop {
            match rx.recv() {
                Ok(_) => {
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

                    // TODO - How to get the result from an arbitrary thread?
                    host.do_send(LoadPluginMessage {
                        plugin_path: Path::new(run_options.plugin_path()).into(),
                    });
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
