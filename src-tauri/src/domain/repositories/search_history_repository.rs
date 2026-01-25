use crate::domain::entities::{SearchCriteria, SearchHistory};
use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Repository for managing Search History persistence.
#[async_trait]
pub trait SearchHistoryRepository: Send + Sync {
    /// Saves a search to history.
    /// If an identical criteria exists, updates its `last_used_at`.
    /// If not, inserts a new record.
    async fn save(&self, criteria: SearchCriteria) -> Result<(), DomainError>;

    /// Retrieves the N most recent searches.
    async fn get_recent(&self, limit: usize) -> Result<Vec<SearchHistory>, DomainError>;

    /// Deletes a specific history entry.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    /// Clears all history.
    async fn clear_all(&self) -> Result<(), DomainError>;
}
