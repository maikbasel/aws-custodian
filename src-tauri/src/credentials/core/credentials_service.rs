use async_trait::async_trait;
use error_stack::Report;
#[cfg(test)]
use mockall::predicate::*;

use crate::credentials::core::api::CredentialsDataAPI;
use crate::credentials::core::error::CredentialsError;
use crate::credentials::core::spi::CredentialsDataSPI;

pub struct CredentialsService {
    credentials_data_spi: Box<dyn CredentialsDataSPI>,
}

impl CredentialsService {
    pub fn new(credentials_data_spi: Box<dyn CredentialsDataSPI>) -> Self {
        Self {
            credentials_data_spi,
        }
    }
}

#[async_trait]
impl CredentialsDataAPI for CredentialsService {
    async fn validate_credentials(
        &self,
        profile_name: &str,
    ) -> error_stack::Result<bool, CredentialsError> {
        let result: error_stack::Result<(), CredentialsError> = self
            .credentials_data_spi
            .get_caller_identity(profile_name)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(error) => match error.current_context() {
                CredentialsError::InvalidCredentialsError => Ok(false),
                CredentialsError::UnexpectedError(error_code) => Err(Report::new(
                    CredentialsError::UnexpectedError(error_code.to_string()),
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::credentials::core::spi::MockCredentialsDataSPI;
    use crate::profile::core::error::ProfileError;

    use super::*;

    #[tokio::test]
    async fn should_return_true_for_valid_credentials() {
        let mut credentialsDataAPIMock = MockCredentialsDataSPI::new();
        credentialsDataAPIMock
            .expect_get_caller_identity()
            .returning(|_| Ok(()));
        let cut = CredentialsService::new(Box::new(credentialsDataAPIMock));

        let result = cut.validate_credentials("dev").await;

        assert_that!(result).is_ok().is_equal_to(true);
    }

    #[tokio::test]
    async fn should_return_false_for_invalid_credentials() {
        let mut credentialsDataAPIMock = MockCredentialsDataSPI::new();
        credentialsDataAPIMock
            .expect_get_caller_identity()
            .returning(|_| Err(Report::from(CredentialsError::InvalidCredentialsError)));
        let cut = CredentialsService::new(Box::new(credentialsDataAPIMock));

        let result = cut.validate_credentials("dev").await;

        assert_that(&result).is_ok().is_equal_to(false);
    }

    #[tokio::test]
    async fn should_return_error_when_unexpected_error_occurs() {
        let mut credentialsDataAPIMock = MockCredentialsDataSPI::new();
        credentialsDataAPIMock
            .expect_get_caller_identity()
            .returning(|_| {
                Err(Report::from(CredentialsError::UnexpectedError(
                    "Test".to_string(),
                )))
            });
        let cut = CredentialsService::new(Box::new(credentialsDataAPIMock));

        let result = cut.validate_credentials("dev").await;

        assert_that(&result).is_err();
        let report = result.unwrap_err();
        assert!(report.contains::<CredentialsError>());
    }
}
