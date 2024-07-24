use async_trait::async_trait;
#[cfg(test)]
use mockall::predicate::*;

use crate::parameters::core::api::ParameterDataAPI;
use crate::parameters::core::domain::Parameter;
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
        page_size: u32,
    ) -> error_stack::Result<Vec<Parameter>, ParameterDataError> {
        let parameter_names = self
            .parameter_data_spi
            .load_available_parameter_names(profile_name, page_size)
            .await?;

        self.parameter_data_spi
            .load_parameters(profile_name, parameter_names)
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
        let input_page_size = 10;
        let output_param_name = "param1";
        let output_param = Parameter {
            name: "param1".to_string(),
            value: ParameterValue::String("value1".to_string()),
            version: 1,
            last_modified_date: None,
            identifier: None,
        };
        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_load_available_parameter_names()
            .with(eq(input_profile_name), eq(input_page_size))
            .returning(move |_, _| Ok(vec![output_param_name.to_string()]));
        mock_parameter_data_spi
            .expect_load_parameters()
            .with(
                eq(input_profile_name),
                eq(vec![output_param_name.to_string()]),
            )
            .returning(move |_, _| Ok(vec![output_param.clone()]));
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let result = cut
            .get_parameters(input_profile_name, input_page_size)
            .await;

        assert_that!(result).is_ok();
    }
}
