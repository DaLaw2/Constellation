//! Item Repository Trait
//!
//! Defines the contract for Item persistence operations.

use crate::domain::entities::Item;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Repository trait for Item persistence.
#[async_trait]
pub trait ItemRepository: Send + Sync {
    /// Saves a new item and returns its ID.
    async fn save(&self, item: &mut Item) -> Result<i64, DomainError>;

    /// Finds an item by its ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, DomainError>;

    /// Finds an item by its path.
    async fn find_by_path(&self, path: &str) -> Result<Option<Item>, DomainError>;

    /// Updates an existing item.
    async fn update(&self, item: &Item) -> Result<(), DomainError>;

    /// Permanently deletes an item.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    /// Adds a tag to an item.
    async fn add_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError>;

    /// Removes a tag from an item.
    async fn remove_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError>;

    /// Gets all tag IDs for an item.
    async fn get_tag_ids(&self, item_id: i64) -> Result<Vec<i64>, DomainError>;

    /// Replaces all tags for an item atomically.
    async fn replace_tags(&self, item_id: i64, tag_ids: Vec<i64>) -> Result<(), DomainError>;
}
