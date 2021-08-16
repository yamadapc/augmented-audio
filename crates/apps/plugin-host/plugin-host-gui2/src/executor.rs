use iced::{futures, Executor};
use std::future::Future;

pub struct PluginHostExecutor(tokio::runtime::Runtime);

impl Executor for PluginHostExecutor {
    fn new() -> Result<Self, futures::io::Error> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("iced-tokio")
            .build()
            .map(Self)
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let _ = tokio::runtime::Runtime::spawn(&self.0, future);
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = tokio::runtime::Runtime::enter(&self.0);
        f()
    }
}
