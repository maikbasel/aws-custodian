use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

use crate::credentials::core::error::CredentialsError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CredentialsDataSPI: Send + Sync {
    async fn validate_credentials(&self, profile_name: &str) -> Result<(), CredentialsError>;
}
