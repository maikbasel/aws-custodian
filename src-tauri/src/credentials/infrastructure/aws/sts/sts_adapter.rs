use async_trait::async_trait;
use aws_sdk_sts::error::ProvideErrorMetadata;
use error_stack::Report;

use crate::common::aws::{localstack_endpoint, shared_config_loader, sts_client};
use crate::credentials::core::error::CredentialsError;
use crate::credentials::core::spi::CredentialsDataSPI;

pub struct STSAdapter;

#[async_trait]
impl CredentialsDataSPI for STSAdapter {
    async fn validate_credentials(
        &self,
        profile_name: &str,
    ) -> error_stack::Result<(), CredentialsError> {
        let mut shared_config_loader = shared_config_loader(profile_name);

        if let Some(localstack_endpoint) = localstack_endpoint() {
            shared_config_loader = shared_config_loader
                .region("us-east-1")
                .endpoint_url(localstack_endpoint);
        }

        let shared_config = shared_config_loader.load().await;

        let client = sts_client(&shared_config);

        let result = client.get_caller_identity().send().await;
        match result {
            Ok(_) => Ok(()),
            Err(sdk_error) => {
                let error_meta = sdk_error.meta();
                let error_code = error_meta.code();
                match error_code {
                    Some("InvalidClientTokenId") => {
                        Err(Report::new(CredentialsError::InvalidCredentialsError))
                    }
                    _ => {
                        let error_code = error_code.unwrap_or("UnknownError");
                        Err(Report::new(CredentialsError::UnexpectedError(
                            error_code.to_string(),
                        )))
                    }
                }
            }
        }
    }
}
