//! Search Commands
//!
//! Thin adapters for search operations that delegate to SearchService.

use crate::application::dto::{ItemDto, SearchCriteriaDto, SearchMode};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn search_items_by_tags_and(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<ItemDto>> {
    state
        .search_service
        .search_by_tags_and(tag_ids)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn search_items_by_tags_or(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<ItemDto>> {
    state
        .search_service
        .search_by_tags_or(tag_ids)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn search_items_by_filename(
    query: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<ItemDto>> {
    state
        .search_service
        .search_by_filename(&query)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}

#[tauri::command]
pub async fn search_items(
    tag_ids: Vec<i64>,
    mode: SearchMode,
    filename_query: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<ItemDto>> {
    let criteria = SearchCriteriaDto {
        tag_ids,
        mode,
        filename_query,
    };

    state
        .search_service
        .search(criteria)
        .await
        .map_err(|e| AppError::InvalidInput(e.to_string()))
}
