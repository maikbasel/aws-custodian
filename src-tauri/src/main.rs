// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use app::__cmd__get_profiles;
use app::profile::application::tauri::profile_handler::get_profiles;
use app::profile::core::spi::ProfileDataSPI;
use app::profile::infrastructure::aws::sdk_config::sdk_config_adapter::SdkConfigAdapter;

#[cfg(not(tarpaulin_include))]
fn main() {
    let profile_data_spi = SdkConfigAdapter;

    tauri::Builder::default()
        .manage(Arc::new(profile_data_spi) as Arc<dyn ProfileDataSPI>)
        .invoke_handler(tauri::generate_handler![get_profiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
