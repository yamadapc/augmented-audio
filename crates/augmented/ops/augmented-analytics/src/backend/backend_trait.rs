use crate::model::{AnalyticsEvent, ClientMetadata};
use async_trait::async_trait;

#[async_trait]
pub trait AnalyticsBackend: Send {
    /// Back-ends should define a method to post events in bulk.
    async fn send_bulk(
        &mut self,
        metadata: &ClientMetadata,
        events: &[AnalyticsEvent],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
