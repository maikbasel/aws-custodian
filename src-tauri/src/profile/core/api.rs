use crate::profile::core::domain::{ProfileSet};
use crate::profile::core::error::ProfileError;

pub trait ProfileAPI {
    fn get_profiles(&self) -> Result<ProfileSet, ProfileError>;
}