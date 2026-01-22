//! Tag Repository Trait
//!
//! Defines the contract for Tag persistence operations.

use crate::domain::entities::Tag;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;

/// Repository trait for Tag persistence.
#[async_trait]
pub trait TagRepository: Send + Sync {
    /// Saves a new tag and returns its ID.
    async fn save(&self, tag: &mut Tag) -> Result<i64, DomainError>;

    /// Finds a tag by its ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<Tag>, DomainError>;

    /// Finds tags by their IDs.
    async fn find_by_ids(&self, ids: &[i64]) -> Result<Vec<Tag>, DomainError>;

    /// Gets all tags for a specific group.
    async fn find_by_group(&self, group_id: i64) -> Result<Vec<Tag>, DomainError>;

    /// Gets all tags.
    async fn find_all(&self) -> Result<Vec<Tag>, DomainError>;

    /// Updates an existing tag.
    async fn update(&self, tag: &Tag) -> Result<(), DomainError>;

    /// Deletes a tag.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    /// Searches tags by value.
    async fn search(
        &self,
        query: &str,
        group_id: Option<i64>,
        limit: usize,
    ) -> Result<Vec<Tag>, DomainError>;

    /// Gets usage counts for all tags (tag_id -> count).
    async fn get_usage_counts(&self) -> Result<HashMap<i64, i64>, DomainError>;

    /// Gets tags for a specific item.
    async fn find_by_item(&self, item_id: i64) -> Result<Vec<Tag>, DomainError>;
}
