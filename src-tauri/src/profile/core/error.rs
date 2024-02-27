use error_stack::{Context, Report};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
