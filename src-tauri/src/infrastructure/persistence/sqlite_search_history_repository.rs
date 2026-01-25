//! SQLite Search History Repository
//!
//! Implementation of the SearchHistoryRepository trait using SQLite.

use crate::application::dto::SearchMode;
use crate::domain::entities::{SearchCriteria, SearchHistory};
use crate::domain::errors::DomainError;
use crate::domain::repositories::SearchHistoryRepository;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::{params, Connection};
use std::sync::Arc;

/// SQLite repository for search history operations.
pub struct SqliteSearchHistoryRepository {
    pool: Arc<Pool>,
}

impl SqliteSearchHistoryRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SearchHistoryRepository for SqliteSearchHistoryRepository {
    async fn save(&self, criteria: SearchCriteria) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let criteria = Arc::new(criteria);

        conn.interact(move |conn: &mut Connection| {
            let tx = conn.transaction()?;

            // 1. Try to find existing history specifically matching these criteria.
            // Matching: same search_mode AND same text_query AND same set of tags.
            // This is complex in SQL, so we can do a simplified check:
            // Find candidates by (text_query, search_mode), then verify tags in application or complex JOIN.
            // 
            // Better approach (SQL):
            // SELECT id FROM search_histories WHERE text_query = ? AND search_mode = ?
            
            let mode_str = match criteria.mode {
                SearchMode::And => "AND",
                SearchMode::Or => "OR",
            };

            let candidates: Vec<i64> = {
                let mut stmt = tx.prepare(
                    "SELECT id FROM search_histories 
                     WHERE (text_query IS ?1 OR (text_query IS NULL AND ?1 IS NULL)) 
                     AND search_mode = ?2"
                )?;
                let rows = stmt.query_map(params![criteria.text_query, mode_str], |row| row.get(0))?;
                rows.collect::<Result<Vec<i64>, _>>()?
            };

            let mut match_id: Option<i64> = None;

            for id in candidates {
                // Check tags for this candidate
                let mut stmt = tx.prepare(
                    "SELECT tag_id FROM search_history_tags WHERE search_history_id = ? ORDER BY tag_id ASC"
                )?;
                let tags: Vec<i64> = stmt.query_map([id], |row| row.get(0))?
                    .collect::<Result<Vec<i64>, _>>()?;

                if tags == criteria.tag_ids {
                    match_id = Some(id);
                    break;
                }
            }

            if let Some(id) = match_id {
                // Update existing
                tx.execute(
                    "UPDATE search_histories SET last_used_at = unixepoch() WHERE id = ?",
                    [id],
                )?;
            } else {
                // Insert new header
                tx.execute(
                    "INSERT INTO search_histories (text_query, search_mode, last_used_at) 
                     VALUES (?1, ?2, unixepoch())",
                    params![criteria.text_query, mode_str],
                )?;
                let new_id = tx.last_insert_rowid();

                // Insert tags
                for tag_id in &criteria.tag_ids {
                    tx.execute(
                        "INSERT INTO search_history_tags (search_history_id, tag_id) VALUES (?, ?)",
                        [new_id, *tag_id],
                    )?;
                }
            }

            tx.commit()?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn get_recent(&self, limit: usize) -> Result<Vec<SearchHistory>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, text_query, search_mode, last_used_at 
                 FROM search_histories 
                 ORDER BY last_used_at DESC 
                 LIMIT ?"
            )?;

            let histories_iter = stmt.query_map([limit], |row| {
                let id: i64 = row.get(0)?;
                let text_query: Option<String> = row.get(1)?;
                let mode_str: String = row.get(2)?;
                let last_used_at: i64 = row.get(3)?;

                let mode = match mode_str.as_str() {
                    "AND" => SearchMode::And,
                    "OR" => SearchMode::Or,
                    _ => SearchMode::And, // Fallback
                };

                Ok((id, text_query, mode, last_used_at))
            })?;

            let mut result = Vec::new();

            // We need to fetch tags for each history. 
            // N+1 query is acceptable here for ensuring correct assembly and normally 'limit' is small (e.g. 10).
            for row in histories_iter {
                let (id, text_query, mode, last_used_at) = row?;
                
                let mut tag_stmt = conn.prepare(
                    "SELECT tag_id FROM search_history_tags WHERE search_history_id = ? ORDER BY tag_id ASC"
                )?;
                let tag_ids: Vec<i64> = tag_stmt.query_map([id], |r| r.get(0))?
                    .collect::<Result<Vec<i64>, _>>()?;

                // No need to sort again if DB query ordered them, but SearchCriteria::new creates consistent object
                let criteria = SearchCriteria::new(text_query, tag_ids, mode);

                result.push(SearchHistory {
                    id,
                    criteria,
                    last_used_at,
                });
            }

            Ok::<Vec<SearchHistory>, rusqlite::Error>(result)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("DELETE FROM search_histories WHERE id = ?", [id])?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn clear_all(&self) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("DELETE FROM search_histories", [])?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }
}

fn map_pool_error(e: deadpool_sqlite::PoolError) -> DomainError {
    DomainError::DatabaseError(format!("Database pool error: {}", e))
}

fn map_interact_error(e: deadpool_sqlite::InteractError) -> DomainError {
    DomainError::DatabaseError(format!("Database interaction error: {}", e))
}

fn map_db_error(e: rusqlite::Error) -> DomainError {
    DomainError::DatabaseError(format!("Database error: {}", e))
}
