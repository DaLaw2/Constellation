//! Tag Commands
//!
//! Thin adapters for tag operations that delegate to TagService.

use crate::application::dto::{CreateTagDto, TagDto, UpdateTagDto};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn create_tag(
    group_id: i64,
    value: String,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let dto = CreateTagDto { group_id, value };

    state
        .tag_service
        .create(dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_tags_by_group(
    group_id: i64,
    state: State<'_, AppState>,
) -> AppResult<Vec<TagDto>> {
    state
        .tag_service
        .get_by_group(group_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_all_tags(state: State<'_, AppState>) -> AppResult<Vec<TagDto>> {
    state
        .tag_service
        .get_all()
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn update_tag(
    id: i64,
    value: Option<String>,
    group_id: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let dto = UpdateTagDto { value, group_id };

    state
        .tag_service
        .update(id, dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn merge_tags(
    source_id: i64,
    target_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .tag_service
        .merge(source_id, target_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn delete_tag(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    state
        .tag_service
        .delete(id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))
}

#[tauri::command]
pub async fn get_tag_usage_counts(state: State<'_, AppState>) -> AppResult<HashMap<i64, i64>> {
    state
        .tag_service
        .get_usage_counts()
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn search_tags(
    query: String,
    group_id: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<TagDto>> {
    state
        .tag_service
        .search(&query, group_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}
