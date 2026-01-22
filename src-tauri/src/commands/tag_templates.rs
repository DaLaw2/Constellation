//! TagTemplate Commands
//!
//! Thin adapters for tag template operations that delegate to TagTemplateService.

use crate::application::dto::{CreateTagTemplateDto, TagTemplateDto, UpdateTagTemplateDto};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_tag_template(
    name: String,
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let dto = CreateTagTemplateDto { name, tag_ids };

    state
        .tag_template_service
        .create(dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn get_tag_templates(state: State<'_, AppState>) -> AppResult<Vec<TagTemplateDto>> {
    state
        .tag_template_service
        .get_all()
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn apply_tag_template(
    item_id: i64,
    template_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state
        .tag_template_service
        .apply_to_item(template_id, item_id)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn delete_tag_template(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    state
        .tag_template_service
        .delete(id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))
}

#[tauri::command]
pub async fn update_tag_template(
    id: i64,
    name: Option<String>,
    tag_ids: Option<Vec<i64>>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let dto = UpdateTagTemplateDto { name, tag_ids };

    state
        .tag_template_service
        .update(id, dto)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}
