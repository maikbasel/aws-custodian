use async_trait::async_trait;
use crate::core::configuration::domain::{Configuration};
use crate::core::configuration::error::ConfigurationError;
use error_stack::{Result};

#[async_trait]
pub trait ConfigurationSPI {
    async fn load_configuration(&self) -> Result<Configuration, ConfigurationError>;
}