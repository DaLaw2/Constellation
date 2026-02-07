//! Application State
//!
//! Holds all services and configuration for the application.

use crate::application::services::{
    ItemService, SearchService, SettingsService, TagGroupService, TagService, TagTemplateService,
    ThumbnailService, UsnRefreshService,
};
use crate::domain::repositories::{
    ItemRepository, SettingsRepository, TagGroupRepository, TagRepository, TagTemplateRepository,
};
use crate::infrastructure::persistence::{
    SqliteItemRepository, SqliteSearchHistoryRepository, SqliteSearchRepository,
    SqliteSettingsRepository, SqliteTagGroupRepository, SqliteTagRepository,
    SqliteTagTemplateRepository,
};
use deadpool_sqlite::Pool;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub db_path: String,
}

/// Application state containing all services.
#[allow(dead_code)]
pub struct AppState {
    pub config: AppConfig,
    pub app_data_dir: std::path::PathBuf,

    // Application Services
    pub item_service: Arc<ItemService>,
    pub tag_service: Arc<TagService>,
    pub tag_group_service: Arc<TagGroupService>,
    pub tag_template_service: Arc<TagTemplateService>,
    pub search_service: Arc<SearchService>,
    pub settings_service: Arc<SettingsService>,
    pub thumbnail_service: Arc<ThumbnailService>,
    pub usn_refresh_service: Arc<UsnRefreshService>,
}

impl AppState {
    pub fn new(pool: Pool, config: AppConfig, app_data_dir: std::path::PathBuf) -> Self {
        let pool = Arc::new(pool);

        // Create repositories
        let item_repo: Arc<dyn ItemRepository> = Arc::new(SqliteItemRepository::new(pool.clone()));
        let tag_repo: Arc<dyn TagRepository> = Arc::new(SqliteTagRepository::new(pool.clone()));
        let tag_group_repo: Arc<dyn TagGroupRepository> =
            Arc::new(SqliteTagGroupRepository::new(pool.clone()));
        let tag_template_repo: Arc<dyn TagTemplateRepository> =
            Arc::new(SqliteTagTemplateRepository::new(pool.clone()));
        let search_repo = Arc::new(SqliteSearchRepository::new(pool.clone()));
        let search_history_repo = Arc::new(SqliteSearchHistoryRepository::new(pool.clone()));
        let settings_repo: Arc<dyn SettingsRepository> =
            Arc::new(SqliteSettingsRepository::new(pool.clone()));

        // Create application services
        let item_service = Arc::new(ItemService::new(item_repo.clone(), tag_repo.clone()));
        let tag_service = Arc::new(TagService::new(tag_repo.clone(), tag_group_repo.clone()));
        let tag_group_service = Arc::new(TagGroupService::new(tag_group_repo.clone()));
        let tag_template_service = Arc::new(TagTemplateService::new(
            tag_template_repo,
            item_repo.clone(),
        ));
        let search_service = Arc::new(SearchService::new(search_repo, search_history_repo));
        let settings_service = Arc::new(SettingsService::new(settings_repo));
        let usn_refresh_service = Arc::new(UsnRefreshService::new(
            pool.clone(),
            item_repo.clone(),
            settings_service.clone(),
        ));
        let thumbnail_service = Arc::new(ThumbnailService::new(
            app_data_dir.clone(),
            settings_service.clone(),
        ));

        Self {
            config,
            app_data_dir,
            item_service,
            tag_service,
            tag_group_service,
            tag_template_service,
            search_service,
            settings_service,
            thumbnail_service,
            usn_refresh_service,
        }
    }
}
