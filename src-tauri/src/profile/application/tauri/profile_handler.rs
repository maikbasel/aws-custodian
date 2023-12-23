use std::sync::Arc;

use crate::profile::core::api::ProfileAPI;
use crate::profile::core::domain::ProfileSet;
use crate::profile::core::error::ProfileError;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    api: tauri::State<'_, Arc<dyn ProfileAPI>>,
) -> Result<ProfileSet, ProfileError> {
    api.get_profiles().await.map_err(ProfileError::from)
}
