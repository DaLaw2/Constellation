//! Thumbnail Commands
//!
//! Thin adapters for thumbnail cache operations.

use crate::application::dto::CacheStatsDto;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_cache_stats(state: State<'_, AppState>) -> AppResult<CacheStatsDto> {
    let stats = state
        .thumbnail_service
        .cache_stats()
        .await
        .map_err(|e| AppError::Thumbnail(e.to_string()))?;

    Ok(CacheStatsDto {
        total_size_bytes: stats.total_size_bytes,
        file_count: stats.file_count,
        max_size_bytes: stats.max_size_bytes,
    })
}

#[tauri::command]
pub async fn clear_thumbnail_cache(state: State<'_, AppState>) -> AppResult<CacheStatsDto> {
    let stats = state
        .thumbnail_service
        .clear_cache()
        .await
        .map_err(|e| AppError::Thumbnail(e.to_string()))?;

    Ok(CacheStatsDto {
        total_size_bytes: stats.total_size_bytes,
        file_count: stats.file_count,
        max_size_bytes: stats.max_size_bytes,
    })
}
