use std::env;

use async_trait::async_trait;
use aws_config::profile::Profile;
use directories::UserDirs;
use error_stack::{Report, ResultExt};
use ini::Ini;
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

    fn save_profile_data(
        &self,
        profile_name: &str,
        settings: &Settings,
    ) -> error_stack::Result<(), ProfileError> {
        update_config_file(profile_name, settings)?;

        update_credentials_file(profile_name, settings)?;

        Ok(())
    }
}

fn update_config_file(profile_name: &str, settings: &Settings) -> Result<(), Report<ProfileError>> {
    let user_dir = UserDirs::new().expect("user dir should exist");

    let default_aws_config_file_location = user_dir.home_dir().join(".aws").join("config");
    let config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap_or(
        default_aws_config_file_location
            .to_string_lossy()
            .into_owned(),
    );
    let mut config_file = Ini::load_from_file(&config_file_location)
        .change_context(ProfileError::ConfigFileLoadError)?;

    let mut profile_section = config_file.with_section(Some(format!("profile {}", profile_name)));
    if let (Some(region), Some(output_format)) =
        (&settings.config.region, &settings.config.output_format)
    {
        profile_section
            .set("region", region)
            .set("output", output_format);

        config_file
            .write_to_file(config_file_location.as_str())
            .change_context(ProfileError::ConfigFileWriteError)?;
    }
    Ok(())
}

fn update_credentials_file(
    profile_name: &str,
    settings: &Settings,
) -> Result<(), Report<ProfileError>> {
    let user_dir = UserDirs::new().expect("user dir should exist");
    let default_aws_credentials_file_location =
        user_dir.home_dir().join(".aws").join("credentials");
    let credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap_or(
        default_aws_credentials_file_location
            .to_string_lossy()
            .into_owned(),
    );
    let mut credentials_file = Ini::load_from_file(&credentials_file_location)
        .change_context(ProfileError::ConfigFileLoadError)?;

    let mut profile_section = credentials_file.with_section(Some(profile_name));
    if let (Some(access_key_id), Some(secret_access_key)) = (
        &settings.credentials.access_key_id,
        &settings.credentials.secret_access_key,
    ) {
        profile_section.set("aws_access_key_id", access_key_id).set(
            "aws_secret_access_key",
            std::str::from_utf8(secret_access_key.unsecure())
                .expect("secret access key should be serializable to be UTF-8 string"),
        );

        credentials_file
            .write_to_file(credentials_file_location.as_str())
            .change_context(ProfileError::CredentialsFileWriteError)?;
    }

    Ok(())
}

fn extract_config(profile: &Profile) -> Config {
    let output_format = profile.get("output");
    let region = profile.get("region");

    Config::new(region, output_format)
}

fn extract_credentials(profile: &Profile) -> Credentials {
    let access_key_id = profile.get("aws_access_key_id");
    let secret_access_key = profile.get("aws_secret_access_key").map(SecStr::from);

    Credentials::new(access_key_id, secret_access_key)
}
