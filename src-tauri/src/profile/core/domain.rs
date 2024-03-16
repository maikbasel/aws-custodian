use std::collections::HashMap;
use std::fmt::Formatter;

use derivative::Derivative;
use error_stack::{Report, Result};
use heck::AsSnakeCase;
use secstr::SecStr;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserializer, Serializer};

use crate::common::report::extract_printable_attachments;
use crate::profile::core::error::ProfileError;

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

const FIELDS: &[&str] = &["access_key_id", "secret_access_key"];

struct CredentialsVisitor;

impl<'de> Visitor<'de> for CredentialsVisitor {
    type Value = Credentials;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("struct Credentials")
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let access_key_id = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let secret_access_key = seq.next_element::<String>()?.map(SecStr::from);

        Ok(Credentials {
            access_key_id,
            secret_access_key,
        })
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut access_key_id = None;
        let mut secret_access_key = None;

        while let Some(key) = map.next_key()? {
            match key {
                "access_key_id" => {
                    if access_key_id.is_some() {
                        return Err(serde::de::Error::duplicate_field("access_key_id"));
                    }
                    access_key_id = map.next_value()?;
                }
                "secret_access_key" => {
                    if secret_access_key.is_some() {
                        return Err(serde::de::Error::duplicate_field("secret_access_key"));
                    }
                    let sec_str: String = map.next_value()?;
                    secret_access_key = Some(SecStr::from(sec_str));
                }
                _ => return Err(serde::de::Error::unknown_field(key, FIELDS)),
            }
        }

        let access_key_id =
            access_key_id.ok_or_else(|| serde::de::Error::missing_field("access_key_id"))?;

        Ok(Credentials {
            access_key_id,
            secret_access_key,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Credentials", FIELDS, CredentialsVisitor)
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

#[derive(Debug, Eq, PartialEq, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub name: String,
    pub credentials: Credentials,
    pub config: Config,
}

impl Profile {
    pub fn new(name: String, credentials: Credentials, config: Config) -> Self {
        Self {
            name,
            credentials,
            config,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Eq, PartialEq)]
pub struct ProfileSet {
    profiles: Vec<Profile>,
    #[derivative(Debug = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub errors: Vec<Report<ProfileError>>,
}

impl ProfileSet {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: Profile) -> Result<(), ProfileError> {
        if profile.name.trim().is_empty() {
            return Err(Report::new(ProfileError::InvalidProfileNameError)
                .attach_printable("profile name can not be empty or blank"));
        }
        self.profiles.push(profile);
        Ok(())
    }

    pub fn profiles(&self) -> &Vec<Profile> {
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

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Word;
    use fake::Fake;
    use rstest::rstest;
    use serde_json::{json, Value};
    use spectral::prelude::*;

    use crate::common::report::extract_printable_attachments;
    use crate::profile::core::error::ProfileError;

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
        let input_profile_name: String = Word().fake();
        let input_profile: Profile = Profile::new(
            input_profile_name.clone(),
            Credentials::default(),
            Config::default(),
        );

        cut.add_profile(input_profile.clone())
            .expect("should not fail");
        let actual = cut
            .profiles
            .iter()
            .find(|profile| profile.name == input_profile_name);

        assert_eq!(actual, Some(&input_profile))
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    fn should_return_error_when_key_is_blank(#[case] input_profile_name: &str) {
        let mut cut: ProfileSet = ProfileSet::new();
        let input_profile: Profile = Profile::new(
            input_profile_name.to_string(),
            Credentials::default(),
            Config::default(),
        );

        let actual = cut.add_profile(input_profile);

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
        let input_profile_name = Word().fake();
        let input_profile: Profile = Profile::new(
            input_profile_name,
            Credentials::default(),
            Config::default(),
        );

        cut.add_profile(input_profile).expect("should not fail");
        let actual = cut.profiles();

        assert_that!(actual.len()).is_equal_to(1);
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
        let profile = Profile::new("my_profile".to_string(), credentials, config);
        profile_set.add_profile(profile).unwrap();
        let expected_value: Value = json!({
            "profiles": [
                {
                     "name": "my_profile",
                    "credentials": {
                        "access_key_id": "my_access_key_id",
                        "secret_access_key": "my_secret_access_key",
                    },
                    "config":{
                        "region": "eu-west-1",
                        "output_format": "json"
                    }
                }
            ],
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

    #[test]
    fn should_deserialize_credentials() {
        let data = r#"{
            "access_key_id": "myAccessKey",
            "secret_access_key": "mySecretKey"
        }"#;

        // Perform the deserialization
        let deserialized: Credentials = serde_json::from_str(data).unwrap();

        // Test the deserialized data
        assert_eq!(deserialized.access_key_id, Some("myAccessKey".to_string()));
        let secret_access_key =
            std::str::from_utf8(deserialized.secret_access_key.as_ref().unwrap().unsecure())
                .expect("secret access key should be serializable to be UTF-8 string");
        assert_eq!(secret_access_key, "mySecretKey".to_string());
    }
}
