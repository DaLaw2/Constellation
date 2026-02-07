//! SQLite Search Repository
//!
//! Specialized repository for search operations.

use crate::application::dto::{ItemDto, SearchMode};
use crate::domain::errors::DomainError;
use crate::domain::search::parse_cql;
use super::cql_executor::expr_to_sql;
use deadpool_sqlite::Pool;
use rusqlite::Connection;
use std::sync::Arc;

/// SQLite repository for search operations.
pub struct SqliteSearchRepository {
    pool: Arc<Pool>,
}

impl SqliteSearchRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    fn map_row_to_item_dto(row: &rusqlite::Row) -> rusqlite::Result<ItemDto> {
        Ok(ItemDto {
            id: row.get(0)?,
            path: row.get(1)?,
            is_directory: row.get(2)?,
            size: row.get(3)?,
            modified_time: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }

    /// Searches items by tags with AND logic (must have ALL specified tags).
    pub async fn search_by_tags_and(&self, tag_ids: Vec<i64>) -> Result<Vec<ItemDto>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let tag_count = tag_ids.len() as i64;

        conn.interact(move |conn: &mut Connection| {
            let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
            let placeholders_str = placeholders.join(", ");

            let sql = format!(
                "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                        i.created_at, i.updated_at
                 FROM items i
                 INNER JOIN item_tags it ON i.id = it.item_id
                 WHERE it.tag_id IN ({})
                 GROUP BY i.id
                 HAVING COUNT(DISTINCT it.tag_id) = ?
                 ORDER BY i.path ASC",
                placeholders_str
            );

            let mut stmt = conn.prepare(&sql)?;

            let mut params: Vec<Box<dyn rusqlite::ToSql>> = tag_ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::ToSql>)
                .collect();
            params.push(Box::new(tag_count));

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let items = stmt
                .query_map(params_refs.as_slice(), Self::map_row_to_item_dto)?
                .collect::<Result<Vec<ItemDto>, _>>()?;

            Ok::<Vec<ItemDto>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    /// Searches items by tags with OR logic (must have ANY of the specified tags).
    pub async fn search_by_tags_or(&self, tag_ids: Vec<i64>) -> Result<Vec<ItemDto>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
            let placeholders_str = placeholders.join(", ");

            let sql = format!(
                "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                        i.created_at, i.updated_at
                 FROM items i
                 INNER JOIN item_tags it ON i.id = it.item_id
                 WHERE it.tag_id IN ({})
                 ORDER BY i.path ASC",
                placeholders_str
            );

            let mut stmt = conn.prepare(&sql)?;

            let params: Vec<Box<dyn rusqlite::ToSql>> = tag_ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::ToSql>)
                .collect();

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let items = stmt
                .query_map(params_refs.as_slice(), Self::map_row_to_item_dto)?
                .collect::<Result<Vec<ItemDto>, _>>()?;

            Ok::<Vec<ItemDto>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    /// Searches items by filename (LIKE query on path).
    pub async fn search_by_filename(&self, query: &str) -> Result<Vec<ItemDto>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let pattern = format!("%{}%", query);

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, path, is_directory, size, modified_time,
                        created_at, updated_at
                 FROM items
                 WHERE path LIKE ?1
                 ORDER BY path ASC",
            )?;

            let items = stmt
                .query_map([&pattern], Self::map_row_to_item_dto)?
                .collect::<Result<Vec<ItemDto>, _>>()?;

            Ok::<Vec<ItemDto>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    /// Combined search with tags and optional filename filter.
    pub async fn search_combined(
        &self,
        tag_ids: Vec<i64>,
        mode: SearchMode,
        filename_query: Option<String>,
    ) -> Result<Vec<ItemDto>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let filename_pattern = filename_query
            .as_ref()
            .filter(|q| !q.trim().is_empty())
            .map(|q| format!("%{}%", q.trim()));

        let has_tags = !tag_ids.is_empty();
        let has_filename = filename_pattern.is_some();
        let tag_count = tag_ids.len() as i64;

        conn.interact(move |conn: &mut Connection| {
            let sql = if has_tags && has_filename {
                let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
                let placeholders_str = placeholders.join(", ");

                match mode {
                    SearchMode::And => format!(
                        "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.path LIKE ?
                         GROUP BY i.id
                         HAVING COUNT(DISTINCT it.tag_id) = ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                    SearchMode::Or => format!(
                        "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.path LIKE ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                }
            } else if has_tags {
                let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
                let placeholders_str = placeholders.join(", ");

                match mode {
                    SearchMode::And => format!(
                        "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({})
                         GROUP BY i.id
                         HAVING COUNT(DISTINCT it.tag_id) = ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                    SearchMode::Or => format!(
                        "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({})
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                }
            } else {
                "SELECT id, path, is_directory, size, modified_time,
                        created_at, updated_at
                 FROM items
                 WHERE path LIKE ?
                 ORDER BY path ASC"
                    .to_string()
            };

            let mut stmt = conn.prepare(&sql)?;

            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

            if has_tags {
                for id in &tag_ids {
                    params.push(Box::new(*id));
                }
            }

            if let Some(ref pattern) = filename_pattern {
                params.push(Box::new(pattern.clone()));
            }

            if has_tags && matches!(mode, SearchMode::And) {
                params.push(Box::new(tag_count));
            }

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let items = stmt
                .query_map(params_refs.as_slice(), Self::map_row_to_item_dto)?
                .collect::<Result<Vec<ItemDto>, _>>()?;

            Ok::<Vec<ItemDto>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    /// Searches items using a CQL query string.
    pub async fn search_cql(&self, query: &str) -> Result<Vec<ItemDto>, DomainError> {
        let expr = parse_cql(query).map_err(|e| DomainError::ValidationError(e.to_string()))?;
        let fragment = expr_to_sql(&expr);

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let sql = format!(
                "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time, \
                        i.created_at, i.updated_at \
                 FROM items i \
                 WHERE i.is_deleted = 0 AND ({}) \
                 ORDER BY i.path ASC",
                fragment.sql
            );

            let mut stmt = conn.prepare(&sql)?;

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                fragment.params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

            let items = stmt
                .query_map(params_refs.as_slice(), Self::map_row_to_item_dto)?
                .collect::<Result<Vec<ItemDto>, _>>()?;

            Ok::<Vec<ItemDto>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }
}

fn map_pool_error(e: deadpool_sqlite::PoolError) -> DomainError {
    DomainError::ValidationError(format!("Database pool error: {}", e))
}

fn map_interact_error(e: deadpool_sqlite::InteractError) -> DomainError {
    DomainError::ValidationError(format!("Database interaction error: {}", e))
}

fn map_db_error(e: rusqlite::Error) -> DomainError {
    DomainError::ValidationError(format!("Database error: {}", e))
}
