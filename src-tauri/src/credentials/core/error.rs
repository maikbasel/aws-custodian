use std::fmt::{Display, Formatter};

use error_stack::{Context, Report};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::json;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CredentialsError {
    InvalidCredentialsError,
    UnexpectedError(String),
}

impl Display for CredentialsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialsError::InvalidCredentialsError => write!(f, "invalid credentials error"),
            CredentialsError::UnexpectedError(reason) => {
                write!(f, "unexpected error: {}", reason)
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

impl Serialize for CredentialsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CredentialsError", 1)?;
        let (code, message) = match self {
            CredentialsError::InvalidCredentialsError => (
                "InvalidCredentialsError",
                CredentialsError::InvalidCredentialsError.to_string(),
            ),
            CredentialsError::UnexpectedError(reason) => ("UnexpectedError", reason.to_string()),
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

    #[test]
    fn should_serialize_invalid_credentials_error_to_json() {
        let error = CredentialsError::InvalidCredentialsError;
        let expected = json!({ "error": {"code": "InvalidCredentialsError", "message": CredentialsError::InvalidCredentialsError.to_string(),} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_serialize_unexpected_error_to_json() {
        let error = CredentialsError::UnexpectedError("UnknownError".to_string());
        let expected =
            json!({ "error": {"code": "UnexpectedError", "message": "UnknownError",} }).to_string();

        let serialized = serde_json::to_string(&error).unwrap();

        assert_eq!(serialized, expected);
    }
}
