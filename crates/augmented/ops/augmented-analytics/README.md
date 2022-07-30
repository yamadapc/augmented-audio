# augmented-analytics

This is a simple, back-end agnostic analytics client for Rust.

There are two main exported entry-points to this crate:

* [`AnalyticsWorker`] is a background worker which will report events on a debounced manner
  (or once a certain batch size is reached ; 20 by default)
* [`AnalyticsClient`] is how to report events ; the events are pushed to a channel which and
  should be picked-up by the worker
  * [`AnalyticsEvent::screen`] is a builder for screen view events
  * [`AnalyticsEvent::event`] is a builder for other events, such as interactions

The worker is back-end agnostic and works over an [`AnalyticsBackend`]. The crate provides:

* [`GoogleAnalyticsBackend`] & [`GoogleAnalyticsConfig`] to work with GA

### Example

```rust
use std::time::Duration;

use tokio::sync::mpsc::unbounded_channel;

use augmented_analytics::{
    AnalyticsClient, AnalyticsEvent, AnalyticsWorker, ClientMetadata, GoogleAnalyticsBackend,
    GoogleAnalyticsConfig,
};

#[tokio::main]
async fn main() {
    // Create a channel for the worker/client
    let (sender, receiver) = unbounded_channel();
    // Create the worker
    let worker = AnalyticsWorker::new(
        Default::default(),
        Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
            "UA-74188650-6",
        ))),
        ClientMetadata::new("1"), // <- this should be an anonymous client-id
        receiver,
    );
    // Spawn the worker
    let _worker_handle = worker.spawn();

    // Create the client
    let client = AnalyticsClient::new(sender);

    // Fire screen events
    client.send(
        AnalyticsEvent::screen()
            .application("testing_analytics_client")
            .version("0.0.0")
            .content("test")
            .build(),
    );

    // Fire events
    client.send(
        AnalyticsEvent::event()
            .category("interaction")
            .action("play")
            .build(),
    );
    tokio::time::sleep(Duration::from_secs(3)).await;
}
```

License: MIT
