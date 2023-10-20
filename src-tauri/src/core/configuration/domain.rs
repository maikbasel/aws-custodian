use std::collections::HashMap;

use error_stack::{Report, Result};

use crate::core::configuration::error::ConfigurationError;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Credentials {
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Config {
    region: Option<String>,
    output_format: Option<String>,
}

#[derive(Debug, Eq, Default, PartialEq, Clone)]
pub struct Settings {
    credentials: Option<Credentials>,
    config: Option<Config>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Configuration {
    pub profiles: HashMap<String, Settings>,
}

impl Configuration {
    pub fn new() -> Self {
        Self { profiles: HashMap::new() }
    }

    pub fn add_profile(&mut self, name: String, settings: Settings) -> Result<(), ConfigurationError> {
        if name.trim().is_empty() {
            let msg = "profile name can not be empty or blank";
            return Err(Report::new(ConfigurationError).attach_printable(msg));
        }

        self.profiles.insert(name, settings);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assertor::{assert_that, ResultAssertion, VecAssertion};
    use rstest::rstest;

    use crate::common::test::report_utils::messages;
    use crate::core::configuration::error::ConfigurationError;

    use super::*;

    #[test]
    fn should_add_profile() {
        let mut cut: Configuration = Configuration::new();
        let input_settings: Settings = Settings { ..Default::default() };
        let input_profile = "key".to_string();

        cut.add_profile(input_profile.clone(), input_settings.clone())
            .expect("should not fail");
        let actual = cut.profiles.get(&input_profile.clone());

        assert_eq!(actual, Some(&input_settings))
    }

    #[rstest]
    #[case("".to_string())]
    #[case(" ".to_string())]
    fn should_return_error_when_key_is_blank(#[case] input_profile: String) {
        let mut cut: Configuration = Configuration::new();
        let input_settings: Settings = Settings { ..Default::default() };

        let actual = cut.add_profile(input_profile, input_settings);

        assert_that!(actual).is_err();
        let report = actual.unwrap_err();
        assert!(report.contains::<ConfigurationError>());
        let messages = messages(report);
        assert_that!(messages).has_length(1);
        assert_that!(messages).contains(String::from("profile name can not be empty or blank"));
    }
}