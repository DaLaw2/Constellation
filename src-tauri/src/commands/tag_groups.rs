//! TagGroup Commands
//!
//! Thin adapters for tag group operations that delegate to TagGroupService.

use crate::application::dto::{CreateTagGroupDto, TagGroupDto, UpdateTagGroupDto};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_tag_group(
    name: String,
    color: Option<String>,
    _display_order: Option<i32>, // Ignored - auto-assigned by service
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let dto = CreateTagGroupDto { name, color };

    state
        .tag_group_service
        .create(dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_tag_groups(state: State<'_, AppState>) -> AppResult<Vec<TagGroupDto>> {
    state
        .tag_group_service
        .get_all()
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn update_tag_group(
    id: i64,
    name: Option<String>,
    color: Option<String>,
    _display_order: Option<i32>, // Handled separately via reorder
    state: State<'_, AppState>,
) -> AppResult<()> {
    let dto = UpdateTagGroupDto { name, color };

    state
        .tag_group_service
        .update(id, dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn delete_tag_group(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    state
        .tag_group_service
        .delete(id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))
}

#[derive(serde::Deserialize)]
pub struct TagGroupOrder {
    pub id: i64,
    pub display_order: i32,
}

#[tauri::command]
pub async fn reorder_tag_groups(
    orders: Vec<TagGroupOrder>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let orders: Vec<(i64, i32)> = orders.into_iter().map(|o| (o.id, o.display_order)).collect();

    state
        .tag_group_service
        .reorder(orders)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}
