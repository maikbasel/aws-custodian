use error_stack::{Context, Report};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProfileDataError {
    InvalidProfileNameError,
    ProfileDataLoadError,
    ProfileNotFoundError,
    ConfigFileLoadError,
    ConfigFileWriteError,
    CredentialsFileLoadError,
    CredentialsFileWriteError,
}

impl Display for ProfileDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileDataError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ProfileDataError::ProfileDataLoadError => write!(f, "failed to load profiles"),
            ProfileDataError::ProfileNotFoundError => write!(f, "profile not found"),
            ProfileDataError::ConfigFileLoadError => write!(f, "failed to load config file"),
            ProfileDataError::ConfigFileWriteError => write!(f, "failed to write config file"),
            ProfileDataError::CredentialsFileLoadError => {
                write!(f, "failed to load credentials file")
            }
            ProfileDataError::CredentialsFileWriteError => {
                write!(f, "failed to write credentials file")
            }
        }
    }
}

impl Context for ProfileDataError {}

impl From<Report<ProfileDataError>> for ProfileDataError {
    fn from(value: Report<ProfileDataError>) -> Self {
        let context = value.current_context();
        context.clone()
    }
}

impl Serialize for ProfileDataError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ProfileDataError", 1)?;
        let (code, message) = match self {
            ProfileDataError::InvalidProfileNameError => (
                "InvalidProfileNameError",
                ProfileDataError::InvalidProfileNameError.to_string(),
            ),
            ProfileDataError::ProfileDataLoadError => (
                "ProfileDataLoadError",
                ProfileDataError::ProfileDataLoadError.to_string(),
            ),
            ProfileDataError::ProfileNotFoundError => (
                "ProfileNotFoundError",
                ProfileDataError::ProfileNotFoundError.to_string(),
            ),
            ProfileDataError::ConfigFileLoadError => (
                "ConfigFileLoadError",
                ProfileDataError::ConfigFileLoadError.to_string(),
            ),
            ProfileDataError::ConfigFileWriteError => (
                "ConfigFileWriteError",
                ProfileDataError::ConfigFileWriteError.to_string(),
            ),
            ProfileDataError::CredentialsFileLoadError => (
                "CredentialsFileLoadError",
                ProfileDataError::CredentialsFileLoadError.to_string(),
            ),
            ProfileDataError::CredentialsFileWriteError => (
                "CredentialsFileWriteError",
                ProfileDataError::CredentialsFileWriteError.to_string(),
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
        let error = ProfileDataError::ProfileDataLoadError;
        let report = Report::new(error.clone());

        let result: ProfileDataError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn should_return_invalid_profile_name_error_when_calling_from_on_report_with_context_invalid_profile_name_error(
    ) {
        let error = ProfileDataError::InvalidProfileNameError;
        let report = Report::new(error.clone());

        let result: ProfileDataError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn serialize_invalid_profile_name_error_to_json() {
        let error = ProfileDataError::InvalidProfileNameError;
        let expected = json!({ "error": {"code": "InvalidProfileNameError", "message": ProfileDataError::InvalidProfileNameError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_profile_data_load_error_to_json() {
        let error = ProfileDataError::ProfileDataLoadError;
        let expected = json!({ "error": {"code": "ProfileDataLoadError", "message": ProfileDataError::ProfileDataLoadError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_profile_not_found_error_to_json() {
        let error = ProfileDataError::ProfileNotFoundError;
        let expected = json!({ "error": {"code": "ProfileNotFoundError", "message": ProfileDataError::ProfileNotFoundError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }
}
