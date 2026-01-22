//! TagTemplate Repository Trait
//!
//! Defines the contract for TagTemplate persistence operations.

use crate::domain::entities::TagTemplate;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Repository trait for TagTemplate persistence.
#[async_trait]
pub trait TagTemplateRepository: Send + Sync {
    /// Saves a new template and returns its ID.
    async fn save(&self, template: &mut TagTemplate) -> Result<i64, DomainError>;

    /// Finds a template by its ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<TagTemplate>, DomainError>;

    /// Gets all templates with their tag IDs.
    async fn find_all(&self) -> Result<Vec<TagTemplate>, DomainError>;

    /// Updates an existing template.
    async fn update(&self, template: &TagTemplate) -> Result<(), DomainError>;

    /// Deletes a template.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;
}
