use error_stack::{Context, Report};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub enum ProfileError {
    InvalidProfileNameError,
    ProfileDataLoadError,
}

impl Display for ProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileError::InvalidProfileNameError => write!(f, "invalid profile name"),
            ProfileError::ProfileDataLoadError => write!(f, "failed to load profiles"),
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
    fn given_invalid_profile_error_should_output_expected_error_message_when_printed() {
        let expected = "invalid profile name";

        assert_eq!(
            format!("{}", ProfileError::InvalidProfileNameError),
            expected
        )
    }

    #[test]
    fn given_profile_load_error_should_output_expected_error_message_when_printed() {
        let expected = "failed to load profiles";

        assert_eq!(format!("{}", ProfileError::ProfileDataLoadError), expected)
    }

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
