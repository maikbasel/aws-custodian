use crate::profiles::core::api::ProfileDataAPI;
use std::sync::Arc;

use crate::profiles::core::domain::{Profile, ProfileSet};
use crate::profiles::core::error::ProfileDataError;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    api: tauri::State<'_, Arc<dyn ProfileDataAPI>>,
) -> Result<ProfileSet, ProfileDataError> {
    api.get_profiles().await.map_err(ProfileDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn create_profile(
    api: tauri::State<'_, Arc<dyn ProfileDataAPI>>,
    profile: Profile,
) -> Result<(), ProfileDataError> {
    log::info!("create_profile: {:?}", profile);
    api.create_profile(&profile).map_err(ProfileDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn edit_profile(
    api: tauri::State<'_, Arc<dyn ProfileDataAPI>>,
    profile: Profile,
) -> Result<(), ProfileDataError> {
    log::info!("edit_profile: {:#?}", profile);
    api.edit_profile(&profile).map_err(ProfileDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn delete_profile(
    api: tauri::State<'_, Arc<dyn ProfileDataAPI>>,
    profile_name: String,
) -> Result<(), ProfileDataError> {
    log::info!("delete_profile: {}", profile_name);
    api.delete_profile(&profile_name)
        .map_err(ProfileDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn delete_profiles(
    api: tauri::State<'_, Arc<dyn ProfileDataAPI>>,
    profile_names: Vec<String>,
) -> Result<(), ProfileDataError> {
    log::info!("delete_profiles: {:?}", profile_names);
    api.delete_profiles(profile_names.as_slice())
        .map_err(ProfileDataError::from)
}
