use crate::profile::core::domain::{ConfigProfiles};
use crate::profile::core::error::ConfigProfilesError;

pub trait ConfigProfilesAPI {
    fn get_config_profiles() -> Result<ConfigProfiles, ConfigProfilesError>;
}