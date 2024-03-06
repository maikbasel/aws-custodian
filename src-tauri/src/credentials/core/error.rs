use std::fmt::{Display, Formatter};

use error_stack::{Context, Report};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub enum CredentialsError {
    InvalidCredentialsError,
    UnexpectedError(String),
}

impl Display for CredentialsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialsError::InvalidCredentialsError => write!(f, "invalid credentials error"),
            CredentialsError::UnexpectedError(error_code) => {
                write!(f, "unexpected error: {}", error_code)
            }
        }
    }
}

impl Context for CredentialsError {}

impl From<Report<CredentialsError>> for CredentialsError {
    fn from(value: Report<CredentialsError>) -> Self {
        let context = value.current_context();
        context.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_invalid_credentials_error_when_calling_from_on_report_with_context_invalid_credentials_error(
    ) {
        let error = CredentialsError::InvalidCredentialsError;
        let report = Report::new(error.clone());

        let result: CredentialsError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn should_return_unexpected_error_when_calling_from_on_report_with_context_unexpected_error() {
        let error = CredentialsError::UnexpectedError("UnknownError".to_string());
        let report = Report::new(error.clone());

        let result: CredentialsError = report.into();

        assert_eq!(error, result);
    }
}
