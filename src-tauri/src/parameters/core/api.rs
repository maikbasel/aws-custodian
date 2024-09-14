use crate::parameters::core::domain::ParameterSet;
use crate::parameters::core::error::ParameterDataError;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParameterRequest {
    pub name: String,
    #[serde(flatten)]
    pub value: Value,
    #[serde(default)]
    pub secure: Option<bool>,
}

impl SetParameterRequest {
    pub fn new(name: String, value: Value, secure: Option<bool>) -> Self {
        Self {
            name,
            value,
            secure,
        }
    }
}

impl From<(String, Value)> for SetParameterRequest {
    fn from((name, value): (String, Value)) -> Self {
        SetParameterRequest::new(name, value, None)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ParameterDataAPI: Send + Sync {
    async fn get_parameters(
        &self,
        profile_name: &str,
        parameter_names: Vec<String>,
    ) -> error_stack::Result<ParameterSet, ParameterDataError>;

    async fn get_available_parameters(
        &self,
        profile_name: &str,
    ) -> error_stack::Result<Vec<String>, ParameterDataError>;

    async fn set_parameter(
        &self,
        profile_name: &str,
        request: SetParameterRequest,
    ) -> error_stack::Result<(), ParameterDataError>;
}
