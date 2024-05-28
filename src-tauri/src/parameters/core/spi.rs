use crate::parameters::core::domain::Parameter;
use crate::parameters::core::error::ParameterDataError;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ParameterDataSPI: Send + Sync {
    async fn load_available_parameter_names(
        &self,
        profile_name: &str,
        page_size: u32,
    ) -> error_stack::Result<Vec<String>, ParameterDataError>;

    async fn load_parameters(
        &self,
        profile_name: &str,
        page_size: u32,
    ) -> error_stack::Result<Vec<Parameter>, ParameterDataError>;
}