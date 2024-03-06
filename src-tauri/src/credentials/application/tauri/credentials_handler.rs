use std::sync::Arc;

use crate::credentials::core::api::CredentialsDataAPI;
use crate::credentials::core::error::CredentialsError;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn validate_credentials(
    api: tauri::State<'_, Arc<dyn CredentialsDataAPI>>,
    profile_name: String,
) -> Result<bool, CredentialsError> {
    api.validate_credentials(profile_name.as_str())
        .await
        .map_err(CredentialsError::from)
}
