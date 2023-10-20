use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct ConfigurationError;

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parsing configuration error")
    }
}

impl std::error::Error for ConfigurationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_output_expected_error_message_when_printed() {
        let expected = "parsing configuration error";

        assert_eq!(format!("{}", ConfigurationError), expected)
    }
}