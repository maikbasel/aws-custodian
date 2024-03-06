// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use backend::__cmd__get_profiles;
use backend::__cmd__validate_credentials;
use backend::credentials::application::tauri::credentials_handler::validate_credentials;
use backend::credentials::core::api::CredentialsDataAPI;
use backend::credentials::core::credentials_service::CredentialsService;
use backend::credentials::infrastructure::aws::sts::sts_adapter::STSAdapter;
use backend::profile::application::tauri::profile_handler::get_profiles;
use backend::profile::core::spi::ProfileDataSPI;
use backend::profile::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

#[cfg(not(tarpaulin_include))]
fn main() {
    let profile_data_spi = SdkConfigAdapter;
    let credentials_data_api = CredentialsService::new(Box::new(STSAdapter));

    tauri::Builder::default()
        .manage(Arc::new(profile_data_spi) as Arc<dyn ProfileDataSPI>)
        .manage(Arc::new(credentials_data_api) as Arc<dyn CredentialsDataAPI>)
        .invoke_handler(tauri::generate_handler![get_profiles, validate_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
