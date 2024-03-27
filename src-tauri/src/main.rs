// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri_plugin_log::LogTarget;

use backend::__cmd__create_profile;
use backend::__cmd__delete_profile;
use backend::__cmd__edit_profile;
use backend::__cmd__get_profiles;
use backend::__cmd__validate_credentials;
use backend::credentials::application::tauri::credentials_handler::validate_credentials;
use backend::credentials::core::api::CredentialsDataAPI;
use backend::credentials::core::credentials_service::CredentialsService;
use backend::credentials::infrastructure::aws::sts::sts_adapter::STSAdapter;
use backend::profile::application::tauri::profile_handler::{
    create_profile, delete_profile, edit_profile, get_profiles,
};
use backend::profile::core::spi::ProfileDataSPI;
use backend::profile::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

#[cfg(not(tarpaulin_include))]
fn main() {
    let profile_data_spi = SdkConfigAdapter;
    let credentials_data_api = CredentialsService::new(Box::new(STSAdapter));

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .manage(Arc::new(profile_data_spi) as Arc<dyn ProfileDataSPI>)
        .manage(Arc::new(credentials_data_api) as Arc<dyn CredentialsDataAPI>)
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            create_profile,
            edit_profile,
            delete_profile,
            validate_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
