use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::parameters::core::domain::ParameterSet;
use crate::parameters::core::error::ParameterDataError;

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
}
