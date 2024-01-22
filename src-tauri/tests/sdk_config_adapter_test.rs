#[cfg(test)]
mod tests {
    use std::{env, fs};

    use ini::Ini;
    use secstr::SecStr;
    use serial_test::serial;
    use spectral::prelude::*;
    use tempfile::{tempdir, TempDir};
    use test_context::{test_context, AsyncTestContext};

    use backend::profile::core::domain::{Config, Credentials, Settings};
    use backend::profile::core::error::ProfileError;
    use backend::profile::core::spi::ProfileDataSPI;
    use backend::profile::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

    struct ValidContext {
        _test_dir: TempDir,
        original_config_file_location: Option<String>,
        original_credentials_file_location: Option<String>,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for ValidContext {
        async fn setup() -> Self {
            let original_config_file_location = env::var("AWS_CONFIG_FILE").ok();
            let original_credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok();

            let test_dir = tempdir().unwrap();
            let test_aws_dir_path = test_dir.path().join(".aws");
            fs::create_dir_all(&test_aws_dir_path).unwrap();

            let mut test_config = Ini::new();
            test_config
                .with_section(Some("profile dev"))
                .set("region", "eu-west-1")
                .set("output", "json");
            let test_config_file_path = test_aws_dir_path.join("config");
            test_config.write_to_file(&test_config_file_path).unwrap();
            env::set_var("AWS_CONFIG_FILE", test_config_file_path);

            let mut test_credentials = Ini::new();
            test_credentials
                .with_section(Some("dev"))
                .set("aws_access_key_id", "devAccessKeyID")
                .set("aws_secret_access_key", "devSecretAccessKey");
            let test_credentials_file_path = test_aws_dir_path.join("credentials");
            test_credentials
                .write_to_file(&test_credentials_file_path)
                .unwrap();
            env::set_var("AWS_SHARED_CREDENTIALS_FILE", test_credentials_file_path);

            ValidContext {
                _test_dir: test_dir,
                original_config_file_location,
                original_credentials_file_location,
            }
        }

        async fn teardown(self) {
            env::set_var(
                "AWS_CONFIG_FILE",
                self.original_config_file_location.as_deref().unwrap_or(""),
            );
            env::set_var(
                "AWS_SHARED_CREDENTIALS_FILE",
                self.original_credentials_file_location
                    .as_deref()
                    .unwrap_or(""),
            );
        }
    }

    struct InvalidContext {
        _test_dir: TempDir,
        original_config_file_location: Option<String>,
        original_credentials_file_location: Option<String>,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for InvalidContext {
        async fn setup() -> Self {
            let original_config_file_location = env::var("AWS_CONFIG_FILE").ok();
            let original_credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok();

            let test_dir = tempdir().unwrap();
            let test_aws_dir_path = test_dir.path().join(".aws");
            fs::create_dir_all(&test_aws_dir_path).unwrap();

            let mut test_config = Ini::new();
            test_config
                .with_section(Some(""))
                .set("region", "eu-west-1")
                .set("output", "json");
            let test_config_file_path = test_aws_dir_path.join("config");
            test_config.write_to_file(&test_config_file_path).unwrap();
            env::set_var("AWS_CONFIG_FILE", test_config_file_path);

            let mut test_credentials = Ini::new();
            test_credentials
                .with_section(Some(""))
                .set("aws_access_key_id", "devAccessKeyID")
                .set("aws_secret_access_key", "devSecretAccessKey");
            let test_credentials_file_path = test_aws_dir_path.join("credentials");
            test_credentials
                .write_to_file(&test_credentials_file_path)
                .unwrap();
            env::set_var("AWS_SHARED_CREDENTIALS_FILE", test_credentials_file_path);

            InvalidContext {
                _test_dir: test_dir,
                original_config_file_location,
                original_credentials_file_location,
            }
        }

        async fn teardown(self) {
            env::set_var(
                "AWS_CONFIG_FILE",
                self.original_config_file_location.as_deref().unwrap_or(""),
            );
            env::set_var(
                "AWS_SHARED_CREDENTIALS_FILE",
                self.original_credentials_file_location
                    .as_deref()
                    .unwrap_or(""),
            );
        }
    }

    #[test_context(ValidContext)]
    #[tokio::test]
    #[serial]
    async fn should_load_config_from_environment(_: &mut ValidContext) {
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let dev_settings = Settings::new(
            Credentials::new(
                Some("devAccessKeyID"),
                Some(SecStr::from("devSecretAccessKey")),
            ),
            Config::new(Some("eu-west-1"), Some("json")),
        );

        let result = cut.load_profile_data().await;

        assert_that(&result).is_ok();
        let actual = result.unwrap();
        let actual_profiles = actual.profiles();
        assert_that(actual_profiles).contains_entry(&"dev".to_string(), &dev_settings);
    }

    #[test_context(InvalidContext)]
    #[tokio::test]
    #[serial]
    async fn should_have_errors_when_loading_config_with_invalid_profile_name(
        _: &mut InvalidContext,
    ) {
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);

        let result = cut.load_profile_data().await;

        assert_that(&result).is_ok();
        let config = result.unwrap();
        assert_that(&config.errors).has_length(1);
        let is_invalid_profile_name_error =
            config.errors.get(0).unwrap().contains::<ProfileError>();
        assert_that(&is_invalid_profile_name_error).is_true();
        let actual = config
            .errors
            .get(0)
            .unwrap()
            .downcast_ref::<ProfileError>()
            .unwrap();
        assert_that(&actual).is_equal_to(&ProfileError::InvalidProfileNameError);
    }
}
