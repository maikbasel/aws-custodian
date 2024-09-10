use async_trait::async_trait;
#[cfg(test)]
use mockall::predicate::*;

use crate::parameters::core::api::ParameterDataAPI;
#[allow(unused_imports)] // false-positive
use crate::parameters::core::domain::{Parameter, ParameterSet};
use crate::parameters::core::error::ParameterDataError;
use crate::parameters::core::spi::ParameterDataSPI;

pub struct ParameterService {
    parameter_data_spi: Box<dyn ParameterDataSPI>,
}

impl ParameterService {
    pub fn new(parameter_data_spi: Box<dyn ParameterDataSPI>) -> Self {
        Self { parameter_data_spi }
    }
}

#[async_trait]
impl ParameterDataAPI for ParameterService {
    async fn get_parameters(
        &self,
        profile_name: &str,
        parameter_names: Vec<String>,
    ) -> error_stack::Result<ParameterSet, ParameterDataError> {
        self.parameter_data_spi
            .load_parameters(profile_name, parameter_names)
            .await
    }

    async fn get_available_parameters(
        &self,
        profile_name: &str,
    ) -> error_stack::Result<Vec<String>, ParameterDataError> {
        self.parameter_data_spi
            .load_available_parameter_names(profile_name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::parameters::core::domain::ParameterValue;
    use crate::parameters::core::spi::MockParameterDataSPI;

    use super::*;

    #[tokio::test]
    async fn should_load_available_parameters() {
        let input_profile_name = "dev";
        let output_param_name = "param1";

        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_load_available_parameter_names()
            .with(eq(input_profile_name))
            .returning(move |_| Ok(vec![output_param_name.to_string()]));
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let result = cut.get_available_parameters(input_profile_name).await;

        assert_that!(result).is_ok();
    }

    #[tokio::test]
    async fn should_load_parameters() {
        let input_profile_name = "dev";
        let input_param_name = "param1";
        let output_param = Parameter::new(
            "param1".to_string(),
            ParameterValue::String("value1".to_string()),
            1,
            None,
            None,
        );
        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_load_parameters()
            .with(
                eq(input_profile_name),
                eq(vec![input_param_name.to_string()]),
            )
            .returning(move |_, _| {
                let mut parameters = ParameterSet::new();
                parameters.add_all_parameters(vec![output_param.clone()]);
                Ok(parameters)
            });
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let result = cut
            .get_parameters(input_profile_name, vec![input_param_name.to_string()])
            .await;

        assert_that!(result).is_ok();
    }
}
