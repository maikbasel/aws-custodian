use aws_config::BehaviorVersion;

pub fn localstack_endpoint() -> Option<String> {
    match std::env::var("LOCALSTACK_ENDPOINT") {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn shared_config_loader(profile_name: &str) -> aws_config::ConfigLoader {
    aws_config::defaults(BehaviorVersion::latest()).credentials_provider(
        aws_config::profile::ProfileFileCredentialsProvider::builder()
            .profile_name(profile_name)
            .build(),
    )
}

pub fn sts_client(config: &aws_config::SdkConfig) -> aws_sdk_sts::Client {
    // Copy config from aws_config::SdkConfig to aws_sdk_sts::Config
    let sts_config_builder = aws_sdk_sts::config::Builder::from(config);

    aws_sdk_sts::Client::from_conf(sts_config_builder.build())
}
