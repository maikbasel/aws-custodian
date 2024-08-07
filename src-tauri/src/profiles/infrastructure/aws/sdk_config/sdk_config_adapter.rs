use std::env;

use async_trait::async_trait;
use aws_config::profile::Profile;
use directories::UserDirs;
use error_stack::{Report, ResultExt};
use ini::Ini;

use crate::common::secure_string::SecureString;
use crate::profiles::core::domain::{Config, Credentials, Profile as DomainProfile, ProfileSet};
use crate::profiles::core::error::ProfileDataError;
use crate::profiles::core::spi::ProfileDataSPI;

pub struct SdkConfigAdapter;

#[async_trait]
impl ProfileDataSPI for SdkConfigAdapter {
    async fn load_profile_data(&self) -> error_stack::Result<ProfileSet, ProfileDataError> {
        let result = aws_config::profile::load(
            // See https://docs.rs/aws-config/latest/aws_config/profile/index.html
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
                    if let Some(sdk_profile) = profile_set.get_profile(profile_name) {
                        let config = Self::extract_config(sdk_profile);
                        let credentials = Self::extract_credentials(sdk_profile);

                        let profile =
                            DomainProfile::new(profile_name.to_string(), credentials, config);

                        configuration.add_profile(profile);
                    } else {
                        panic!(
                            "profile set should contain profile name: `{}`",
                            profile_name
                        )
                    }
                }

                Ok(configuration)
            }
            Err(e) => Err(Report::from(e).change_context(ProfileDataError::ProfileDataLoadError)),
        }
    }

    fn save_profile_data(
        &self,
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        Self::create_profile_in_config_file(profile)?;

        Self::create_profile_in_credentials_file(profile)?;

        Ok(())
    }

    fn remove_profile_data(&self, profile_name: &str) -> error_stack::Result<(), ProfileDataError> {
        Self::delete_from_config(profile_name)?;

        Self::delete_from_credentials_file(profile_name)?;

        Ok(())
    }

    fn remove_profiles_data(
        &self,
        profile_names: &[String],
    ) -> error_stack::Result<(), ProfileDataError> {
        // Might need some optimization in the future.
        for profile_name in profile_names {
            Self::delete_from_config(profile_name)?;
            Self::delete_from_credentials_file(profile_name)?;
        }

        Ok(())
    }

    fn update_profile_data(
        &self,
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        Self::update_profile_in_config_file(profile)?;

        Self::update_profile_in_credentials_file(profile)?;

        Ok(())
    }
}

impl SdkConfigAdapter {
    fn get_config_file_location() -> error_stack::Result<String, ProfileDataError> {
        let user_dir = UserDirs::new().expect("user dir should exist");
        let default_aws_config_file_location = user_dir.home_dir().join(".aws").join("config");

        let config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap_or(
            default_aws_config_file_location
                .to_string_lossy()
                .into_owned(),
        );

        Ok(config_file_location)
    }

    fn get_credentials_file_location() -> error_stack::Result<String, ProfileDataError> {
        let user_dir = UserDirs::new().expect("user dir should exist");

        let default_aws_credentials_file_location =
            user_dir.home_dir().join(".aws").join("credentials");

        let credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap_or(
            default_aws_credentials_file_location
                .to_string_lossy()
                .into_owned(),
        );

        Ok(credentials_file_location)
    }

    fn delete_from_credentials_file(
        profile_name: &str,
    ) -> error_stack::Result<(), ProfileDataError> {
        let credentials_file_location = Self::get_credentials_file_location()?;
        let mut config_file = Self::load_credentials_file(&credentials_file_location)?;

        config_file.delete(Some(profile_name));

        config_file
            .write_to_file(credentials_file_location.as_str())
            .change_context(ProfileDataError::CredentialsFileWriteError)?;
        Ok(())
    }

    fn delete_from_config(profile_name: &str) -> error_stack::Result<(), ProfileDataError> {
        let config_file_location = Self::get_config_file_location()?;
        let mut config_file = Self::load_config_file(&config_file_location)?;

        config_file.delete(Some(format!("profile {}", profile_name)));

        config_file
            .write_to_file(config_file_location.as_str())
            .change_context(ProfileDataError::ConfigFileWriteError)?;
        Ok(())
    }

    fn load_credentials_file(
        credentials_file_location: &String,
    ) -> error_stack::Result<Ini, ProfileDataError> {
        Ini::load_from_file(credentials_file_location)
            .change_context(ProfileDataError::CredentialsFileLoadError)
    }

    fn load_config_file(
        config_file_location: &String,
    ) -> error_stack::Result<Ini, ProfileDataError> {
        Ini::load_from_file(config_file_location)
            .change_context(ProfileDataError::ConfigFileLoadError)
    }

    fn create_profile_in_config_file(
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        let config_file_location = Self::get_config_file_location()?;
        let mut config_file = Ini::load_from_file(&config_file_location)
            .change_context(ProfileDataError::ConfigFileLoadError)?;

        let mut profile_section =
            config_file.with_section(Some(format!("profile {}", profile.name)));
        if let (Some(region), Some(output_format)) =
            (&profile.config.region, &profile.config.output_format)
        {
            profile_section
                .set("region", region)
                .set("output", output_format);

            config_file
                .write_to_file(config_file_location.as_str())
                .change_context(ProfileDataError::ConfigFileWriteError)?;
        }
        Ok(())
    }

    fn create_profile_in_credentials_file(
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        let credentials_file_location = Self::get_credentials_file_location()?;
        let mut credentials_file = Ini::load_from_file(&credentials_file_location)
            .change_context(ProfileDataError::ConfigFileLoadError)?;

        let mut profile_section = credentials_file.with_section(Some(profile.name.clone()));
        if let (Some(access_key_id), Some(secret_access_key)) = (
            &profile.credentials.access_key_id,
            &profile.credentials.secret_access_key,
        ) {
            profile_section
                .set("aws_access_key_id", access_key_id)
                .set("aws_secret_access_key", secret_access_key.as_str());

            credentials_file
                .write_to_file(credentials_file_location.as_str())
                .change_context(ProfileDataError::CredentialsFileWriteError)?;
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
        let secret_access_key = profile.get("aws_secret_access_key").map(SecureString::from);

        Credentials::new(access_key_id, secret_access_key)
    }

    fn update_profile_in_config_file(
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        let config_file_location = Self::get_config_file_location()?;
        let mut config_file = Ini::load_from_file(&config_file_location)
            .change_context(ProfileDataError::ConfigFileLoadError)?;

        let profile_section = config_file.section_mut(Some(format!("profile {}", profile.name)));

        if profile_section.is_none() {
            return Err(Report::new(ProfileDataError::ProfileNotFoundError));
        }

        let properties = profile_section.unwrap();
        if let (Some(region), Some(output_format)) =
            (&profile.config.region, &profile.config.output_format)
        {
            properties.insert("region", region);
            properties.insert("output", output_format);

            config_file
                .write_to_file(config_file_location.as_str())
                .change_context(ProfileDataError::ConfigFileWriteError)?;
        }

        Ok(())
    }

    fn update_profile_in_credentials_file(
        profile: &DomainProfile,
    ) -> error_stack::Result<(), ProfileDataError> {
        let credentials_file_location = Self::get_credentials_file_location()?;
        let mut credentials_file = Ini::load_from_file(&credentials_file_location)
            .change_context(ProfileDataError::ConfigFileLoadError)?;

        let profile_section = credentials_file.section_mut(Some(profile.name.clone()));

        if profile_section.is_none() {
            return Err(Report::new(ProfileDataError::ProfileNotFoundError));
        }

        let properties = profile_section.unwrap();
        if let (Some(access_key_id), Some(secret_access_key)) = (
            &profile.credentials.access_key_id,
            &profile.credentials.secret_access_key,
        ) {
            properties.insert("aws_access_key_id", access_key_id);
            properties.insert("aws_secret_access_key", secret_access_key.as_str());

            credentials_file
                .write_to_file(credentials_file_location.as_str())
                .change_context(ProfileDataError::ConfigFileWriteError)?;
        }

        Ok(())
    }
}
