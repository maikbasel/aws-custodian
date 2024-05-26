use async_trait::async_trait;
use error_stack::Result;
#[cfg(test)]
use mockall::automock;

use crate::profiles::core::domain::{Profile, ProfileSet};
use crate::profiles::core::error::ProfileDataError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProfileDataSPI: Send + Sync {
    async fn load_profile_data(&self) -> Result<ProfileSet, ProfileDataError>;

    fn save_profile_data(&self, profile: &Profile) -> Result<(), ProfileDataError>;

    fn remove_profile_data(&self, profile_name: &str) -> Result<(), ProfileDataError>;

    fn remove_profiles_data(&self, profile_names: &[String]) -> Result<(), ProfileDataError>;

    fn update_profile_data(&self, profile: &Profile) -> Result<(), ProfileDataError>;
}
