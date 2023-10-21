use async_trait::async_trait;

use crate::core::configuration::domain::Configuration;
use crate::core::configuration::error::ConfigurationError;
use crate::core::configuration::spi::ConfigurationSPI;

pub struct ParameterStoreAdapter {}

#[async_trait]
impl ConfigurationSPI for ParameterStoreAdapter {
    async fn load_configuration() -> Result<Configuration, ConfigurationError> {
        todo!()
    }
}