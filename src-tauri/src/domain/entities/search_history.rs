use crate::application::dto::SearchMode;
use serde::{Deserialize, Serialize};

/// Represents a historical search entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistory {
    /// Unique identifier
    pub id: i64,
    /// The criteria used for the search
    pub criteria: SearchCriteria,
    /// When this search was last performed (unix timestamp)
    pub last_used_at: i64,
}

/// Helper struct for defining search criteria equality.
/// This matches the Ubiquitous Language definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchCriteria {
    /// Optional text query
    pub text_query: Option<String>,
    /// List of tag IDs included in the search.
    /// MUST be sorted for consistent equality comparison.
    pub tag_ids: Vec<i64>,
    /// Logical operator for tags
    pub mode: SearchMode,
}

impl SearchCriteria {
    /// Creates a new SearchCriteria, ensuring tag_ids are sorted.
    pub fn new(text_query: Option<String>, mut tag_ids: Vec<i64>, mode: SearchMode) -> Self {
        tag_ids.sort_unstable(); // Ensure sorted for equality check

        // Normalize empty string to None
        let text_query = text_query.filter(|s| !s.trim().is_empty());

        Self {
            text_query,
            tag_ids,
            mode,
        }
    }
}
