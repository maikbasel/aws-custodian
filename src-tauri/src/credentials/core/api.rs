use crate::credentials::core::error::CredentialsError;
use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CredentialsDataAPI: Send + Sync {
    async fn validate_credentials(&self, profile_name: &str) -> Result<bool, CredentialsError>;
}
