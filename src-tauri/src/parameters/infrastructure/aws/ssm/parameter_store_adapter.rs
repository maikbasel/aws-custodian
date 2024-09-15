use async_trait::async_trait;
use aws_sdk_ssm::config::http::HttpResponse;
use aws_sdk_ssm::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_ssm::operation::put_parameter::PutParameterError;
use aws_sdk_ssm::types::{
    Parameter as SSMParameter, ParameterMetadata as SSMParamterMetadata,
    ParameterType as SSMParamterType, ParameterType,
};
use aws_sdk_ssm::Client;
use chrono::DateTime;
use error_stack::{Report, ResultExt};

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
    ) -> error_stack::Result<Vec<String>, ParameterDataError> {
        let client = Self::get_ssm_client(profile_name).await;

        let result: Result<Vec<_>, _> = client
            .describe_parameters()
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
                let error_meta = err.meta();
                let error_code = error_meta.code();
                let error_message = error_meta.message();

                tracing::error!("Error: [{:?}] {:?}", error_code, error_message);

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
        let name_chunks = parameter_names.chunks(10);
        let client = Self::get_ssm_client(profile_name).await;
        let mut parameters = vec![];

        for name_chunk in name_chunks {
            let parameters_chunk = Self::load_parameter_chunk(name_chunk.to_vec(), &client).await?;
            parameters.extend(parameters_chunk);
        }

        let mut parameter_set = ParameterSet::new();
        parameter_set.add_all_parameters(parameters);

        Ok(parameter_set)
    }

    async fn upsert_parameter(
        &self,
        profile_name: &str,
        parameter: Parameter,
    ) -> error_stack::Result<(), ParameterDataError> {
        let client = Self::get_ssm_client(profile_name).await;

        let handle_error = |err: SdkError<PutParameterError, HttpResponse>| {
            let error_meta = err.meta();
            let error_code = error_meta.code();
            let error_message = error_meta.message();

            tracing::error!("Error: [{:?}] {:?}", error_code, error_message);

            ParameterDataError::ParameterDataWriteError(
                error_message
                    .unwrap_or("unknown parameter error")
                    .to_string(),
            )
        };

        let result = match parameter.value {
            ParameterValue::String(value) => {
                client
                    .put_parameter()
                    .name(parameter.name)
                    .value(value)
                    .r#type(ParameterType::String)
                    .send()
                    .await
            }
            ParameterValue::SecureString(value) => {
                client
                    .put_parameter()
                    .name(parameter.name)
                    .value(value.as_str())
                    .r#type(ParameterType::SecureString)
                    .send()
                    .await
            }
            ParameterValue::StringList(list) => {
                client
                    .put_parameter()
                    .name(parameter.name)
                    .value(list.join(","))
                    .r#type(ParameterType::StringList)
                    .send()
                    .await
            }
        };

        result
            .map(|_| ())
            .map_err(|err| Report::from(handle_error(err)))
    }
}

impl ParameterStoreAdapter {
    async fn get_ssm_client(profile_name: &str) -> Client {
        let mut shared_config_loader = shared_config_loader(profile_name).await;

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

        let version = Some(ssm_parameter.version);
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

    async fn load_parameter_chunk(
        parameter_names: Vec<String>,
        client: &Client,
    ) -> error_stack::Result<Vec<Parameter>, ParameterDataError> {
        let result = client
            .get_parameters()
            .set_names(Some(parameter_names))
            .with_decryption(true)
            .send()
            .await;

        match result {
            Ok(response) => {
                let parameters: Vec<Parameter> = response
                    .parameters()
                    .iter()
                    .flat_map(Self::parse_ssm_parameter)
                    .collect();

                Ok(parameters)
            }
            Err(err) => {
                let error_meta = err.meta();
                let error_code = error_meta.code();
                let error_message = error_meta.message();

                tracing::error!("Error: [{:?}] {:?}", error_code, error_message);

                Err(Report::from(err).change_context(ParameterDataError::ParameterDataLoadError))
            }
        }
    }
}
