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

use actix::Recipient;
use notify::DebouncedEvent;

use crate::audio_io::LoadPluginMessage;

#[mockall::automock]
pub trait LoadPluginSink {
    fn push(&self, msg: LoadPluginMessage);
}

impl LoadPluginSink for Recipient<LoadPluginMessage> {
    fn push(&self, msg: LoadPluginMessage) {
        self.do_send(msg);
    }
}

// TODO:
// * This should signal when ready
// * This should accept a stop signal
// * Should probably use an actor instead of looking as is
pub fn run_file_watch_loop(
    rx: Receiver<DebouncedEvent>,
    plugin_path: &Path,
    recipient: impl LoadPluginSink,
) -> ! {
    let inner = || -> Result<(), std::io::Error> {
        let mut current_hash = get_file_hash(plugin_path)?;
        loop {
            match rx.recv() {
                Ok(_) => {
                    let new_hash = get_file_hash(plugin_path)?;
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

                    recipient.push(LoadPluginMessage {
                        plugin_path: Path::new(plugin_path).into(),
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

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_get_file_hash() {
        let data_path =
            tempdir::TempDir::new("plugin_host_lib__file_watch__test_get_file_watch").unwrap();
        let test_file_path = data_path.path().join("test.txt");

        let test_str = "something";
        std::fs::write(&test_file_path, test_str).unwrap();

        let result = get_file_hash(&test_file_path).unwrap();
        assert_eq!(result, "437b930db84b8079c2dd804a71936b5f");
    }

    #[test]
    fn test_run_file_watch_loop() {
        let data_path =
            tempdir::TempDir::new("plugin_host_lib__file_watch__test_get_file_watch").unwrap();
        let test_file_path = data_path.path().join("test.txt");
        std::fs::write(&test_file_path, "something").unwrap();

        impl LoadPluginSink for std::sync::mpsc::Sender<LoadPluginMessage> {
            fn push(&self, msg: LoadPluginMessage) {
                self.send(msg).unwrap();
            }
        }

        let (event_tx, event_rx) = channel();
        let (sink_tx, sink_rx) = channel();

        std::thread::spawn({
            let test_file_path = test_file_path.clone();
            move || run_file_watch_loop(event_rx, &test_file_path, sink_tx)
        });

        std::thread::sleep(Duration::from_millis(100));
        std::fs::write(&test_file_path, "something-else").unwrap();
        event_tx
            .send(DebouncedEvent::Write(test_file_path.clone()))
            .unwrap();
        let _ = sink_rx.recv_timeout(Duration::from_millis(100)).unwrap();

        event_tx
            .send(DebouncedEvent::Write(test_file_path.clone()))
            .unwrap();
        assert!(sink_rx.recv_timeout(Duration::from_millis(100)).is_err());
    }
}
