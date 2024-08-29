use std::time::Duration;
use aws_config::{BehaviorVersion, ConfigLoader};
use aws_sdk_ssm::config::Builder as SsmBuilder;
use aws_sdk_ssm::Client as SsmClient;
use aws_sdk_ssm::config::retry::RetryConfig;
use aws_sdk_sts::config::Builder as StsBuilder;
use aws_sdk_sts::Client as StsClient;

pub fn localstack_endpoint() -> Option<String> {
    match std::env::var("LOCALSTACK_ENDPOINT") {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn shared_config_loader(profile_name: &str) -> ConfigLoader {
    aws_config::defaults(BehaviorVersion::latest()).credentials_provider(
        aws_config::profile::ProfileFileCredentialsProvider::builder()
            .profile_name(profile_name)
            .build(),
    )
}

pub fn sts_client(config: &aws_config::SdkConfig) -> StsClient {
    // Copy config from aws_config::SdkConfig to aws_sdk_sts::Config
    let sts_config_builder = StsBuilder::from(config);

    StsClient::from_conf(sts_config_builder.build())
}

pub fn ssm_client(config: &aws_config::SdkConfig) -> SsmClient {
    let retry_config = RetryConfig::adaptive()
        .with_max_attempts(8)
        .with_initial_backoff(Duration::from_millis(200))
        .with_max_backoff(Duration::from_secs(5));
    let sts_config_builder = SsmBuilder::from(config).retry_config(retry_config);

    SsmClient::from_conf(sts_config_builder.build())
}
