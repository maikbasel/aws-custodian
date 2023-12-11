use std::fmt::{Debug, Display, Formatter};
use error_stack::{Context};

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigurationError {
    InvalidProfileNameError,
    ProfileLoadError,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ConfigurationError::ProfileLoadError => write!(f, "failed to load profile")
        }
    }
}

impl Context for ConfigurationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_invalid_profile_error_should_output_expected_error_message_when_printed() {
        let expected = "invalid profile name";

        assert_eq!(format!("{}", ConfigurationError::InvalidProfileNameError), expected)
    }

    #[test]
    fn given_profile_load_error_should_output_expected_error_message_when_printed() {
        let expected = "failed to load profile";

        assert_eq!(format!("{}", ConfigurationError::ProfileLoadError), expected)
    }
}