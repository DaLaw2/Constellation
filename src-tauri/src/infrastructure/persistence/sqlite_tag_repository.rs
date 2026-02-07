//! SQLite Tag Repository
//!
//! Implementation of TagRepository for SQLite.

use crate::domain::entities::Tag;
use crate::domain::errors::DomainError;
use crate::domain::repositories::TagRepository;
use crate::domain::value_objects::TagValue;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::{Connection, OptionalExtension};
use std::collections::HashMap;
use std::sync::Arc;

/// SQLite implementation of TagRepository.
pub struct SqliteTagRepository {
    pool: Arc<Pool>,
}

impl SqliteTagRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    fn map_row_to_tag(row: &rusqlite::Row) -> rusqlite::Result<Tag> {
        let value_str: String = row.get(2)?;
        // Use safe fallback for corrupted database data
        let value = TagValue::new(value_str).unwrap_or_else(|_| TagValue::invalid());

        Ok(Tag::reconstitute(
            row.get(0)?,
            row.get(1)?,
            value,
            row.get(3)?,
            row.get(4)?,
        ))
    }
}

#[async_trait]
impl TagRepository for SqliteTagRepository {
    async fn save(&self, tag: &mut Tag) -> Result<i64, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let group_id = tag.group_id();
        let value = tag.value().to_string();

        let id = conn
            .interact(move |conn: &mut Connection| {
                // Check if group exists
                let group_exists: bool = conn.query_row(
                    "SELECT COUNT(*) FROM tag_groups WHERE id = ?1",
                    [group_id],
                    |row| row.get::<_, i64>(0).map(|count| count > 0),
                )?;

                if !group_exists {
                    return Err(rusqlite::Error::InvalidQuery);
                }

                conn.execute(
                    "INSERT INTO tags (group_id, value) VALUES (?1, ?2)",
                    (&group_id, &value),
                )?;
                Ok::<i64, rusqlite::Error>(conn.last_insert_rowid())
            })
            .await
            .map_err(map_interact_error)?
            .map_err(|e| {
                if matches!(e, rusqlite::Error::InvalidQuery) {
                    DomainError::TagGroupNotFound(tag.group_id().to_string())
                } else {
                    map_db_error(e)
                }
            })?;

        tag.set_id(id);
        Ok(id)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Tag>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let result = conn
                .query_row(
                    "SELECT id, group_id, value, created_at, updated_at FROM tags WHERE id = ?1",
                    [id],
                    Self::map_row_to_tag,
                )
                .optional()?;
            Ok::<Option<Tag>, rusqlite::Error>(result)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_ids(&self, ids: &[i64]) -> Result<Vec<Tag>, DomainError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let ids = ids.to_vec();

        conn.interact(move |conn: &mut Connection| {
            let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
            let sql = format!(
                "SELECT id, group_id, value, created_at, updated_at FROM tags WHERE id IN ({})",
                placeholders.join(", ")
            );

            let mut stmt = conn.prepare(&sql)?;
            let params: Vec<Box<dyn rusqlite::ToSql>> = ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::ToSql>)
                .collect();
            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let tags = stmt
                .query_map(params_refs.as_slice(), Self::map_row_to_tag)?
                .collect::<Result<Vec<Tag>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_group(&self, group_id: i64) -> Result<Vec<Tag>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags WHERE group_id = ?1 ORDER BY value ASC",
            )?;

            let tags = stmt
                .query_map([group_id], Self::map_row_to_tag)?
                .collect::<Result<Vec<Tag>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_all(&self) -> Result<Vec<Tag>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags ORDER BY group_id ASC, value ASC",
            )?;

            let tags = stmt
                .query_map([], Self::map_row_to_tag)?
                .collect::<Result<Vec<Tag>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn update(&self, tag: &Tag) -> Result<(), DomainError> {
        let id = tag.id().ok_or_else(|| {
            DomainError::ValidationError("Cannot update tag without ID".to_string())
        })?;

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let value = tag.value().to_string();
        let group_id = tag.group_id();

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let exists: bool =
                    conn.query_row("SELECT COUNT(*) FROM tags WHERE id = ?1", [id], |row| {
                        row.get::<_, i64>(0).map(|count| count > 0)
                    })?;

                if !exists {
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }

                conn.execute(
                    "UPDATE tags SET value = ?1, group_id = ?2, updated_at = unixepoch() WHERE id = ?3",
                    (&value, group_id, id),
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
                let rows = conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
                Ok::<usize, rusqlite::Error>(rows)
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        if deleted == 0 {
            return Err(DomainError::TagNotFound(id.to_string()));
        }

        Ok(())
    }

    async fn search(
        &self,
        query: &str,
        group_id: Option<i64>,
        limit: usize,
    ) -> Result<Vec<Tag>, DomainError> {
        let query = query.trim().to_string();
        if query.is_empty() && group_id.is_none() {
            return Ok(Vec::new());
        }

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let sql = if let Some(_gid) = group_id {
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags WHERE group_id = ?1 AND value LIKE ?2
                 ORDER BY value ASC LIMIT ?3"
            } else {
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags WHERE value LIKE ?1
                 ORDER BY value ASC LIMIT ?2"
            };

            let pattern = format!("%{}%", query);
            let mut stmt = conn.prepare(sql)?;

            let tags = if let Some(gid) = group_id {
                stmt.query_map((gid, &pattern, limit), Self::map_row_to_tag)?
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                stmt.query_map((&pattern, limit), Self::map_row_to_tag)?
                    .collect::<Result<Vec<_>, _>>()?
            };

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn get_usage_counts(&self) -> Result<HashMap<i64, i64>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT it.tag_id, COUNT(*) as count
                 FROM item_tags it
                 INNER JOIN items i ON i.id = it.item_id
                 WHERE i.is_deleted = 0
                 GROUP BY it.tag_id",
            )?;

            let rows =
                stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;

            let mut map = HashMap::new();
            for row in rows {
                let (tag_id, count) = row?;
                map.insert(tag_id, count);
            }

            Ok::<HashMap<i64, i64>, rusqlite::Error>(map)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_item(&self, item_id: i64) -> Result<Vec<Tag>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT t.id, t.group_id, t.value, t.created_at, t.updated_at
                 FROM tags t
                 INNER JOIN item_tags it ON it.tag_id = t.id
                 WHERE it.item_id = ?1
                 ORDER BY t.value ASC",
            )?;

            let tags = stmt
                .query_map([item_id], Self::map_row_to_tag)?
                .collect::<Result<Vec<Tag>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_by_items(&self, item_ids: &[i64]) -> Result<HashMap<i64, Vec<Tag>>, DomainError> {
        if item_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let ids = item_ids.to_vec();

        conn.interact(move |conn: &mut Connection| {
            let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
            let sql = format!(
                "SELECT it.item_id, t.id, t.group_id, t.value, t.created_at, t.updated_at
                 FROM item_tags it
                 INNER JOIN tags t ON t.id = it.tag_id
                 WHERE it.item_id IN ({})
                 ORDER BY it.item_id, t.value ASC",
                placeholders.join(", ")
            );

            let mut stmt = conn.prepare(&sql)?;
            let params: Vec<Box<dyn rusqlite::ToSql>> = ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::ToSql>)
                .collect();
            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let mut map: HashMap<i64, Vec<Tag>> = HashMap::new();
            let mut rows = stmt.query(params_refs.as_slice())?;
            while let Some(row) = rows.next()? {
                let item_id: i64 = row.get(0)?;
                let value_str: String = row.get(3)?;
                let value =
                    crate::domain::value_objects::TagValue::new(value_str).unwrap_or_else(|_| {
                        crate::domain::value_objects::TagValue::invalid()
                    });
                let tag = Tag::reconstitute(
                    row.get(1)?,
                    row.get(2)?,
                    value,
                    row.get(4)?,
                    row.get(5)?,
                );
                map.entry(item_id).or_default().push(tag);
            }

            Ok::<HashMap<i64, Vec<Tag>>, rusqlite::Error>(map)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn reassign_items(
        &self,
        source_tag_id: i64,
        target_tag_id: i64,
    ) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                // Update item_tags: change source to target, skip duplicates
                // First, delete rows where the item already has the target tag
                conn.execute(
                    "DELETE FROM item_tags
                     WHERE tag_id = ?1
                     AND item_id IN (
                         SELECT item_id FROM item_tags WHERE tag_id = ?2
                     )",
                    [source_tag_id, target_tag_id],
                )?;

                // Then reassign remaining source associations to target
                conn.execute(
                    "UPDATE item_tags SET tag_id = ?1 WHERE tag_id = ?2",
                    [target_tag_id, source_tag_id],
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
}

fn map_pool_error(e: deadpool_sqlite::PoolError) -> DomainError {
    DomainError::ValidationError(format!("Database pool error: {}", e))
}

fn map_interact_error(e: deadpool_sqlite::InteractError) -> DomainError {
    DomainError::ValidationError(format!("Database interaction error: {}", e))
}

fn map_db_error(e: rusqlite::Error) -> DomainError {
    match e {
        rusqlite::Error::QueryReturnedNoRows => {
            DomainError::TagNotFound("Tag not found".to_string())
        }
        _ => DomainError::ValidationError(format!("Database error: {}", e)),
    }
}
