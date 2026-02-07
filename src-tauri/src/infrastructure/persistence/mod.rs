//! Persistence Infrastructure
//!
//! SQLite implementations of repository interfaces.

mod cql_executor;
mod schema;
mod sqlite_item_repository;
mod sqlite_search_history_repository;
mod sqlite_search_repository;
mod sqlite_settings_repository;
mod sqlite_tag_group_repository;
mod sqlite_tag_repository;
mod sqlite_tag_template_repository;

pub use schema::init_database;
pub use sqlite_item_repository::SqliteItemRepository;
pub use sqlite_search_history_repository::SqliteSearchHistoryRepository;
pub use sqlite_search_repository::SqliteSearchRepository;
pub use sqlite_settings_repository::SqliteSettingsRepository;
pub use sqlite_tag_group_repository::SqliteTagGroupRepository;
pub use sqlite_tag_repository::SqliteTagRepository;
pub use sqlite_tag_template_repository::SqliteTagTemplateRepository;
