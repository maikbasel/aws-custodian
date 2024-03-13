use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

use crate::profile::core::domain::{Profile, ProfileSet};
use crate::profile::core::error::ProfileError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfileDataSPI: Send + Sync {
    async fn load_profile_data(&self) -> Result<ProfileSet, ProfileError>;

    fn save_profile_data(&self, profile: &Profile) -> Result<(), ProfileError>;

    fn remove_profile_data(&self, profile_name: &str) -> Result<(), ProfileError>;

    fn update_profile_data(&self, profile: &Profile) -> Result<(), ProfileError>;
}
