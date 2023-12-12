use async_trait::async_trait;
use crate::profile::core::domain::{ProfileSet};
use crate::profile::core::error::ProfileError;
use error_stack::{Result};

#[async_trait]
pub trait ProfileDataSPI {
    async fn load_profile_data(&self) -> Result<ProfileSet, ProfileError>;
}