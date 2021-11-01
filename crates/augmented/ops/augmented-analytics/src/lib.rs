//! This is a simple, back-end agnostic analytics client for Rust.
//!
//! There are two main exported entry-points to this crate:
//!
//! * [`AnalyticsWorker`] is a background worker which will report events on a debounced manner
//!   (or once a certain batch size is reached ; 20 by default)
//! * [`AnalyticsClient`] is how to report events ; the events are pushed to a channel which and
//!   should be picked-up by the worker
//!   * [`AnalyticsEvent::screen`] is a builder for screen view events
//!   * [`AnalyticsEvent::event`] is a builder for other events, such as interactions
//!
//! The worker is back-end agnostic and works over an [`AnalyticsBackend`]. The crate provides:
//!
//! * [`GoogleAnalyticsBackend`] & [`GoogleAnalyticsConfig`] to work with GA
//!
//! ## Example
//!
//! ```rust
//! use std::time::Duration;
//!
//! use tokio::sync::mpsc::unbounded_channel;
//!
//! use augmented_analytics::{
//!     AnalyticsClient, AnalyticsEvent, AnalyticsWorker, ClientMetadata, GoogleAnalyticsBackend,
//!     GoogleAnalyticsConfig,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a channel for the worker/client
//!     let (sender, receiver) = unbounded_channel();
//!     // Create the worker
//!     let worker = AnalyticsWorker::new(
//!         Default::default(),
//!         Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
//!             "UA-74188650-6",
//!         ))),
//!         ClientMetadata::new("1"), // <- this should be an anonymous client-id
//!         receiver,
//!     );
//!     // Spawn the worker
//!     let _worker_handle = worker.spawn();
//!
//!     // Create the client
//!     let client = AnalyticsClient::new(sender);
//!
//!     // Fire screen events
//!     client.send(
//!         AnalyticsEvent::screen()
//!             .application("testing_analytics_client")
//!             .version("0.0.0")
//!             .content("test")
//!             .build(),
//!     );
//!
//!     // Fire events
//!     client.send(
//!         AnalyticsEvent::event()
//!             .category("interaction")
//!             .action("play")
//!             .build(),
//!     );
//!     tokio::time::sleep(Duration::from_secs(3)).await;
//! }
//! ```
use std::time::Duration;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub use crate::backend::AnalyticsBackend;
pub use crate::backend::{GoogleAnalyticsBackend, GoogleAnalyticsConfig};
pub use crate::model::{AnalyticsEvent, ClientMetadata};
use tokio::task::JoinHandle;

mod backend;
mod model;

/// Configuration for the worker
pub struct AnalyticsWorkerConfig {
    /// This is the debounce time. If events are fired in a period smaller than this duration, their
    /// post request will be delayed, unless the max-batch-size is reached.
    pub max_flush_duration: Duration,
    /// Maximum batch-size, defaults to 20
    pub max_batch_size: usize,
}

impl Default for AnalyticsWorkerConfig {
    fn default() -> Self {
        Self {
            max_flush_duration: Duration::from_millis(1000),
            max_batch_size: 20,
        }
    }
}

/// A background worker to flush analytics events
pub struct AnalyticsWorker {
    config: AnalyticsWorkerConfig,
    backend: Box<dyn AnalyticsBackend>,
    client_metadata: ClientMetadata,
    events_queue: UnboundedReceiver<AnalyticsEvent>,
}

impl AnalyticsWorker {
    /// Create a new analytics worker with provided parameters
    pub fn new(
        config: AnalyticsWorkerConfig,
        backend: Box<dyn AnalyticsBackend>,
        client_metadata: ClientMetadata,
        events_queue: UnboundedReceiver<AnalyticsEvent>,
    ) -> Self {
        AnalyticsWorker {
            config,
            backend,
            client_metadata,
            events_queue,
        }
    }

    /// Spawn the worker as a future. It'll loop until the client handle is dropped.
    pub fn spawn(mut self) -> JoinHandle<()> {
        tokio::spawn(async move { self.run_loop().await })
    }

    /// Runs a debounced loop to flush events
    async fn run_loop(&mut self) {
        let mut events = vec![];
        let duration = self.config.max_flush_duration;
        loop {
            let recv_future = self.events_queue.recv();
            let result = tokio::time::timeout(duration, recv_future).await;
            match result {
                Ok(Some(event)) => {
                    events.push(event);
                    if events.len() >= self.config.max_batch_size {
                        self.flush(&mut events).await;
                    }
                }
                Ok(None) => {
                    self.flush(&mut events).await;
                    break;
                }
                Err(_) => {
                    self.flush(&mut events).await;
                }
            }
        }
    }

    /// Force flush all events until the sender queue is closed
    #[doc(hidden)]
    pub async fn flush_all(&mut self) {
        let mut events = vec![];

        while let Some(event) = self.events_queue.recv().await {
            log::debug!("Flushing event {:?}", event);
            events.push(event);
        }
        self.send_bulk(&events).await;
    }

    /// Flushes the current batch of events and clears the queue
    async fn flush(&mut self, events: &mut Vec<AnalyticsEvent>) {
        self.send_bulk(events).await;
        events.clear();
    }

    /// Sends a slice of events to the configured back-end
    async fn send_bulk(&mut self, events: &[AnalyticsEvent]) {
        let result = self.backend.send_bulk(&self.client_metadata, events).await;
        if let Err(err) = result {
            log::error!("Failed to post events error={}", err);
            // TODO - Push back to queue or retry
        }
    }
}

/// Client to fire analytics events with
pub struct AnalyticsClient {
    events_queue: UnboundedSender<AnalyticsEvent>,
}

impl AnalyticsClient {
    /// Create a client which pushes events to this [`UnboundedSender`]
    pub fn new(events_queue: UnboundedSender<AnalyticsEvent>) -> Self {
        AnalyticsClient { events_queue }
    }

    /// Fire an [`AnalyticsEvent`] by pushing it onto the events queue
    pub fn send(&self, event: AnalyticsEvent) {
        // Errors will happen if the receiver is closed.
        // In that case, the events will be dropped.
        if let Err(_) = self.events_queue.send(event) {
            log::error!("Receiver is down, but analytics event was fired.");
        }
    }
}
