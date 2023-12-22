use std::sync::Arc;

use crate::profile::core::api::ProfileAPI;
use crate::profile::core::domain::ProfileSet;

#[derive(serde::Serialize)]
pub struct ErrorResponse {}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_profiles(
    _api: tauri::State<'_, Arc<dyn ProfileAPI>>,
) -> Result<ProfileSet, ErrorResponse> {
    todo!()
}
