use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

use crate::profiles::core::domain::{Profile, ProfileSet};
use crate::profiles::core::error::ProfileDataError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfileDataAPI: Send + Sync {
    async fn get_profiles(&self) -> Result<ProfileSet, ProfileDataError>;

    fn create_profile(&self, profile: &Profile) -> Result<(), ProfileDataError>;

    fn edit_profile(&self, profile: &Profile) -> Result<(), ProfileDataError>;

    fn delete_profile(&self, profile_name: &str) -> Result<(), ProfileDataError>;

    fn delete_profiles(&self, profile_names: &[String]) -> Result<(), ProfileDataError>;
}
