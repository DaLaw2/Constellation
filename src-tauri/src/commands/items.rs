//! Item Commands
//!
//! Thin adapters for item operations that delegate to ItemService.

use crate::application::dto::{CreateItemDto, ItemDto, TagDto, UpdateItemDto};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn create_item(
    path: String,
    is_directory: bool,
    size: Option<i64>,
    modified_time: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let dto = CreateItemDto {
        path,
        is_directory,
        size,
        modified_time,
    };

    state
        .item_service
        .create(dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_item(id: i64, state: State<'_, AppState>) -> AppResult<ItemDto> {
    state
        .item_service
        .get_by_id(id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Item with id {}", id)))
}

#[tauri::command]
pub async fn get_item_by_path(
    path: String,
    state: State<'_, AppState>,
) -> AppResult<Option<ItemDto>> {
    state
        .item_service
        .get_by_path(&path)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_items_by_paths(
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<ItemDto>> {
    state
        .item_service
        .get_by_paths(paths)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn update_item(
    id: i64,
    path: Option<String>,
    size: Option<i64>,
    modified_time: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let dto = UpdateItemDto {
        path,
        size,
        modified_time,
    };

    state
        .item_service
        .update(id, dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn delete_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    state
        .item_service
        .delete(id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))
}

#[tauri::command]
pub async fn add_tag_to_item(
    item_id: i64,
    tag_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .item_service
        .add_tag(item_id, tag_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn remove_tag_from_item(
    item_id: i64,
    tag_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .item_service
        .remove_tag(item_id, tag_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_tags_for_item(item_id: i64, state: State<'_, AppState>) -> AppResult<Vec<TagDto>> {
    state
        .item_service
        .get_tags(item_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_tags_for_items(
    item_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<HashMap<i64, Vec<TagDto>>> {
    state
        .item_service
        .get_tags_batch(item_ids)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn update_item_tags(
    item_id: i64,
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .item_service
        .update_tags(item_id, tag_ids)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}
