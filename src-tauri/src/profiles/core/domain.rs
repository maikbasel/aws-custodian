use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::common::secure_string::SecureString;

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<SecureString>,
}

impl Credentials {
    pub fn new(access_key_id: Option<&str>, secret_access_key: Option<SecureString>) -> Self {
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

#[derive(Derivative, Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileSet {
    profiles: Vec<Profile>,
}

impl ProfileSet {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    pub fn profiles(&self) -> &Vec<Profile> {
        &self.profiles
    }

    pub fn sort_profiles_asc(&mut self) {
        self.profiles.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

impl Default for ProfileSet {
    fn default() -> Self {
        ProfileSet::new()
    }
}

#[cfg(test)]
mod tests {
    use fake::faker::lorem::en::Word;
    use fake::Fake;
    use serde_json::{json, Value};
    use spectral::prelude::*;

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

        cut.add_profile(input_profile.clone());
        let actual = cut
            .profiles
            .iter()
            .find(|profile| profile.name == input_profile_name);

        assert_eq!(actual, Some(&input_profile))
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

        cut.add_profile(input_profile);
        let actual = cut.profiles();

        assert_that!(actual.len()).is_equal_to(1);
    }

    #[test]
    fn should_serialize_credentials() {
        let expected = r#"{"access_key_id":"my_access_key","secret_access_key":"my_secret_key"}"#;
        let secure_string = SecureString::from("my_secret_key");
        let credentials = Credentials {
            access_key_id: Some("my_access_key".to_string()),
            secret_access_key: Some(secure_string),
        };

        let serialized = serde_json::to_string(&credentials).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    #[should_panic(expected = "should serialize to utf8 string")]
    fn should_panic_when_failing_to_serialize_credentials() {
        let bad_data = vec![0, 159, 146, 150]; // Non UTF-8 bytes
        let bad_sec_str = SecureString::from(bad_data);

        let cred = Credentials {
            access_key_id: Some("my_access_key".to_string()),
            secret_access_key: Some(bad_sec_str),
        };

        let _ = serde_json::to_string(&cred).unwrap();
    }

    #[test]
    fn should_serialize_profile_set() {
        let mut profile_set = ProfileSet::new();
        let credentials = Credentials::new(
            Some("my_access_key_id"),
            Some(SecureString::from("my_secret_access_key")),
        );
        let config = Config::new(Some("eu-west-1"), Some("json"));
        let profile = Profile::new("my_profile".to_string(), credentials, config);
        profile_set.add_profile(profile);
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
        });

        let serialized_profile_set = serde_json::to_string(&profile_set).unwrap();
        let serialized_profile_value: Value =
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
        let secret_access_key = deserialized.secret_access_key.unwrap();
        assert_eq!(secret_access_key.as_str(), "mySecretKey");
    }

    #[test]
    fn should_sort_profiles_asc() {
        let mut profile_set = ProfileSet::new();
        profile_set.add_profile(Profile::new(
            "c".to_string(),
            Credentials::default(),
            Config::default(),
        ));
        profile_set.add_profile(Profile::new(
            "b".to_string(),
            Credentials::default(),
            Config::default(),
        ));
        profile_set.add_profile(Profile::new(
            "a".to_string(),
            Credentials::default(),
            Config::default(),
        ));

        profile_set.sort_profiles_asc();

        let sorted_profiles = profile_set.profiles();

        assert_that!(&sorted_profiles[0].name).is_equal_to("a".to_string());
        assert_that!(&sorted_profiles[1].name).is_equal_to("b".to_string());
        assert_that!(&sorted_profiles[2].name).is_equal_to("c".to_string());
    }
}
