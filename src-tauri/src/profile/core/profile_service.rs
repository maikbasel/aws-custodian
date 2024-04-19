use async_trait::async_trait;
#[cfg(test)]
use mockall::predicate::*;

use crate::profile::core::api::ProfileDataAPI;
use crate::profile::core::domain::{Profile, ProfileSet};
use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;

#[allow(dead_code)]
pub struct ProfileService {
    profile_data_spi: Box<dyn ProfileDataSPI>,
}

#[async_trait]
impl ProfileDataAPI for ProfileService {
    async fn get_profiles(&self) -> error_stack::Result<ProfileSet, ProfileError> {
        todo!()
    }


    fn create_profile(&self, profile: Profile) -> error_stack::Result<(), ProfileError> {
        todo!()
    }

    fn edit_profile(&self, profile: Profile) -> error_stack::Result<(), ProfileError> {
        todo!()
    }

    fn delete_profile(&self, profile_name: String) -> error_stack::Result<(), ProfileError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::profile::core::domain::{Config, Credentials};
    use crate::profile::core::spi::MockProfileDataSPI;

    use super::*;

    #[tokio::test]
    async fn should_sort_profiles_asc_by_name() {
        let profile_1 = Profile::new("b".to_string(), Credentials::default(), Config::default(),);
        let profile_2 = Profile::new("a".to_string(), Credentials::default(), Config::default(),);
        let mut profile_set = ProfileSet::new();
        profile_set.add_profile(profile_1).expect("should not fail");
        profile_set.add_profile(profile_2).expect("should not fail");
        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        let t = Arc::new(profile_set);
    }
}