use async_trait::async_trait;
use aws_config::profile::Profile;
use error_stack::Report;
use secstr::SecStr;

use crate::core::profile::domain::{Config, ConfigProfiles, Credentials, Settings};
use crate::core::profile::error::ConfigProfilesError;
use crate::core::profile::spi::ConfigProfilesSPI;

pub struct ConfigProfilesAdapter;

#[async_trait]
impl ConfigProfilesSPI for ConfigProfilesAdapter {
    async fn load_config_profiles(&self) -> error_stack::Result<ConfigProfiles, ConfigProfilesError> {
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
                let mut configuration = ConfigProfiles::new();

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
                    .change_context(ConfigProfilesError::ConfigLoadError))
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