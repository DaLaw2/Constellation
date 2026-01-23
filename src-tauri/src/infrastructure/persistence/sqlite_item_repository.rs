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

        Ok(Item::reconstitute(
            row.get(0)?,
            path,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
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

        let id = conn
            .interact(move |conn: &mut Connection| {
                conn.execute(
                    "INSERT INTO items (path, is_directory, size, modified_time) VALUES (?1, ?2, ?3, ?4)",
                    (&path, &is_directory, &size, &modified_time),
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
                    "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                     FROM items WHERE id = ?1 AND is_deleted = 0",
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
                    "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                     FROM items WHERE path = ?1 AND is_deleted = 0",
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

    async fn update(&self, item: &Item) -> Result<(), DomainError> {
        let id = item.id().ok_or_else(|| {
            DomainError::ValidationError("Cannot update item without ID".to_string())
        })?;

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let path = item.path().to_string();
        let size = item.size();
        let modified_time = item.modified_time();

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
                    "UPDATE items SET path = ?1, size = ?2, modified_time = ?3, updated_at = unixepoch() WHERE id = ?4",
                    (&path, &size, &modified_time, id),
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

    async fn soft_delete(&self, id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let item_deleted: Option<bool> = conn
                    .query_row("SELECT is_deleted FROM items WHERE id = ?1", [id], |row| {
                        row.get(0)
                    })
                    .optional()?;

                match item_deleted {
                    None => return Err(rusqlite::Error::QueryReturnedNoRows),
                    Some(true) => return Err(rusqlite::Error::InvalidQuery),
                    Some(false) => {}
                }

                conn.execute(
                    "UPDATE items SET is_deleted = 1, deleted_at = unixepoch(), updated_at = unixepoch() WHERE id = ?1",
                    [id],
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
        .map_err(|e| {
            if matches!(e, rusqlite::Error::InvalidQuery) {
                DomainError::ItemAlreadyDeleted
            } else {
                map_db_error(e)
            }
        })
    }

    async fn restore(&self, id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let restored = conn
            .interact(move |conn: &mut Connection| {
                let rows = conn.execute(
                    "UPDATE items SET is_deleted = 0, deleted_at = NULL, updated_at = unixepoch() WHERE id = ?1 AND is_deleted = 1",
                    [id],
                )?;
                Ok::<usize, rusqlite::Error>(rows)
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        if restored == 0 {
            return Err(DomainError::ItemNotFound(id.to_string()));
        }

        Ok(())
    }

    async fn find_deleted(&self) -> Result<Vec<Item>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                 FROM items WHERE is_deleted = 1 ORDER BY deleted_at DESC",
            )?;

            let items = stmt
                .query_map([], Self::map_row_to_item)?
                .collect::<Result<Vec<Item>, _>>()?;

            Ok::<Vec<Item>, rusqlite::Error>(items)
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
                    "SELECT COUNT(*) FROM items WHERE id = ?1 AND is_deleted = 0",
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
