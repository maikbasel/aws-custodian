use async_trait::async_trait;
use crate::core::profile::domain::{ConfigProfiles};
use crate::core::profile::error::ConfigProfilesError;
use error_stack::{Result};

#[async_trait]
pub trait ConfigProfilesSPI {
    async fn load_config_profiles(&self) -> Result<ConfigProfiles, ConfigProfilesError>;
}