use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use crate::parameters::core::domain::Parameter;
use crate::parameters::core::error::ParameterDataError;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ParameterDataAPI: Send + Sync {
    async fn get_parameters(&self, profile_name: &str, page_size: u32) -> error_stack::Result<Vec<Parameter>, ParameterDataError>;
}