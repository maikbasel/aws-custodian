use std::sync::Arc;

use crate::profile::core::domain::{Profile, ProfileSet};
use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    spi: tauri::State<'_, Arc<dyn ProfileDataSPI>>,
) -> Result<ProfileSet, ProfileError> {
    spi.load_profile_data().await.map_err(ProfileError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn create_profile(
    spi: tauri::State<'_, Arc<dyn ProfileDataSPI>>,
    profile: Profile,
) -> Result<(), ProfileError> {
    log::info!("create_profile: {:?}", profile);
    spi.save_profile_data(&profile).map_err(ProfileError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn edit_profile(
    spi: tauri::State<'_, Arc<dyn ProfileDataSPI>>,
    profile: Profile,
) -> Result<(), ProfileError> {
    log::info!("edit_profile: {:#?}", profile);
    spi.update_profile_data(&profile)
        .map_err(ProfileError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn delete_profile(
    spi: tauri::State<'_, Arc<dyn ProfileDataSPI>>,
    profile_name: String,
) -> Result<(), ProfileError> {
    log::info!("delete_profile: {}", profile_name);
    spi.remove_profile_data(&profile_name)
        .map_err(ProfileError::from)
}
