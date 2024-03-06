use std::fmt::{Display, Formatter};

use error_stack::Context;

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
