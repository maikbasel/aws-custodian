use crate::core::profile::domain::{ConfigProfiles};
use crate::core::profile::error::ConfigProfilesError;

pub trait ConfigProfilesAPI {
    fn get_config_profiles() -> Result<ConfigProfiles, ConfigProfilesError>;
}