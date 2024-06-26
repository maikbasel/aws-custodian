// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri_plugin_log::LogTarget;

use backend::__cmd__create_profile;
use backend::__cmd__delete_profile;
use backend::__cmd__delete_profiles;
use backend::__cmd__edit_profile;
use backend::__cmd__get_profiles;
use backend::__cmd__validate_credentials;
use backend::credentials::application::tauri::credentials_handler::validate_credentials;
use backend::credentials::core::api::CredentialsDataAPI;
use backend::credentials::core::credentials_service::CredentialsService;
use backend::credentials::infrastructure::aws::sts::sts_adapter::STSAdapter;
use backend::profiles::application::tauri::profile_handler::{
    create_profile, delete_profile, delete_profiles, edit_profile, get_profiles,
};
use backend::profiles::core::api::ProfileDataAPI;
use backend::profiles::core::profile_service::ProfileService;
use backend::profiles::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

#[cfg(not(tarpaulin_include))]
fn main() {
    env_logger::init();

    let profile_data_spi = SdkConfigAdapter;
    let profile_data_api = ProfileService::new(Box::new(profile_data_spi));
    let credentials_data_api = CredentialsService::new(Box::new(STSAdapter));

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .manage(Arc::new(profile_data_api) as Arc<dyn ProfileDataAPI>)
        .manage(Arc::new(credentials_data_api) as Arc<dyn CredentialsDataAPI>)
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            create_profile,
            edit_profile,
            delete_profile,
            validate_credentials,
            delete_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
