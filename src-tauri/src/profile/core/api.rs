use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

use crate::profile::core::domain::{Profile, ProfileSet};
use crate::profile::core::error::ProfileError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfileDataAPI: Send + Sync {
    async fn get_profiles(&self) -> Result<ProfileSet, ProfileError>;

    fn create_profile(&self, profile: &Profile) -> Result<(), ProfileError>;

    fn edit_profile(&self, profile: &Profile) -> Result<(), ProfileError>;

    fn delete_profile(&self, profile_name: &str) -> Result<(), ProfileError>;
}
