use std::collections::HashMap;
use std::sync::Arc;

use derivative::Derivative;
use error_stack::{Report, Result};
use secstr::SecStr;
use crate::profile::core::api::ProfileAPI;

use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Credentials {
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<SecStr>,
}

impl Credentials {
    pub fn new(access_key_id: Option<String>, secret_access_key: Option<SecStr>) -> Self {
        Self { access_key_id, secret_access_key }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Config {
    pub region: Option<String>,
    pub output_format: Option<String>,
}

impl Config {
    pub fn new(region: Option<String>, output_format: Option<String>) -> Self {
        Self { region, output_format }
    }
}

#[derive(Debug, Eq, Default, PartialEq, Clone)]
pub struct Settings {
    pub credentials: Option<Credentials>,
    pub config: Option<Config>,
}

impl Settings {
    pub fn new(credentials: Credentials, config: Config) -> Self {
        Self { credentials: Some(credentials), config: Some(config) }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Eq, PartialEq)]
pub struct ProfileSet {
    profiles: HashMap<String, Settings>,
    #[derivative(Debug = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub errors: Vec<Report<ProfileError>>,
}

impl ProfileSet {
    pub fn new() -> Self {
        Self { profiles: HashMap::new(), errors: Vec::new() }
    }

    // changed from String to &str
    pub fn add_profile(&mut self, name: &str, settings: Settings) -> Result<(), ProfileError> {
        if name.trim().is_empty() {
            return Err(Report::new(ProfileError::InvalidProfileNameError)
                .attach_printable("profile name can not be empty or blank"));
        }
        self.profiles.insert(name.to_string(), settings);  // convert to String here as hashmap key is String
        Ok(())
    }

    pub fn profiles(&self) -> &HashMap<String, Settings> {
        &self.profiles
    }
}

impl Default for ProfileSet {
    fn default() -> Self {
        ProfileSet::new()
    }
}

struct ProfileService {
    profile_data_spi: Arc<dyn ProfileDataSPI>
}

impl ProfileAPI for ProfileService {
    fn get_profiles(&self) -> std::result::Result<ProfileSet, ProfileError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use fake::faker::lorem::en::Word;
    use rstest::rstest;
    use spectral::prelude::*;

    use crate::common::test::report_utils::messages;
    use crate::profile::core::error::ProfileError;

    use super::*;

    #[test]
    fn should_add_profile() {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_settings: Settings = Settings { ..Default::default() };
        let input_profile: String = Word().fake();

        cut.add_profile(&input_profile, input_settings.clone())
            .expect("should not fail");
        let actual = cut.profiles.get(&input_profile);

        assert_eq!(actual, Some(&input_settings))
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    fn should_return_error_when_key_is_blank(#[case] input_profile: &str) {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_settings: Settings = Settings { ..Default::default() };

        let actual = cut.add_profile(input_profile, input_settings);

        assert_that(&actual).is_err();
        let report = actual.unwrap_err();
        assert!(report.contains::<ProfileError>());
        let messages = messages(report);
        assert_that(&messages).has_length(1);
        assert_that(&messages).contains(String::from("profile name can not be empty or blank"));
    }

    #[test]
    fn should_return_profiles() {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_settings: Settings = Settings { ..Default::default() };
        let input_profile = Word().fake();

        cut.add_profile(input_profile, input_settings)
            .expect("should not fail");
        let actual = cut.profiles();

        assert_that!(actual.len()).is_equal_to(1);
    }
}