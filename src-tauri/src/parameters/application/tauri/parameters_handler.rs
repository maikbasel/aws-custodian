use std::sync::Arc;

use crate::parameters::core::api::{ParameterDataAPI, SetParameterRequest};
use crate::parameters::core::domain::ParameterSet;
use crate::parameters::core::error::ParameterDataError;

#[derive(serde::Serialize)]
pub struct GetAvailableParametersResponse {
    names: Vec<String>,
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_available_parameters(
    api: tauri::State<'_, Arc<dyn ParameterDataAPI>>,
    profile_name: String,
) -> Result<GetAvailableParametersResponse, ParameterDataError> {
    let result = api.get_available_parameters(profile_name.as_str()).await;

    result
        .map(|names| GetAvailableParametersResponse { names })
        .map_err(ParameterDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn get_parameters(
    api: tauri::State<'_, Arc<dyn ParameterDataAPI>>,
    profile_name: String,
    parameter_names: Vec<String>,
) -> Result<ParameterSet, ParameterDataError> {
    let result = api
        .get_parameters(profile_name.as_str(), parameter_names)
        .await;

    result.map_err(ParameterDataError::from)
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub async fn set_parameter(
    api: tauri::State<'_, Arc<dyn ParameterDataAPI>>,
    profile_name: String,
    request: SetParameterRequest,
) -> Result<(), ParameterDataError> {
    let result = api
        .set_parameter(profile_name.as_str(), request)
        .await;

    result.map_err(ParameterDataError::from)
}