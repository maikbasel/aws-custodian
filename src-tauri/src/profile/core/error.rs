use std::fmt::{Debug, Display, Formatter};
use error_stack::{Context};

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigProfilesError {
    InvalidProfileNameError,
    ConfigLoadError,
}

impl Display for ConfigProfilesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigProfilesError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ConfigProfilesError::ConfigLoadError => write!(f, "failed to load config")
        }
    }
}

impl Context for ConfigProfilesError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_invalid_profile_error_should_output_expected_error_message_when_printed() {
        let expected = "invalid profile name";

        assert_eq!(format!("{}", ConfigProfilesError::InvalidProfileNameError), expected)
    }

    #[test]
    fn given_profile_load_error_should_output_expected_error_message_when_printed() {
        let expected = "failed to load config";

        assert_eq!(format!("{}", ConfigProfilesError::ConfigLoadError), expected)
    }
}