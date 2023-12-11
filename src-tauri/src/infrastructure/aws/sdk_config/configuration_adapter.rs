use async_trait::async_trait;
use aws_config::profile::Profile;
use error_stack::Report;
use secstr::SecStr;

use crate::core::configuration::domain::{Config, Configuration, Credentials, Settings};
use crate::core::configuration::error::ConfigurationError;
use crate::core::configuration::spi::ConfigurationSPI;

pub struct ConfigurationAdapter;

#[async_trait]
impl ConfigurationSPI for ConfigurationAdapter {
    async fn load_configuration(&self) -> error_stack::Result<Configuration, ConfigurationError> {
        // See https://docs.rs/aws-config/latest/aws_config/profile/index.html
        let result = aws_config::profile::load(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            None,
        ).await;

        match result {
            Ok(profile_set) => {
                let profile_names = profile_set.profiles();
                let mut configuration = Configuration::new();

                for profile_name in profile_names {
                    if let Some(profile) = profile_set.get_profile(profile_name) {
                        let config = extract_config(profile);
                        let credentials = extract_credentials(profile);

                        let settings = Settings::new(credentials, config);

                        if let Err(e) = configuration.add_profile(profile_name, settings) {
                            configuration.errors.push(e);
                        }
                    } else {
                        panic!("profile set should contain profile name: `{}`", profile_name)
                    }
                }

                Ok(configuration)
            }
            Err(e) => {
                Err(Report::from(e)
                    .change_context(ConfigurationError::ProfileLoadError))
            }
        }
    }
}

fn extract_config(profile: &Profile) -> Config {
    let output_format = profile.get("output")
        .map(|value| value.to_string());
    let region = profile.get("region")
        .map(|value| value.to_string());
    let config = Config::new(region, output_format);
    config
}

fn extract_credentials(profile: &Profile) -> Credentials {
    let access_key_id = profile.get("aws_access_key_id")
        .map(|value| value.to_string());
    let secret_access_key = profile.get("aws_secret_access_key")
        .map(|value| SecStr::from(value));
    let credentials = Credentials::new(access_key_id, secret_access_key);

    credentials
}