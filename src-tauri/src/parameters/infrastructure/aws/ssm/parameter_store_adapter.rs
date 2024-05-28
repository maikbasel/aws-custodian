use async_trait::async_trait;
use aws_sdk_ssm::operation::describe_parameters::{DescribeParametersError, DescribeParametersOutput};
use aws_sdk_ssm::types::ParameterMetadata;
use error_stack::{ Report, ResultExt};

use crate::common::aws::{localstack_endpoint, shared_config_loader, ssm_client};
use crate::parameters::core::domain::Parameter;
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
        let mut shared_config_loader = shared_config_loader(profile_name);

        if let Some(localstack_endpoint) = localstack_endpoint() {
            shared_config_loader = shared_config_loader
                .region("us-east-1")
                .endpoint_url(localstack_endpoint);
        }

        let shared_config = shared_config_loader.load().await;

        let client = ssm_client(&shared_config);
        
        let result: Result<Vec<_>, _> = client.describe_parameters()
            .max_results(page_size as i32)
            .into_paginator()
            .send()
            .collect()
            .await;
        
        let mut parameter_names: Vec<String> = vec![];
        match result {
            Ok(responses) => {
                let parameters_metadata: Vec<&ParameterMetadata> = responses.iter()
                    .flat_map(|response| response.parameters().iter()).
                    collect();
                for parameter_metadata in parameters_metadata {
                    let parameter_name = parameter_metadata.name.as_ref().expect("parameter should have a name");
                    parameter_names.push(parameter_name.clone());
                }
                log::info!("TEST123\
                 {:?}", parameter_names);
                Ok(parameter_names)
            }
            Err(err) => Err(Report::from(err).change_context(ParameterDataError::ParameterMetaDataLoadError))
        }
    }

    async fn load_parameters(
        &self,
        profile_name: &str,
        page_size: u32,
    ) -> error_stack::Result<Vec<Parameter>, ParameterDataError> {
        todo!()
    }
}
