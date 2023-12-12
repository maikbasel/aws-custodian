use async_trait::async_trait;
use crate::profile::core::domain::{ConfigProfiles};
use crate::profile::core::error::ConfigProfilesError;
use error_stack::{Result};

#[async_trait]
pub trait ConfigProfilesSPI {
    async fn load_config_profiles(&self) -> Result<ConfigProfiles, ConfigProfilesError>;
}