//! TagGroup Repository Trait
//!
//! Defines the contract for TagGroup persistence operations.

use crate::domain::entities::TagGroup;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Repository trait for TagGroup persistence.
#[async_trait]
pub trait TagGroupRepository: Send + Sync {
    /// Saves a new tag group and returns its ID.
    async fn save(&self, group: &mut TagGroup) -> Result<i64, DomainError>;

    /// Finds a tag group by its ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<TagGroup>, DomainError>;

    /// Gets all tag groups ordered by display order.
    async fn find_all(&self) -> Result<Vec<TagGroup>, DomainError>;

    /// Updates an existing tag group.
    async fn update(&self, group: &TagGroup) -> Result<(), DomainError>;

    /// Deletes a tag group and all its tags.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    /// Updates the display order of multiple groups atomically.
    async fn reorder(&self, orders: Vec<(i64, i32)>) -> Result<(), DomainError>;

    /// Checks if a tag group exists.
    async fn exists(&self, id: i64) -> Result<bool, DomainError>;
}
