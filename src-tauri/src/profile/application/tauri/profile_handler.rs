use std::sync::Arc;

use crate::profile::core::domain::ProfileSet;
use crate::profile::core::error::ProfileError;
use crate::profile::core::spi::ProfileDataSPI;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    spi: tauri::State<'_, Arc<dyn ProfileDataSPI>>,
) -> Result<ProfileSet, ProfileError> {
    spi.load_profile_data().await.map_err(ProfileError::from)
}
