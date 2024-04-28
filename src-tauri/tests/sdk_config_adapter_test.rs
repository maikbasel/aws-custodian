#[cfg(test)]
mod tests {
    use std::{env, fs};

    use directories::UserDirs;
    use ini::Ini;
    use secstr::SecStr;
    use serial_test::serial;
    use spectral::prelude::*;
    use tempfile::{tempdir, TempDir};
    use test_context::{test_context, AsyncTestContext};

    use backend::profile::core::domain::{Config, Credentials, Profile};
    use backend::profile::core::spi::ProfileDataSPI;
    use backend::profile::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

    struct ValidContext {
        _test_dir: TempDir,
        original_config_file_location: String,
        original_credentials_file_location: String,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for ValidContext {
        async fn setup() -> Self {
            let user_dir = UserDirs::new().expect("user dir should exist");

            let default_aws_config_file_location = user_dir.home_dir().join(".aws").join("config");
            let original_config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap_or(
                default_aws_config_file_location
                    .to_string_lossy()
                    .into_owned(),
            );

            let default_aws_credentials_file_location =
                user_dir.home_dir().join(".aws").join("credentials");
            let original_credentials_file_location =
                env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap_or(
                    default_aws_credentials_file_location
                        .to_string_lossy()
                        .into_owned(),
                );

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
            env::set_var("AWS_CONFIG_FILE", self.original_config_file_location);
            env::set_var(
                "AWS_SHARED_CREDENTIALS_FILE",
                self.original_credentials_file_location,
            );
        }
    }

    #[test_context(ValidContext)]
    #[tokio::test]
    #[serial]
    async fn should_load_config_from_environment(_: &mut ValidContext) {
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let dev_profile = Profile::new(
            "dev".to_string(),
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
        assert_that(actual_profiles).contains(&dev_profile);
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_create_new_profile(_: &mut ValidContext) {
        let config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let input_profile = Profile::new(
            "new".to_string(),
            Credentials::new(
                Some("newAccessKeyID"),
                Some(SecStr::from("newSecretAccessKey")),
            ),
            Config::new(Some("eu-west-1"), Some("json")),
        );

        let result = cut.save_profile_data(&input_profile);

        assert_that(&result).is_ok();
        let actual_config = Ini::load_from_file(config_file_location).unwrap();
        let actual_profile_section = &actual_config.section(Some("profile new"));
        assert_that(actual_profile_section).is_some();
        assert_that(&actual_profile_section.unwrap().get("region"))
            .is_some()
            .is_equal_to("eu-west-1");
        assert_that(&actual_profile_section.unwrap().get("output"))
            .is_some()
            .is_equal_to("json");
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_create_credentials_for_new_profile(_: &mut ValidContext) {
        let credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let input_profile = Profile::new(
            "new".to_string(),
            Credentials::new(
                Some("newAccessKeyID"),
                Some(SecStr::from("newSecretAccessKey")),
            ),
            Config::new(Some("eu-west-1"), Some("json")),
        );

        let result = cut.save_profile_data(&input_profile);

        assert_that(&result).is_ok();
        let actual_credentials = Ini::load_from_file(credentials_file_location).unwrap();
        let actual_profile_section = &actual_credentials.section(Some("new"));
        assert_that(actual_profile_section).is_some();
        assert_that(&actual_profile_section.unwrap().get("aws_access_key_id"))
            .is_some()
            .is_equal_to("newAccessKeyID");
        assert_that(&actual_profile_section.unwrap().get("aws_secret_access_key"))
            .is_some()
            .is_equal_to("newSecretAccessKey");
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_remove_config_for_given_profile(_: &mut ValidContext) {
        let config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);

        let result = cut.remove_profile_data("dev");

        assert_that(&result).is_ok();
        let actual_config = Ini::load_from_file(config_file_location).unwrap();
        assert_that(&actual_config.section(Some("profile dev"))).is_none();
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_remove_credentials_for_given_profile(_: &mut ValidContext) {
        let credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);

        let result = cut.remove_profile_data("dev");

        assert_that(&result).is_ok();
        let actual_credentials = Ini::load_from_file(credentials_file_location).unwrap();
        assert_that(&actual_credentials.section(Some("dev"))).is_none();
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_update_config_for_given_profile(_: &mut ValidContext) {
        let config_file_location = env::var("AWS_CONFIG_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let input_profile = Profile::new(
            "dev".to_string(),
            Credentials::new(
                Some("newAccessKeyID"),
                Some(SecStr::from("newSecretAccessKey")),
            ),
            Config::new(Some("eu-east-1"), Some("table")),
        );

        let result = cut.update_profile_data(&input_profile);

        assert_that(&result).is_ok();
        let actual_config = Ini::load_from_file(config_file_location).unwrap();
        let actual_profile_section = &actual_config.section(Some("profile dev"));
        assert_that(&actual_profile_section.unwrap().get("region"))
            .is_some()
            .is_equal_to("eu-east-1");
        assert_that(&actual_profile_section.unwrap().get("output"))
            .is_some()
            .is_equal_to("table");
    }

    #[test_context(ValidContext)]
    #[test]
    #[serial]
    fn should_update_credentials_for_given_profile(_: &mut ValidContext) {
        let credentials_file_location = env::var("AWS_SHARED_CREDENTIALS_FILE").ok().unwrap();
        let cut: Box<dyn ProfileDataSPI> = Box::new(SdkConfigAdapter);
        let input_profile = Profile::new(
            "dev".to_string(),
            Credentials::new(
                Some("newAccessKeyID"),
                Some(SecStr::from("newSecretAccessKey")),
            ),
            Config::new(Some("eu-east-1"), Some("table")),
        );

        let result = cut.update_profile_data(&input_profile);

        assert_that(&result).is_ok();
        let actual_credentials = Ini::load_from_file(credentials_file_location).unwrap();
        let actual_profile_section = &actual_credentials.section(Some("dev"));
        assert_that(actual_profile_section).is_some();
        assert_that(&actual_profile_section.unwrap().get("aws_access_key_id"))
            .is_some()
            .is_equal_to("newAccessKeyID");
        assert_that(&actual_profile_section.unwrap().get("aws_secret_access_key"))
            .is_some()
            .is_equal_to("newSecretAccessKey");
    }
}
