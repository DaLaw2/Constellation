//! Settings Repository Trait
//!
//! Defines the contract for settings persistence operations.

use crate::domain::errors::DomainError;
use async_trait::async_trait;

/// Repository trait for settings persistence (key-value store).
#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Gets a setting value by key.
    async fn get(&self, key: &str) -> Result<Option<String>, DomainError>;

    /// Gets all stored settings as key-value pairs.
    async fn get_all(&self) -> Result<Vec<(String, String)>, DomainError>;

    /// Sets a setting value (upsert).
    async fn set(&self, key: &str, value: &str) -> Result<(), DomainError>;

    /// Deletes a setting (resets to default).
    async fn delete(&self, key: &str) -> Result<(), DomainError>;
}
