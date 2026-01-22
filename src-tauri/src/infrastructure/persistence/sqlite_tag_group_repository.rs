//! SQLite TagGroup Repository
//!
//! Implementation of TagGroupRepository for SQLite.

use crate::domain::entities::TagGroup;
use crate::domain::errors::DomainError;
use crate::domain::repositories::TagGroupRepository;
use crate::domain::value_objects::Color;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::Connection;
use std::sync::Arc;

/// SQLite implementation of TagGroupRepository.
pub struct SqliteTagGroupRepository {
    pool: Arc<Pool>,
}

impl SqliteTagGroupRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    fn map_row_to_group(row: &rusqlite::Row) -> rusqlite::Result<TagGroup> {
        let color_str: Option<String> = row.get(2)?;
        let color = color_str.and_then(|c| Color::new(c).ok());

        Ok(TagGroup::reconstitute(
            row.get(0)?,
            row.get(1)?,
            color,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
        ))
    }
}

#[async_trait]
impl TagGroupRepository for SqliteTagGroupRepository {
    async fn save(&self, group: &mut TagGroup) -> Result<i64, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let name = group.name().to_string();
        let color = group.color().map(|c| c.to_string());
        let display_order = group.display_order();

        let id = conn
            .interact(move |conn: &mut Connection| {
                conn.execute(
                    "INSERT INTO tag_groups (name, color, display_order) VALUES (?1, ?2, ?3)",
                    (&name, &color, display_order),
                )?;
                Ok::<i64, rusqlite::Error>(conn.last_insert_rowid())
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        group.set_id(id);
        Ok(id)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<TagGroup>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let result = conn
                .query_row(
                    "SELECT id, name, color, display_order, created_at, updated_at
                     FROM tag_groups WHERE id = ?1",
                    [id],
                    Self::map_row_to_group,
                )
                .optional();
            match result {
                Ok(group) => Ok(group),
                Err(e) => Err(e),
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_all(&self) -> Result<Vec<TagGroup>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, name, color, display_order, created_at, updated_at
                 FROM tag_groups ORDER BY display_order ASC",
            )?;

            let groups = stmt
                .query_map([], Self::map_row_to_group)?
                .collect::<Result<Vec<TagGroup>, _>>()?;

            Ok::<Vec<TagGroup>, rusqlite::Error>(groups)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn update(&self, group: &TagGroup) -> Result<(), DomainError> {
        let id = group.id().ok_or_else(|| {
            DomainError::ValidationError("Cannot update group without ID".to_string())
        })?;

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let name = group.name().to_string();
        let color = group.color().map(|c| c.to_string());
        let display_order = group.display_order();

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let exists: bool = conn.query_row(
                    "SELECT COUNT(*) FROM tag_groups WHERE id = ?1",
                    [id],
                    |row| row.get::<_, i64>(0).map(|count| count > 0),
                )?;

                if !exists {
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }

                conn.execute(
                    "UPDATE tag_groups SET name = ?1, color = ?2, display_order = ?3, updated_at = unixepoch() WHERE id = ?4",
                    (&name, &color, display_order, id),
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
                let rows = conn.execute("DELETE FROM tag_groups WHERE id = ?1", [id])?;
                Ok::<usize, rusqlite::Error>(rows)
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        if deleted == 0 {
            return Err(DomainError::TagGroupNotFound(id.to_string()));
        }

        Ok(())
    }

    async fn reorder(&self, orders: Vec<(i64, i32)>) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                for (id, order) in orders {
                    conn.execute(
                        "UPDATE tag_groups SET display_order = ?1, updated_at = unixepoch() WHERE id = ?2",
                        (order, id),
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

    async fn exists(&self, id: i64) -> Result<bool, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM tag_groups WHERE id = ?1",
                [id],
                |row| row.get::<_, i64>(0).map(|count| count > 0),
            )?;
            Ok::<bool, rusqlite::Error>(exists)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }
}

use rusqlite::OptionalExtension;

fn map_pool_error(e: deadpool_sqlite::PoolError) -> DomainError {
    DomainError::ValidationError(format!("Database pool error: {}", e))
}

fn map_interact_error(e: deadpool_sqlite::InteractError) -> DomainError {
    DomainError::ValidationError(format!("Database interaction error: {}", e))
}

fn map_db_error(e: rusqlite::Error) -> DomainError {
    match e {
        rusqlite::Error::QueryReturnedNoRows => {
            DomainError::TagGroupNotFound("Tag group not found".to_string())
        }
        _ => DomainError::ValidationError(format!("Database error: {}", e)),
    }
}
