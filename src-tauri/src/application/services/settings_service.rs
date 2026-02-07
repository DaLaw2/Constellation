//! Settings Application Service
//!
//! Orchestrates settings-related operations with default value fallback.

use crate::domain::entities::SettingsDefaults;
use crate::domain::errors::DomainError;
use crate::domain::repositories::SettingsRepository;
use std::collections::HashMap;
use std::sync::Arc;

/// Service for settings operations.
pub struct SettingsService {
    repo: Arc<dyn SettingsRepository>,
}

impl SettingsService {
    pub fn new(repo: Arc<dyn SettingsRepository>) -> Self {
        Self { repo }
    }

    /// Gets a setting value by key, falling back to the default if not stored.
    pub async fn get(&self, key: &str) -> Result<Option<String>, DomainError> {
        let stored = self.repo.get(key).await?;
        Ok(stored.or_else(|| SettingsDefaults::get(key).map(|s| s.to_string())))
    }

    /// Gets all settings, merging stored values with defaults.
    pub async fn get_all(&self) -> Result<HashMap<String, String>, DomainError> {
        let mut settings = SettingsDefaults::all();
        let stored = self.repo.get_all().await?;
        for (key, value) in stored {
            settings.insert(key, value);
        }
        Ok(settings)
    }

    /// Sets a setting value.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), DomainError> {
        self.repo.set(key, value).await
    }

    /// Resets a setting to its default by removing the stored value.
    pub async fn reset(&self, key: &str) -> Result<(), DomainError> {
        self.repo.delete(key).await
    }
}
