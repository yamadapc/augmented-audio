use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use iced_futures::{futures, BoxStream};
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

const BUFFER_SIZE: usize = 10;

pub enum State {
    Ready {
        file_path: PathBuf,
    },
    Watching {
        watcher: RecommendedWatcher,
        thread: tokio::task::JoinHandle<()>,
        message_rx: tokio::sync::mpsc::Receiver<FileWatchMessage>,
    },
    Finished,
}

pub enum FileWatchMessage {
    Started,
    Changed,
    Error,
}

fn get_file_hash(path: &Path) -> Result<String, std::io::Error> {
    let file_contents = std::fs::read(path)?;
    let digest = md5::compute(file_contents);
    Ok(format!("{:x}", digest))
}

fn run_file_watch_loop(
    plugin_path: &Path,
    rx: Receiver<DebouncedEvent>,
    output: tokio::sync::mpsc::Sender<FileWatchMessage>,
) {
    let inner = || -> Result<(), std::io::Error> {
        let mut current_hash = get_file_hash(plugin_path)?;
        log::info!(
            "Initializing plugin file watch loop. Start hash: {}",
            current_hash
        );
        loop {
            match rx.recv() {
                Ok(_) => match get_file_hash(plugin_path) {
                    Ok(new_hash) => {
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

                        let output = output.clone();
                        tokio::spawn(async move {
                            if let Err(err) = output.send(FileWatchMessage::Changed).await {
                                log::error!("Failed to write change to tokio channel: {}", err);
                            }
                        });
                    }
                    Err(err) => {
                        log::error!("Failed to read file {}", err);
                    }
                },
                // Recv fails if the sender is closed, so no messages will be received
                Err(_) => {
                    log::warn!("Sender closed, stopping receiver");
                    return Ok(());
                }
            }
        }
    };

    if let Err(err) = inner() {
        log::warn!("Error in file watcher thread: {}", err);
    }
    log::info!("Stopping file watcher thread");
}

/// Represents a single target path being watched & wraps file-watching to it can be used by `iced`.
pub struct FileWatcher {
    target_path: PathBuf,
}

impl FileWatcher {
    /// Create a file-watcher object
    pub fn new(plugin_file: &Path) -> Self {
        Self {
            target_path: PathBuf::from(plugin_file),
        }
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for FileWatcher
where
    H: Hasher,
{
    type Output = FileWatchMessage;

    fn hash(&self, state: &mut H) {
        self.target_path.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready {
                file_path: self.target_path.clone(),
            },
            move |mut state| async move {
                match state {
                    State::Ready { file_path } => {
                        log::info!("Starting file-watcher over {}", file_path.to_str().unwrap());
                        let (tx, rx) = channel();
                        let (output_tx, output_rx) = tokio::sync::mpsc::channel(BUFFER_SIZE);
                        if let Ok(mut watcher) = watcher(tx, Duration::from_secs(2)) {
                            if let Err(err) =
                                watcher.watch(file_path.clone(), RecursiveMode::NonRecursive)
                            {
                                log::error!("Failure to watch path {}", err);
                                return Some((FileWatchMessage::Error, State::Finished));
                            }

                            let thread = tokio::task::spawn_blocking({
                                let plugin_file = PathBuf::from(file_path);
                                move || run_file_watch_loop(&plugin_file, rx, output_tx)
                            });
                            Some((
                                FileWatchMessage::Started,
                                State::Watching {
                                    thread,
                                    watcher,
                                    message_rx: output_rx,
                                },
                            ))
                        } else {
                            Some((FileWatchMessage::Error, State::Finished))
                        }
                    }
                    State::Watching {
                        ref mut message_rx, ..
                    } => message_rx.recv().await.map(|message| (message, state)),
                    State::Finished => {
                        let _: () = iced::futures::future::pending().await;
                        None
                    }
                }
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct_watcher() {
        let _watcher = FileWatcher::new("/tmp/target".as_ref());
    }
}
