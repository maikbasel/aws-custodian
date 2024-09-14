use crate::common::secure_string::SecureString;
use crate::parameters::core::api::{ParameterDataAPI, SetParameterRequest, Value};
use crate::parameters::core::domain::ParameterValue;
#[allow(unused_imports)] // false-positive
use crate::parameters::core::domain::{Parameter, ParameterSet};
use crate::parameters::core::error::ParameterDataError;
use crate::parameters::core::spi::ParameterDataSPI;
use async_trait::async_trait;
#[cfg(test)]
use mockall::predicate::*;

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

    async fn set_parameter(
        &self,
        profile_name: &str,
        request: SetParameterRequest,
    ) -> error_stack::Result<(), ParameterDataError> {
        match request.value {
            Value::Single(value) => {
                let parameter_value;
                if request.secure.unwrap_or(false) {
                    let secure_value = SecureString::from(value);
                    parameter_value = ParameterValue::SecureString(secure_value);
                } else {
                    parameter_value = ParameterValue::String(value);
                }

                let parameter: Parameter = (request.name.to_string(), parameter_value).into();
                self.parameter_data_spi
                    .upsert_parameter(profile_name, parameter)
                    .await
            }
            Value::Multiple(values) => {
                let parameter: Parameter =
                    (request.name.to_string(), ParameterValue::StringList(values)).into();

                self.parameter_data_spi
                    .upsert_parameter(profile_name, parameter)
                    .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::core::domain::{Parameter, ParameterValue};
    use crate::parameters::core::spi::MockParameterDataSPI;
    use spectral::prelude::*;

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
            Some(1),
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

    #[tokio::test]
    async fn should_upsert_string_parameter() {
        let input_profile_name = "dev";
        let parameter_value = "value1".to_string();
        let parameter_name = "param1".to_string();
        let input_parameter: Parameter = (
            parameter_name.clone(),
            ParameterValue::String(parameter_value.clone()),
        )
            .into();
        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_upsert_parameter()
            .with(eq(input_profile_name), eq(input_parameter))
            .returning(|_, _| Ok(()));
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let actual = cut
            .set_parameter(
                input_profile_name,
                (parameter_name, Value::Single(parameter_value)).into(),
            )
            .await;

        assert_that!(actual).is_ok();
    }

    #[tokio::test]
    async fn should_upsert_secure_parameter() {
        let input_profile_name = "dev";
        let parameter_value = "value1".to_string();
        let parameter_name = "param1".to_string();
        let input_parameter: Parameter = (
            parameter_name.clone(),
            ParameterValue::SecureString(SecureString::from(parameter_value.clone())),
        )
            .into();
        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_upsert_parameter()
            .with(eq(input_profile_name), eq(input_parameter))
            .returning(|_, _| Ok(()));
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let actual = cut
            .set_parameter(
                input_profile_name,
                SetParameterRequest::new(
                    parameter_name,
                    Value::Single(parameter_value),
                    Some(true),
                ),
            )
            .await;

        assert_that!(actual).is_ok();
    }

    #[tokio::test]
    async fn should_upsert_string_list_parameter() {
        let input_profile_name = "dev";
        let parameter_value = "value1".to_string();
        let parameter_name = "param1".to_string();
        let input_parameter: Parameter = (
            parameter_name.clone(),
            ParameterValue::StringList(vec![parameter_value.clone()]),
        )
            .into();
        let mut mock_parameter_data_spi = MockParameterDataSPI::new();
        mock_parameter_data_spi
            .expect_upsert_parameter()
            .with(eq(input_profile_name), eq(input_parameter))
            .returning(|_, _| Ok(()));
        let cut = ParameterService::new(Box::new(mock_parameter_data_spi));

        let actual = cut
            .set_parameter(
                input_profile_name,
                (parameter_name, Value::Multiple(vec![parameter_value])).into(),
            )
            .await;

        assert_that!(actual).is_ok();
    }
}
