use std::fmt::{Debug, Display, Formatter};
use error_stack::{Context};

#[derive(Debug, Eq, PartialEq)]
pub enum ProfileError {
    InvalidProfileNameError,
    ProfileDataLoadError,
}

impl Display for ProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ProfileError::ProfileDataLoadError => write!(f, "failed to load profiles")
        }
    }
}

impl Context for ProfileError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_invalid_profile_error_should_output_expected_error_message_when_printed() {
        let expected = "invalid profile name";

        assert_eq!(format!("{}", ProfileError::InvalidProfileNameError), expected)
    }

    #[test]
    fn given_profile_load_error_should_output_expected_error_message_when_printed() {
        let expected = "failed to load profiles";

        assert_eq!(format!("{}", ProfileError::ProfileDataLoadError), expected)
    }
}