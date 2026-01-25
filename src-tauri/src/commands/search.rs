//! Search Commands
//!
//! Thin adapters for search operations that delegate to SearchService.

use crate::application::dto::{ItemDto, SearchCriteriaDto, SearchHistoryDto, SearchMode};
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

#[tauri::command]
pub async fn get_recent_search_history(
    limit: usize,
    state: State<'_, AppState>,
) -> AppResult<Vec<SearchHistoryDto>> {
    state
        .search_service
        .get_recent_history(limit)
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}

#[tauri::command]
pub async fn delete_search_history(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    state
        .search_service
        .delete_history(id)
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}

#[tauri::command]
pub async fn clear_search_history(state: State<'_, AppState>) -> AppResult<()> {
    state
        .search_service
        .clear_history()
        .await
        .map_err(|e| AppError::Domain(e.to_string()))
}
