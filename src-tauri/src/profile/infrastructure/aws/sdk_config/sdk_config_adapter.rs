use async_trait::async_trait;
use aws_config::profile::Profile;
use error_stack::Report;
use secstr::SecStr;

use crate::profile::core::domain::{Config, Credentials, ProfileSet, Settings};
use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;

pub struct SdkConfigAdapter;

#[async_trait]
impl ProfileDataSPI for SdkConfigAdapter {
    async fn load_profile_data(&self) -> error_stack::Result<ProfileSet, ProfileError> {
        // See https://docs.rs/aws-config/latest/aws_config/profile/index.html
        let result = aws_config::profile::load(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            None,
        )
        .await;

        match result {
            Ok(profile_set) => {
                let profile_names = profile_set.profiles();
                let mut configuration = ProfileSet::new();

                for profile_name in profile_names {
                    if let Some(profile) = profile_set.get_profile(profile_name) {
                        let config = extract_config(profile);
                        let credentials = extract_credentials(profile);

                        let settings = Settings::new(credentials, config);

                        if let Err(e) = configuration.add_profile(profile_name, settings) {
                            configuration.errors.push(e);
                        }
                    } else {
                        panic!(
                            "profile set should contain profile name: `{}`",
                            profile_name
                        )
                    }
                }

                Ok(configuration)
            }
            Err(e) => Err(Report::from(e).change_context(ProfileError::ProfileDataLoadError)),
        }
    }
}

fn extract_config(profile: &Profile) -> Config {
    let output_format = profile.get("output").map(|value| value.to_string());
    let region = profile.get("region").map(|value| value.to_string());

    Config::new(region, output_format)
}

fn extract_credentials(profile: &Profile) -> Credentials {
    let access_key_id = profile
        .get("aws_access_key_id")
        .map(|value| value.to_string());
    let secret_access_key = profile.get("aws_secret_access_key").map(SecStr::from);

    Credentials::new(access_key_id, secret_access_key)
}
