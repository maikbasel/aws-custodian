use error_stack::{Context, Report};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProfileError {
    InvalidProfileNameError,
    ProfileDataLoadError,
    ProfileNotFoundError,
    ConfigFileLoadError,
    ConfigFileWriteError,
    CredentialsFileLoadError,
    CredentialsFileWriteError,
}

impl Display for ProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ProfileError::ProfileDataLoadError => write!(f, "failed to load profiles"),
            ProfileError::ProfileNotFoundError => write!(f, "profile not found"),
            ProfileError::ConfigFileLoadError => write!(f, "failed to load config file"),
            ProfileError::ConfigFileWriteError => write!(f, "failed to write config file"),
            ProfileError::CredentialsFileLoadError => write!(f, "failed to load credentials file"),
            ProfileError::CredentialsFileWriteError => {
                write!(f, "failed to write credentials file")
            }
        }
    }
}

impl Context for ProfileError {}

impl From<Report<ProfileError>> for ProfileError {
    fn from(value: Report<ProfileError>) -> Self {
        let context = value.current_context();
        context.clone()
    }
}

impl Serialize for ProfileError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProfileError", 1)?;
        let (code, message) = match self {
            ProfileError::InvalidProfileNameError => (
                "InvalidProfileNameError",
                ProfileError::InvalidProfileNameError.to_string(),
            ),
            ProfileError::ProfileDataLoadError => (
                "ProfileDataLoadError",
                ProfileError::ProfileDataLoadError.to_string(),
            ),
            ProfileError::ProfileNotFoundError => (
                "ProfileNotFoundError",
                ProfileError::ProfileNotFoundError.to_string(),
            ),
            ProfileError::ConfigFileLoadError => (
                "ConfigFileLoadError",
                ProfileError::ConfigFileLoadError.to_string(),
            ),
            ProfileError::ConfigFileWriteError => (
                "ConfigFileWriteError",
                ProfileError::ConfigFileWriteError.to_string(),
            ),
            ProfileError::CredentialsFileLoadError => (
                "CredentialsFileLoadError",
                ProfileError::CredentialsFileLoadError.to_string(),
            ),
            ProfileError::CredentialsFileWriteError => (
                "CredentialsFileWriteError",
                ProfileError::CredentialsFileWriteError.to_string(),
            ),
        };
        state.serialize_field("error", &json!({ "code": code, "message": message }))?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_return_profile_data_load_error_when_calling_from_on_report_with_context_profile_data_load_error(
    ) {
        let error = ProfileError::ProfileDataLoadError;
        let report = Report::new(error.clone());

        let result: ProfileError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn should_return_invalid_profile_name_error_when_calling_from_on_report_with_context_invalid_profile_name_error(
    ) {
        let error = ProfileError::InvalidProfileNameError;
        let report = Report::new(error.clone());

        let result: ProfileError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn serialize_invalid_profile_name_error_to_json() {
        let error = ProfileError::InvalidProfileNameError;
        let expected = json!({ "error": {"code": "InvalidProfileNameError", "message": ProfileError::InvalidProfileNameError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_profile_data_load_error_to_json() {
        let error = ProfileError::ProfileDataLoadError;
        let expected = json!({ "error": {"code": "ProfileDataLoadError", "message": ProfileError::ProfileDataLoadError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_profile_not_found_error_to_json() {
        let error = ProfileError::ProfileNotFoundError;
        let expected = json!({ "error": {"code": "ProfileNotFoundError", "message": ProfileError::ProfileNotFoundError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }
}
