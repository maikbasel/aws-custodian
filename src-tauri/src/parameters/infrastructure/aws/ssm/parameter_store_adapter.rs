use async_trait::async_trait;
use aws_sdk_ssm::types::{
    Parameter as SSMParameter, ParameterMetadata as SSMParamterMetadata,
    ParameterType as SSMParamterType,
};
use chrono::DateTime;
use error_stack::Report;

use crate::common::aws::{localstack_endpoint, shared_config_loader, ssm_client};
use crate::common::secure_string::SecureString;
use crate::parameters::core::domain::{Parameter, ParameterSet, ParameterValue};
use crate::parameters::core::error::ParameterDataError;
use crate::parameters::core::spi::ParameterDataSPI;

pub struct ParameterStoreAdapter;

#[async_trait]
impl ParameterDataSPI for ParameterStoreAdapter {
    async fn load_available_parameter_names(
        &self,
        profile_name: &str,
        page_size: u32,
    ) -> error_stack::Result<Vec<String>, ParameterDataError> {
        let client = Self::get_ssm_client(profile_name).await;

        let result: Result<Vec<_>, _> = client
            .describe_parameters()
            .max_results(page_size as i32)
            .into_paginator()
            .send()
            .collect()
            .await;

        let mut parameter_names: Vec<String> = vec![];
        match result {
            Ok(responses) => {
                let parameters_metadata: Vec<&SSMParamterMetadata> = responses
                    .iter()
                    .flat_map(|response| response.parameters().iter())
                    .collect();
                for parameter_metadata in parameters_metadata {
                    let parameter_name = parameter_metadata
                        .name
                        .as_ref()
                        .expect("parameter should have a name"); // TODO: Return error instead.
                    parameter_names.push(parameter_name.clone());
                }

                Ok(parameter_names)
            }
            Err(err) => {
                Err(Report::from(err)
                    .change_context(ParameterDataError::ParameterMetaDataLoadError))
            }
        }
    }

    async fn load_parameters(
        &self,
        profile_name: &str,
        parameter_names: Vec<String>,
    ) -> error_stack::Result<ParameterSet, ParameterDataError> {
        let client = Self::get_ssm_client(profile_name).await;

        let result = client
            .get_parameters()
            .set_names(Some(parameter_names))
            .send()
            .await;

        match result {
            Ok(response) => {
                let mut parameters = ParameterSet::new();
                parameters.add_all_parameters(
                    response
                        .parameters()
                        .iter()
                        .flat_map(Self::parse_ssm_parameter)
                        .collect(),
                );
                Ok(parameters)
            }
            Err(err) => {
                Err(Report::from(err).change_context(ParameterDataError::ParameterDataLoadError))
            }
        }
    }
}

impl ParameterStoreAdapter {
    async fn get_ssm_client(profile_name: &str) -> aws_sdk_ssm::Client {
        let mut shared_config_loader = shared_config_loader(profile_name);

        if let Some(localstack_endpoint) = localstack_endpoint() {
            shared_config_loader = shared_config_loader
                .region("us-east-1")
                .endpoint_url(localstack_endpoint);
        }

        let shared_config = shared_config_loader.load().await;

        ssm_client(&shared_config)
    }

    fn parse_ssm_parameter(ssm_parameter: &SSMParameter) -> Result<Parameter, ParameterDataError> {
        let name = ssm_parameter
            .name
            .clone()
            .ok_or(ParameterDataError::InvalidParameter(
                "parameters should have a name".to_string(),
            ))?;
        let parameter_type = ssm_parameter
            .r#type()
            .ok_or(ParameterDataError::InvalidParameter(
                "parameters should have a type".to_string(),
            ))?;
        let value = Self::parse_ssm_parameter_value(ssm_parameter, parameter_type)?;

        let version = ssm_parameter.version;
        let last_modified_date = ssm_parameter.last_modified_date.map(|date_time| {
            let nanos = date_time.as_nanos();
            let millis = (nanos / 1_000_000) as i64;

            DateTime::from_timestamp_millis(millis).expect("should be valid date time")
        });
        let identifier = ssm_parameter.arn.clone();

        Ok(Parameter {
            name,
            value,
            version,
            last_modified_date,
            identifier,
        })
    }

    fn parse_ssm_parameter_value(
        ssm_parameter: &SSMParameter,
        parameter_type: &SSMParamterType,
    ) -> Result<ParameterValue, ParameterDataError> {
        match ssm_parameter.value.clone() {
            Some(ssm_parameter_value) => match parameter_type {
                SSMParamterType::SecureString => Ok(ParameterValue::SecureString(
                    SecureString::from(ssm_parameter_value),
                )),
                SSMParamterType::String => {
                    Ok(ParameterValue::String(ssm_parameter_value.to_string()))
                }
                SSMParamterType::StringList => {
                    let string_values = ssm_parameter_value
                        .split(',')
                        .map(|item| item.to_string())
                        .collect();
                    Ok(ParameterValue::StringList(string_values))
                }
                other if other.as_str() == "NewFeature" => Err(
                    ParameterDataError::UnsupportedParameterType(other.as_str().to_string()),
                ),
                _ => Err(ParameterDataError::UnknownParameterType),
            },
            None => Err(ParameterDataError::InvalidParameter(
                "parameters should have a value".to_string(),
            )),
        }
    }
}
