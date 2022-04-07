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
