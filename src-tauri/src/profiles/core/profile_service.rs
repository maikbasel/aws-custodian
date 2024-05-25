use async_trait::async_trait;

use crate::profiles::core::api::ProfileDataAPI;
use crate::profiles::core::domain::{Profile, ProfileSet};
use crate::profiles::core::error::ProfileError;
use crate::profiles::core::spi::ProfileDataSPI;

#[allow(dead_code)]
pub struct ProfileService {
    profile_data_spi: Box<dyn ProfileDataSPI>,
}

impl ProfileService {
    pub fn new(profile_data_spi: Box<dyn ProfileDataSPI>) -> Self {
        Self { profile_data_spi }
    }
}

#[async_trait]
impl ProfileDataAPI for ProfileService {
    async fn get_profiles(&self) -> error_stack::Result<ProfileSet, ProfileError> {
        let mut profile_set = self.profile_data_spi.load_profile_data().await?;

        profile_set.sort_profiles_asc();

        Ok(profile_set)
    }

    fn create_profile(&self, profile: &Profile) -> error_stack::Result<(), ProfileError> {
        self.profile_data_spi.save_profile_data(profile)
    }

    fn edit_profile(&self, profile: &Profile) -> error_stack::Result<(), ProfileError> {
        self.profile_data_spi.update_profile_data(profile)
    }

    fn delete_profile(&self, profile_name: &str) -> error_stack::Result<(), ProfileError> {
        self.profile_data_spi.remove_profile_data(profile_name)
    }

    fn delete_profiles(&self, profile_names: &[String]) -> error_stack::Result<(), ProfileError> {
        self.profile_data_spi.remove_profiles_data(profile_names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::core::domain::{Config, Credentials};
    use crate::profiles::core::spi::MockProfileDataSPI;
    use mockall::predicate::eq;
    use spectral::prelude::*;

    #[tokio::test]
    async fn should_sort_profiles_asc_by_name() {
        let profile_1 = Profile::new("b".to_string(), Credentials::default(), Config::default());
        let profile_2 = Profile::new("a".to_string(), Credentials::default(), Config::default());
        let expected = vec![profile_2.clone(), profile_1.clone()];
        let mut profile_set = ProfileSet::new();
        profile_set.add_profile(profile_1);
        profile_set.add_profile(profile_2);

        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        profile_data_spi_mock
            .expect_load_profile_data()
            .returning(move || Ok(profile_set.clone()));

        let profile_service = ProfileService::new(Box::new(profile_data_spi_mock));

        let actual = profile_service.get_profiles().await.unwrap();

        assert_that!(actual.profiles()).is_equal_to(&expected);
    }

    #[test]
    fn should_save_profile() {
        let profile = Profile::new("a".to_string(), Credentials::default(), Config::default());
        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        profile_data_spi_mock
            .expect_save_profile_data()
            .with(eq(profile.clone()))
            .times(1)
            .returning(move |_| Ok(()));
        let profile_service = ProfileService::new(Box::new(profile_data_spi_mock));

        let actual = profile_service.create_profile(&profile);

        assert_that!(actual).is_ok();
    }

    #[test]
    fn should_edit_profile() {
        let profile = Profile::new("a".to_string(), Credentials::default(), Config::default());
        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        profile_data_spi_mock
            .expect_update_profile_data()
            .with(eq(profile.clone()))
            .times(1)
            .returning(move |_| Ok(()));
        let profile_service = ProfileService::new(Box::new(profile_data_spi_mock));

        let actual = profile_service.edit_profile(&profile);

        assert_that!(actual).is_ok();
    }

    #[test]
    fn should_delete_profile() {
        let profile_name = "a".to_string();
        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        profile_data_spi_mock
            .expect_remove_profile_data()
            .with(eq(profile_name.clone()))
            .times(1)
            .returning(move |_| Ok(()));
        let profile_service = ProfileService::new(Box::new(profile_data_spi_mock));

        let actual = profile_service.delete_profile(&profile_name);

        assert_that!(actual).is_ok();
    }

    #[test]
    fn should_delete_profiles() {
        let profile_names = ["a".to_string(), "b".to_string()];
        let mut profile_data_spi_mock = MockProfileDataSPI::new();
        profile_data_spi_mock
            .expect_remove_profiles_data()
            .with(eq(profile_names.clone()))
            .times(1)
            .returning(move |_| Ok(()));
        let profile_service = ProfileService::new(Box::new(profile_data_spi_mock));

        let actual = profile_service.delete_profiles(&profile_names);

        assert_that!(actual).is_ok();
    }
}
