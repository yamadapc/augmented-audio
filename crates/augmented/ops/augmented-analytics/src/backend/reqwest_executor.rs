use mockall::automock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(test)]
    #[error("A mock error")]
    MockError,
}

pub type Result<T> = std::result::Result<T, ExecutorError>;

pub struct RequestExecutor;

impl Default for RequestExecutor {
    fn default() -> Self {
        Self {}
    }
}

#[automock]
impl RequestExecutor {
    pub async fn execute(
        &self,
        client: &reqwest::Client,
        request: reqwest::Request,
    ) -> Result<reqwest::StatusCode> {
        log::trace!("Firing analytics request");
        Ok(client
            .execute(request)
            .await
            .map(|response| response.status())?)
    }
}
