use std::sync::Arc;

use crate::profile::core::api::ProfileAPI;
use crate::profile::core::domain::ProfileSet;
use crate::profile::core::error::ProfileError;

#[derive(serde::Serialize)]
pub struct ErrorResponse {}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    api: tauri::State<'_, Arc<dyn ProfileAPI>>,
) -> Result<ProfileSet, ProfileError> {
    let result = api.get_profiles().await;
    if let Ok(profile_set) = result {
        profile_set
    } else if let Err(e) = result {
        e.downcast_ref::<ProfileError>()
    }
}
