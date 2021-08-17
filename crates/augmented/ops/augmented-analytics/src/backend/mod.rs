mod backend_trait;
mod google;
mod reqwest_executor;

pub use backend_trait::AnalyticsBackend;
pub use google::{GoogleAnalyticsBackend, GoogleAnalyticsConfig};
