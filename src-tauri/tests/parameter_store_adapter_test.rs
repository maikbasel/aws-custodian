#[cfg(test)]
mod tests {
    use std::{env, fs};

    use directories::UserDirs;
    use ini::Ini;
    use serial_test::serial;
    use spectral::prelude::*;
    use tempfile::{tempdir, TempDir};
    use test_context::{test_context, AsyncTestContext};
    use testcontainers::core::{ExecCommand, WaitFor};
    use testcontainers::runners::AsyncRunner;
    use testcontainers::RunnableImage;
    use testcontainers_modules::localstack::LocalStack;

    use backend::parameters::core::domain::ParameterValue;
    use backend::parameters::core::spi::ParameterDataSPI;
    use backend::parameters::infrastructure::aws::ssm::parameter_store_adapter::ParameterStoreAdapter;

    struct TestContext {
        _test_dir: TempDir,
        profile: String,
        original_config_file_location: String,
        original_credentials_file_location: String,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for TestContext {
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

            let profile_name = "dev";

            let mut test_config = Ini::new();
            test_config
                .with_section(Some(format!("profile {}", profile_name)))
                .set("region", "eu-west-1")
                .set("output", "json");
            let test_config_file_path = test_aws_dir_path.join("config");
            test_config.write_to_file(&test_config_file_path).unwrap();
            env::set_var("AWS_CONFIG_FILE", test_config_file_path);

            let mut test_credentials = Ini::new();
            test_credentials
                .with_section(Some(profile_name))
                .set("aws_access_key_id", "devAccessKeyID")
                .set("aws_secret_access_key", "devSecretAccessKey");
            let test_credentials_file_path = test_aws_dir_path.join("credentials");
            test_credentials
                .write_to_file(&test_credentials_file_path)
                .unwrap();
            env::set_var("AWS_SHARED_CREDENTIALS_FILE", test_credentials_file_path);

            TestContext {
                _test_dir: test_dir,
                profile: profile_name.to_string(),
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

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_load_available_parameter_names(ctx: &mut TestContext) {
        let localstack: RunnableImage<LocalStack> = LocalStack.into();
        let localstack = localstack.with_env_var(("SERVICES", "ssm"));
        let localstack_container = localstack.start().await;
        localstack_container
            .exec(
                ExecCommand::new(vec![
                    "awslocal",
                    "ssm",
                    "put-parameter",
                    "--name",
                    "key1",
                    "--value",
                    "val1",
                    "--type",
                    "String",
                ])
                .with_cmd_ready_condition(WaitFor::Healthcheck),
            )
            .await;
        localstack_container
            .exec(
                ExecCommand::new(vec![
                    "awslocal",
                    "ssm",
                    "put-parameter",
                    "--name",
                    "key2",
                    "--value",
                    "val2",
                    "--type",
                    "String",
                ])
                .with_cmd_ready_condition(WaitFor::Healthcheck),
            )
            .await;
        let host_port = localstack_container.get_host_port_ipv4(4566).await;
        let endpoint_url = format!("http://127.0.0.1:{host_port}");
        env::set_var("LOCALSTACK_ENDPOINT", endpoint_url);
        let cut: Box<dyn ParameterDataSPI> = Box::new(ParameterStoreAdapter);

        let actual = cut.load_available_parameter_names(&ctx.profile, 1).await;

        assert_that!(actual).is_ok();
        let actual_names = actual.unwrap();
        assert_that!(actual_names).contains("key1".to_string());
        assert_that!(actual_names).contains("key2".to_string());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_load_available_parameters(ctx: &mut TestContext) {
        let localstack: RunnableImage<LocalStack> = LocalStack.into();
        let localstack = localstack.with_env_var(("SERVICES", "ssm"));
        let localstack_container = localstack.start().await;
        localstack_container
            .exec(
                ExecCommand::new(vec![
                    "awslocal",
                    "ssm",
                    "put-parameter",
                    "--name",
                    "key1",
                    "--value",
                    "val1",
                    "--type",
                    "String",
                ])
                .with_cmd_ready_condition(WaitFor::Healthcheck),
            )
            .await;
        localstack_container
            .exec(
                ExecCommand::new(vec![
                    "awslocal",
                    "ssm",
                    "put-parameter",
                    "--name",
                    "key2",
                    "--value",
                    "val2",
                    "--type",
                    "String",
                ])
                .with_cmd_ready_condition(WaitFor::Healthcheck),
            )
            .await;
        let host_port = localstack_container.get_host_port_ipv4(4566).await;
        let endpoint_url = format!("http://127.0.0.1:{host_port}");
        env::set_var("LOCALSTACK_ENDPOINT", endpoint_url);
        let cut: Box<dyn ParameterDataSPI> = Box::new(ParameterStoreAdapter);

        let actual = cut
            .load_parameters(&ctx.profile, vec!["key1".to_string(), "key2".to_string()])
            .await;

        assert_that!(actual).is_ok();
        let actual_parameters = actual.unwrap();
        assert_that!(actual_parameters).has_length(2);

        let value1 = actual_parameters[0].clone();
        let value2 = actual_parameters[1].clone();

        // TODO: Should probably not depend on correct order.
        assert_that!(value1.name).is_equal_to("key2".to_string());
        assert_that!(value1.value).is_equal_to(&ParameterValue::String("val2".to_string()));
        assert_that!(value2.name).is_equal_to("key1".to_string());
        assert_that!(value2.value).is_equal_to(&ParameterValue::String("val1".to_string()));
    }
}
