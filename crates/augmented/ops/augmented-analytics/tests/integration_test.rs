use std::time::Duration;

use tokio::sync::mpsc::unbounded_channel;

use augmented_analytics::{
    AnalyticsClient, AnalyticsEvent, AnalyticsWorker, ClientMetadata, GoogleAnalyticsBackend,
    GoogleAnalyticsConfig,
};

#[tokio::test]
async fn test_create_and_use_client() {
    let _ = wisual_logger::try_init_from_env();

    let (sender, receiver) = unbounded_channel();
    let mut worker = AnalyticsWorker::new(
        Default::default(),
        Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
            "UA-74188650-6",
        ))),
        ClientMetadata::new("1"),
        receiver,
    );

    {
        let client = AnalyticsClient::new(sender);
        client.send(
            AnalyticsEvent::screen()
                .application("testing_analytics_client")
                .version("0.0.0")
                .content("test")
                .build(),
        );
        client.send(
            AnalyticsEvent::event()
                .category("interaction")
                .action("play")
                .build(),
        )
    }

    worker.flush_all().await;
}

#[tokio::test]
async fn test_setup_background_worker() {
    let _ = wisual_logger::try_init_from_env();

    let (sender, receiver) = unbounded_channel();
    let worker = AnalyticsWorker::new(
        Default::default(),
        Box::new(GoogleAnalyticsBackend::new(GoogleAnalyticsConfig::new(
            "UA-74188650-6",
        ))),
        ClientMetadata::new("1"),
        receiver,
    );
    let _worker_handle = worker.spawn();
    let client = AnalyticsClient::new(sender);
    client.send(
        AnalyticsEvent::screen()
            .application("testing_analytics_client")
            .version("0.0.0")
            .content("test")
            .build(),
    );
    client.send(
        AnalyticsEvent::event()
            .category("interaction")
            .action("play")
            .build(),
    );
    tokio::time::sleep(Duration::from_secs(3)).await;
}
