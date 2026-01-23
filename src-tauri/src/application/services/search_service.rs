//! Search Application Service
//!
//! Orchestrates search operations across items.

use crate::application::dto::{ItemDto, SearchCriteriaDto};
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::SqliteSearchRepository;
use std::sync::Arc;

/// Service for search operations.
pub struct SearchService {
    search_repo: Arc<SqliteSearchRepository>,
}

impl SearchService {
    pub fn new(search_repo: Arc<SqliteSearchRepository>) -> Self {
        Self { search_repo }
    }

    /// Searches items by tags with AND logic.
    pub async fn search_by_tags_and(&self, tag_ids: Vec<i64>) -> Result<Vec<ItemDto>, DomainError> {
        if tag_ids.is_empty() {
            return Ok(Vec::new());
        }
        self.search_repo.search_by_tags_and(tag_ids).await
    }

    /// Searches items by tags with OR logic.
    pub async fn search_by_tags_or(&self, tag_ids: Vec<i64>) -> Result<Vec<ItemDto>, DomainError> {
        if tag_ids.is_empty() {
            return Ok(Vec::new());
        }
        self.search_repo.search_by_tags_or(tag_ids).await
    }

    /// Searches items by filename.
    pub async fn search_by_filename(&self, query: &str) -> Result<Vec<ItemDto>, DomainError> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(Vec::new());
        }
        self.search_repo.search_by_filename(query).await
    }

    /// Combined search with tags and optional filename filter.
    pub async fn search(&self, criteria: SearchCriteriaDto) -> Result<Vec<ItemDto>, DomainError> {
        let has_tags = !criteria.tag_ids.is_empty();
        let has_filename = criteria
            .filename_query
            .as_ref()
            .map(|q| !q.trim().is_empty())
            .unwrap_or(false);

        if !has_tags && !has_filename {
            return Ok(Vec::new());
        }

        self.search_repo
            .search_combined(criteria.tag_ids, criteria.mode, criteria.filename_query)
            .await
    }
}
