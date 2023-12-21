use crate::profile::core::api::ProfileAPI;
use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct GetConfigProfilesResponse {}

#[derive(serde::Serialize)]
pub struct ErrorResponse {}

#[tauri::command]
pub async fn get_profiles(
    _api: tauri::State<'_, Arc<dyn ProfileAPI>>,
) -> Result<GetConfigProfilesResponse, ErrorResponse> {
    todo!()
}
