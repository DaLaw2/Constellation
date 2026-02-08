//! SQLite Item Repository
//!
//! Implementation of ItemRepository for SQLite.

use crate::domain::entities::Item;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ItemRepository;
use crate::domain::value_objects::FilePath;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::{Connection, OptionalExtension};
use std::sync::Arc;

/// SQLite implementation of ItemRepository.
pub struct SqliteItemRepository {
    pool: Arc<Pool>,
}

impl SqliteItemRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    fn map_row_to_item(row: &rusqlite::Row) -> rusqlite::Result<Item> {
        let path_str: String = row.get(1)?;
        // Use safe fallback for corrupted database data
        let path = FilePath::new(path_str).unwrap_or_else(|_| FilePath::invalid());

        // file_reference_number is stored as nullable i64 in SQLite, convert to u64 (0 = none)
        let frn: Option<i64> = row.get(5)?;
        let frn = frn.map(|v| v as u64).unwrap_or(0);

        Ok(Item::reconstitute(
            row.get(0)?,
            path,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            frn,
            row.get(6)?,
            row.get(7)?,
        ))
    }
}

#[async_trait]
impl ItemRepository for SqliteItemRepository {
    async fn save(&self, item: &mut Item) -> Result<i64, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let path = item.path().to_string();
        let is_directory = item.is_directory();
        let size = item.size();
        let modified_time = item.modified_time();
        let frn = item.file_reference_number() as i64;

        let id = conn
            .interact(move |conn: &mut Connection| {
                conn.execute(
                    "INSERT INTO items (path, is_directory, size, modified_time, file_reference_number) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (&path, &is_directory, &size, &modified_time, frn),
                )?;
                Ok::<i64, rusqlite::Error>(conn.last_insert_rowid())
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        item.set_id(id);
        Ok(id)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Item>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let result = conn
                .query_row(
                    "SELECT id, path, is_directory, size, modified_time, file_reference_number, created_at, updated_at
                     FROM items WHERE id = ?1",
                    [id],
                    Self::map_row_to_item,
                )
                .optional()?;
            Ok::<Option<Item>, rusqlite::Error>(result)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_path(&self, path: &str) -> Result<Option<Item>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let path = path.to_string();

        conn.interact(move |conn: &mut Connection| {
            let result = conn
                .query_row(
                    "SELECT id, path, is_directory, size, modified_time, file_reference_number, created_at, updated_at
                     FROM items WHERE path = ?1",
                    [&path],
                    Self::map_row_to_item,
                )
                .optional()?;
            Ok::<Option<Item>, rusqlite::Error>(result)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_paths(&self, paths: &[String]) -> Result<Vec<Item>, DomainError> {
        if paths.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let paths = paths.to_vec();

        conn.interact(move |conn: &mut Connection| {
            let mut all_items = Vec::new();

            // SQLite has a limit of ~999 bound parameters, chunk to stay safe
            const CHUNK_SIZE: usize = 500;
            for chunk in paths.chunks(CHUNK_SIZE) {
                let placeholders: Vec<&str> = chunk.iter().map(|_| "?").collect();
                let sql = format!(
                    "SELECT id, path, is_directory, size, modified_time, file_reference_number, created_at, updated_at
                     FROM items WHERE path IN ({})",
                    placeholders.join(", ")
                );

                let mut stmt = conn.prepare(&sql)?;
                let params: Vec<&dyn rusqlite::ToSql> =
                    chunk.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

                let items = stmt
                    .query_map(params.as_slice(), Self::map_row_to_item)?
                    .collect::<Result<Vec<_>, _>>()?;

                all_items.extend(items);
            }

            Ok::<Vec<Item>, rusqlite::Error>(all_items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn update(&self, item: &Item) -> Result<(), DomainError> {
        let id = item.id().ok_or_else(|| {
            DomainError::ValidationError("Cannot update item without ID".to_string())
        })?;

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let path = item.path().to_string();
        let size = item.size();
        let modified_time = item.modified_time();
        let frn = item.file_reference_number() as i64;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let exists: bool = conn.query_row(
                    "SELECT COUNT(*) FROM items WHERE id = ?1",
                    [id],
                    |row| row.get::<_, i64>(0).map(|count| count > 0),
                )?;

                if !exists {
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }

                conn.execute(
                    "UPDATE items SET path = ?1, size = ?2, modified_time = ?3, file_reference_number = ?4, updated_at = unixepoch() WHERE id = ?5",
                    (&path, &size, &modified_time, frn, id),
                )?;

                Ok::<(), rusqlite::Error>(())
            })();

            match result {
                Ok(_) => {
                    conn.execute("COMMIT", [])?;
                    Ok(())
                }
                Err(e) => {
                    conn.execute("ROLLBACK", [])?;
                    Err(e)
                }
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let deleted = conn
            .interact(move |conn: &mut Connection| {
                let rows = conn.execute("DELETE FROM items WHERE id = ?1", [id])?;
                Ok::<usize, rusqlite::Error>(rows)
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        if deleted == 0 {
            return Err(DomainError::ItemNotFound(id.to_string()));
        }

        Ok(())
    }

    async fn add_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute(
                "INSERT OR IGNORE INTO item_tags (item_id, tag_id) VALUES (?1, ?2)",
                (item_id, tag_id),
            )?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn remove_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute(
                "DELETE FROM item_tags WHERE item_id = ?1 AND tag_id = ?2",
                (item_id, tag_id),
            )?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn get_tag_ids(&self, item_id: i64) -> Result<Vec<i64>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare("SELECT tag_id FROM item_tags WHERE item_id = ?1")?;
            let ids = stmt
                .query_map([item_id], |row| row.get(0))?
                .collect::<Result<Vec<i64>, _>>()?;
            Ok::<Vec<i64>, rusqlite::Error>(ids)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn replace_tags(&self, item_id: i64, tag_ids: Vec<i64>) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let exists: bool = conn.query_row(
                    "SELECT COUNT(*) FROM items WHERE id = ?1",
                    [item_id],
                    |row| row.get::<_, i64>(0).map(|count| count > 0),
                )?;

                if !exists {
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }

                conn.execute("DELETE FROM item_tags WHERE item_id = ?1", [item_id])?;

                for tag_id in tag_ids {
                    conn.execute(
                        "INSERT INTO item_tags (item_id, tag_id) VALUES (?1, ?2)",
                        (item_id, tag_id),
                    )?;
                }

                Ok::<(), rusqlite::Error>(())
            })();

            match result {
                Ok(_) => {
                    conn.execute("COMMIT", [])?;
                    Ok(())
                }
                Err(e) => {
                    conn.execute("ROLLBACK", [])?;
                    Err(e)
                }
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_active_by_path_prefix(&self, prefix: &str) -> Result<Vec<Item>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let pattern = format!("{}%", prefix);

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, path, is_directory, size, modified_time, file_reference_number,
                        created_at, updated_at
                 FROM items WHERE path LIKE ?1 AND is_deleted = 0",
            )?;
            let items = stmt
                .query_map([&pattern], Self::map_row_to_item)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok::<Vec<Item>, rusqlite::Error>(items)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }
}

// Error mapping helpers
fn map_pool_error(e: deadpool_sqlite::PoolError) -> DomainError {
    DomainError::ValidationError(format!("Database pool error: {}", e))
}

fn map_interact_error(e: deadpool_sqlite::InteractError) -> DomainError {
    DomainError::ValidationError(format!("Database interaction error: {}", e))
}

fn map_db_error(e: rusqlite::Error) -> DomainError {
    match e {
        rusqlite::Error::QueryReturnedNoRows => {
            DomainError::ItemNotFound("Item not found".to_string())
        }
        _ => DomainError::ValidationError(format!("Database error: {}", e)),
    }
}
