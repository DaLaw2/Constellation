//! Settings Commands
//!
//! Thin adapters for settings operations that delegate to SettingsService.

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_all_settings(
    state: State<'_, AppState>,
) -> AppResult<HashMap<String, String>> {
    state
        .settings_service
        .get_all()
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}

#[tauri::command]
pub async fn update_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .settings_service
        .set(&key, &value)
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}

#[tauri::command]
pub async fn reset_setting(
    key: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .settings_service
        .reset(&key)
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}
