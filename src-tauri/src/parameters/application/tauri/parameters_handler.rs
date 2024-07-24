use std::sync::Arc;

use crate::parameters::core::api::ParameterDataAPI;
use crate::parameters::core::domain::Parameters;
use crate::parameters::core::error::ParameterDataError;

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_parameters(
    api: tauri::State<'_, Arc<dyn ParameterDataAPI>>,
    profile_name: String,
    page_size: u32,
) -> Result<Parameters, ParameterDataError> {
    let result = api.get_parameters(profile_name.as_str(), page_size).await;

    result.map_err(ParameterDataError::from)
}
