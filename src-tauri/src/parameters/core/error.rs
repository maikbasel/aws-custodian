use std::fmt::{Display, Formatter};

use error_stack::{Context, Report};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::json;

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterDataError {
    ParameterMetaDataLoadError,
    ParameterDataLoadError,
    InvalidParameter(String),
    UnsupportedParameterType(String),
    UnknownParameterType,
    ParameterDataWriteError(String),
}

impl Context for ParameterDataError {}

impl From<Report<ParameterDataError>> for ParameterDataError {
    fn from(value: Report<ParameterDataError>) -> Self {
        let context = value.current_context();
        context.clone()
    }
}

impl Display for ParameterDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterDataError::ParameterMetaDataLoadError => {
                write!(f, "failed to load parameter metadata")
            }
            ParameterDataError::ParameterDataLoadError => {
                write!(f, "failed to load parameter data")
            }
            ParameterDataError::InvalidParameter(param) => {
                write!(f, "invalid parameter: {}", param)
            }
            ParameterDataError::UnsupportedParameterType(param_type) => {
                write!(f, "unsupported parameter type: {}", param_type)
            }
            ParameterDataError::UnknownParameterType => write!(f, "unknown parameter type"),
            ParameterDataError::ParameterDataWriteError(reason) => {
                write!(f, "failed to write parameter data: {}", reason)
            }
        }
    }
}

impl Serialize for ParameterDataError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ParameterDataError", 1)?;
        let (code, message) = match self {
            ParameterDataError::ParameterMetaDataLoadError => {
                ("ParameterMetaDataLoadError", self.to_string())
            }
            ParameterDataError::ParameterDataLoadError => {
                ("ParameterDataLoadError", self.to_string())
            }
            ParameterDataError::InvalidParameter(_) => ("InvalidParameter", self.to_string()),
            ParameterDataError::UnsupportedParameterType(_) => {
                ("UnsupportedParameterType", self.to_string())
            }
            ParameterDataError::UnknownParameterType => ("UnknownParameterType", self.to_string()),
            ParameterDataError::ParameterDataWriteError(reason) => ("ParameterDataWriteError", reason.to_string()),
        };
        state.serialize_field("error", &json!({ "code": code, "message": message }))?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use error_stack::Report;
    use serde_json::json;

    use super::*;

    #[test]
    fn should_return_parameter_data_load_error_when_calling_from_on_report_with_context_parameter_data_load_error(
    ) {
        let error = ParameterDataError::ParameterDataLoadError;
        let report = Report::new(error.clone());

        let result: ParameterDataError = report.into();

        assert_eq!(error, result);
    }

    #[test]
    fn should_serialize_parameter_metadata_load_error() {
        let error = ParameterDataError::ParameterMetaDataLoadError;
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "error": {
                "code": "ParameterMetaDataLoadError",
                "message": "failed to load parameter metadata"
            }
        });
        assert_eq!(serialized, expected.to_string());
    }

    #[test]
    fn should_serialize_parameter_data_load_error() {
        let error = ParameterDataError::ParameterDataLoadError;
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "error": {
                "code": "ParameterDataLoadError",
                "message": "failed to load parameter data"
            }
        });
        assert_eq!(serialized, expected.to_string());
    }

    #[test]
    fn should_serialize_invalid_parameter() {
        let error = ParameterDataError::InvalidParameter("param1".to_string());
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "error": {
                "code": "InvalidParameter",
                "message": "invalid parameter: param1"
            }
        });
        assert_eq!(serialized, expected.to_string());
    }

    #[test]
    fn should_serialize_unsupported_parameter_type() {
        let error = ParameterDataError::UnsupportedParameterType("type1".to_string());
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "error": {
                "code": "UnsupportedParameterType",
                "message": "unsupported parameter type: type1"
            }
        });
        assert_eq!(serialized, expected.to_string());
    }

    #[test]
    fn s_serialize_unknown_parameter_type() {
        let error = ParameterDataError::UnknownParameterType;
        let serialized = serde_json::to_string(&error).unwrap();
        let expected = json!({
            "error": {
                "code": "UnknownParameterType",
                "message": "unknown parameter type"
            }
        });
        assert_eq!(serialized, expected.to_string());
    }
}
