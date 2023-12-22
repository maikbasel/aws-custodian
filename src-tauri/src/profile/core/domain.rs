use std::collections::HashMap;
use std::sync::Arc;

use crate::common::report::extract_printable_attachments;
use async_trait::async_trait;
use derivative::Derivative;
use error_stack::{Report, Result};
use secstr::SecStr;
use serde::ser::SerializeStruct;
use serde::Serializer;

use crate::profile::core::api::ProfileAPI;
use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;
use heck::AsSnakeCase;
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Credentials {
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<SecStr>,
}

impl serde::Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Credentials", 2)?;

        state.serialize_field("access_key_id", &self.access_key_id)?;
        let secret_access_key = &self
            .secret_access_key
            .as_ref()
            .map(|sec_str| sec_str.unsecure())
            .map(std::str::from_utf8)
            .map_or("Error transforming secret", |result| match result {
                Ok(sec_str) => sec_str,
                Err(e) => panic!("failed to serialize credentials: {}", e),
            });
        state.serialize_field("secret_access_key", secret_access_key)?;

        state.end()
    }
}

impl Credentials {
    pub fn new(access_key_id: Option<&str>, secret_access_key: Option<SecStr>) -> Self {
        let access_key_id_str = access_key_id.map(|r| r.to_string());

        Self {
            access_key_id: access_key_id_str,
            secret_access_key,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, serde::Serialize)]
pub struct Config {
    pub region: Option<String>,
    pub output_format: Option<String>,
}

impl Config {
    pub fn new(region: Option<&str>, output_format: Option<&str>) -> Self {
        let region_str = region.map(|r| r.to_string());
        let output_format_str = output_format.map(|o| o.to_string());

        Self {
            region: region_str,
            output_format: output_format_str,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct Settings {
    pub credentials: Credentials,
    pub config: Config,
}

impl Settings {
    pub fn new(credentials: Credentials, config: Config) -> Self {
        Self {
            credentials,
            config,
        }
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
        Self {
            profiles: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, name: &str, settings: Settings) -> Result<(), ProfileError> {
        if name.trim().is_empty() {
            return Err(Report::new(ProfileError::InvalidProfileNameError)
                .attach_printable("profile name can not be empty or blank"));
        }
        self.profiles.insert(name.to_string(), settings);
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

impl serde::Serialize for ProfileSet {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProfileSet", 2)?;

        state.serialize_field("profiles", &self.profiles)?;
        let error_messages: HashMap<String, Vec<String>> = self
            .errors
            .iter()
            .map(|report| {
                let error_message = report.to_string();
                let error_attachments = extract_printable_attachments(report);
                (format!("{}", AsSnakeCase(error_message)), error_attachments)
            })
            .collect();
        state.serialize_field("errors", &error_messages)?;

        state.end()
    }
}

pub struct ProfileService {
    profile_data_spi: Arc<dyn ProfileDataSPI>,
}

impl ProfileService {
    pub fn new(profile_data_spi: Arc<dyn ProfileDataSPI>) -> Self {
        Self { profile_data_spi }
    }
}

#[async_trait]
impl ProfileAPI for ProfileService {
    async fn get_profiles<'a>(&'a self) -> Result<ProfileSet, ProfileError> {
        self.profile_data_spi.load_profile_data().await
    }
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Word;
    use fake::Fake;
    use rstest::rstest;
    use serde_json::{json, Value};
    use spectral::prelude::*;

    use crate::common::report::extract_printable_attachments;
    use crate::profile::core::error::ProfileError;
    use crate::profile::core::spi::MockProfileDataSPI;

    use super::*;

    #[test]
    fn should_create_empty_profile_set() {
        let expected = ProfileSet::new();

        let actual = ProfileSet::default();

        assert_that(&actual).is_equal_to(expected)
    }

    #[test]
    fn should_add_profile() {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_settings: Settings = Settings::new(Credentials::default(), Config::default());
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
        let input_settings: Settings = Settings::new(Credentials::default(), Config::default());

        let actual = cut.add_profile(input_profile, input_settings);

        assert_that(&actual).is_err();
        let report = actual.unwrap_err();
        assert!(report.contains::<ProfileError>());
        let messages = extract_printable_attachments(&report);
        assert_that(&messages).has_length(1);
        assert_that(&messages).contains(String::from("profile name can not be empty or blank"));
    }

    #[test]
    fn should_return_profiles() {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_settings: Settings = Settings::new(Credentials::default(), Config::default());
        let input_profile = Word().fake();

        cut.add_profile(input_profile, input_settings)
            .expect("should not fail");
        let actual = cut.profiles();

        assert_that!(actual.len()).is_equal_to(1);
    }

    #[tokio::test]
    async fn should_load_profile_data_and_return_the_result() {
        let mut mock_profile_data_spi = MockProfileDataSPI::new();
        mock_profile_data_spi
            .expect_load_profile_data()
            .return_once(|| Ok(ProfileSet::new()));
        let cut = ProfileService::new(Arc::new(mock_profile_data_spi));

        let actual = cut.get_profiles().await;

        assert_that!(actual).is_ok();
    }

    #[test]
    fn should_serialize_credentials() {
        let expected = r#"{"access_key_id":"my_access_key","secret_access_key":"my_secret_key"}"#;
        let credentials = Credentials {
            access_key_id: Some("my_access_key".to_string()),
            secret_access_key: Some(SecStr::from("my_secret_key")),
        };

        let serialized = serde_json::to_string(&credentials).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    #[should_panic(expected = "failed to serialize credentials")]
    fn should_panic_when_failing_to_serialize_credentials() {
        let bad_data = vec![0, 159, 146, 150]; // Non UTF-8 bytes
        let bad_sec_str = SecStr::new(bad_data);

        let cred = Credentials {
            access_key_id: Some("my_access_key".to_string()),
            secret_access_key: Some(bad_sec_str),
        };

        let _ = serde_json::to_string(&cred).unwrap();
    }

    #[test]
    fn should_serialize_profile_set() {
        let mut profile_set = ProfileSet::new();
        let report =
            Report::new(ProfileError::InvalidProfileNameError).attach_printable("some details");
        profile_set.errors.push(report);
        let credentials = Credentials::new(
            Some("my_access_key_id"),
            Some(SecStr::from("my_secret_access_key")),
        );
        let config = Config::new(Some("eu-west-1"), Some("json"));
        let settings = Settings::new(credentials, config);
        profile_set.add_profile("my_profile", settings).unwrap();
        let expected_value: Value = json!({
            "profiles": {
                "my_profile": {
                    "credentials": {
                        "access_key_id": "my_access_key_id",
                        "secret_access_key": "my_secret_access_key",
                    },
                    "config":{
                        "region": "eu-west-1",
                        "output_format": "json"
                    }
                }
            },
            "errors": {
                 "invalid_profile_name": [
                    "some details"
                ]
            }
        });

        let serialized_profile_set = serde_json::to_string(&profile_set).unwrap();
        let serialized_profile_value: serde_json::Value =
            serde_json::from_str(&serialized_profile_set).unwrap();

        assert_eq!(serialized_profile_value, expected_value);
    }
}
