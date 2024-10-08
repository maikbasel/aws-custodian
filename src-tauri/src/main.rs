// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use backend::__cmd__create_profile;
use backend::__cmd__delete_profile;
use backend::__cmd__delete_profiles;
use backend::__cmd__edit_profile;
use backend::__cmd__get_available_parameters;
use backend::__cmd__get_parameters;
use backend::__cmd__get_profiles;
use backend::__cmd__set_parameter;
use backend::__cmd__validate_credentials;
use backend::credentials::application::tauri::credentials_handler::validate_credentials;
use backend::credentials::core::api::CredentialsDataAPI;
use backend::credentials::core::credentials_service::CredentialsService;
use backend::credentials::infrastructure::aws::sts::sts_adapter::STSAdapter;
use backend::parameters::application::tauri::parameters_handler::{
    get_available_parameters, get_parameters, set_parameter,
};
use backend::parameters::core::api::ParameterDataAPI;
use backend::parameters::core::parameter_service::ParameterService;
use backend::parameters::infrastructure::aws::ssm::parameter_store_adapter::ParameterStoreAdapter;
use backend::profiles::application::tauri::profile_handler::{
    create_profile, delete_profile, delete_profiles, edit_profile, get_profiles,
};
use backend::profiles::core::api::ProfileDataAPI;
use backend::profiles::core::profile_service::ProfileService;
use backend::profiles::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;
use std::sync::Arc;

#[allow(unused_assignments)]
#[cfg(not(tarpaulin_include))]
fn main() {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        let devtools = devtools::init();

        builder = builder.plugin(devtools);
    }

    #[cfg(not(debug_assertions))]
    {
        use tauri_plugin_log::{Builder, LogTarget};

        let log_plugin = Builder::default()
            .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
            .level_for("tracing", log::LevelFilter::Error)
            .level_for("tauri", log::LevelFilter::Error)
            .level_for("aws_config", log::LevelFilter::Error)
            .level(log::LevelFilter::Info)
            .build();

        builder = builder.plugin(log_plugin);
    }

    let profile_data_spi = SdkConfigAdapter;
    let profile_data_api = ProfileService::new(Box::new(profile_data_spi));
    let credentials_data_api = CredentialsService::new(Box::new(STSAdapter));
    let parameter_data_spi = ParameterStoreAdapter;
    let parameter_data_api = ParameterService::new(Box::new(parameter_data_spi));

    builder
        .manage(Arc::new(profile_data_api) as Arc<dyn ProfileDataAPI>)
        .manage(Arc::new(credentials_data_api) as Arc<dyn CredentialsDataAPI>)
        .manage(Arc::new(parameter_data_api) as Arc<dyn ParameterDataAPI>)
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            create_profile,
            edit_profile,
            delete_profile,
            validate_credentials,
            delete_profiles,
            get_available_parameters,
            get_parameters,
            set_parameter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
