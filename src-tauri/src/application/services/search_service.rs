//! Search Application Service
//!
//! Orchestrates search operations across items.

use crate::application::dto::{ItemDto, SearchCriteriaDto, SearchHistoryDto};
use crate::domain::entities::SearchCriteria;
use crate::domain::errors::DomainError;
use crate::domain::repositories::SearchHistoryRepository;
use crate::infrastructure::persistence::{SqliteSearchHistoryRepository, SqliteSearchRepository};
use std::sync::Arc;

/// Service for search operations.
pub struct SearchService {
    search_repo: Arc<SqliteSearchRepository>,
    history_repo: Arc<SqliteSearchHistoryRepository>,
}

impl SearchService {
    pub fn new(
        search_repo: Arc<SqliteSearchRepository>,
        history_repo: Arc<SqliteSearchHistoryRepository>,
    ) -> Self {
        Self {
            search_repo,
            history_repo,
        }
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

        // Result of the search
        let results = self
            .search_repo
            .search_combined(
                criteria.tag_ids.clone(),
                criteria.mode,
                criteria.filename_query.clone(),
            )
            .await?;

        // Save to history (fire and forget approx, but we await it here for simplicity)
        // Only save if it's a valid search (which we checked above)
        let history_criteria =
            SearchCriteria::new(criteria.filename_query, criteria.tag_ids, criteria.mode);

        if let Err(e) = self.history_repo.save(history_criteria).await {
            // Log error but don't fail the search?
            // For now, let's treat it as non-fatal but maybe log to stderr
            eprintln!("Failed to save search history: {}", e);
        }

        Ok(results)
    }

    /// Searches items using a CQL query string.
    pub async fn search_cql(&self, query: &str) -> Result<Vec<ItemDto>, DomainError> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(Vec::new());
        }
        self.search_repo.search_cql(query).await
    }

    /// Retrieves recent search history.
    pub async fn get_recent_history(
        &self,
        limit: usize,
    ) -> Result<Vec<SearchHistoryDto>, DomainError> {
        let histories = self.history_repo.get_recent(limit).await?;

        let dtos = histories
            .into_iter()
            .map(|h| SearchHistoryDto {
                id: h.id,
                criteria: SearchCriteriaDto {
                    tag_ids: h.criteria.tag_ids,
                    mode: h.criteria.mode,
                    filename_query: h.criteria.text_query,
                },
                last_used_at: h.last_used_at,
            })
            .collect();

        Ok(dtos)
    }

    /// Deletes a specific history entry.
    pub async fn delete_history(&self, id: i64) -> Result<(), DomainError> {
        self.history_repo.delete(id).await
    }

    /// Clears all search history.
    pub async fn clear_history(&self) -> Result<(), DomainError> {
        self.history_repo.clear_all().await
    }
}
