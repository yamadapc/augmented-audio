mod backend_trait;
mod google;

pub use backend_trait::AnalyticsBackend;
pub use google::{GoogleAnalyticsBackend, GoogleAnalyticsConfig};
