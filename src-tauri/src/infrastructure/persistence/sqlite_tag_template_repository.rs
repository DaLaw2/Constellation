//! SQLite TagTemplate Repository
//!
//! Implementation of TagTemplateRepository for SQLite.

use crate::domain::entities::TagTemplate;
use crate::domain::errors::DomainError;
use crate::domain::repositories::TagTemplateRepository;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::{Connection, OptionalExtension};
use std::sync::Arc;

/// SQLite implementation of TagTemplateRepository.
pub struct SqliteTagTemplateRepository {
    pool: Arc<Pool>,
}

impl SqliteTagTemplateRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagTemplateRepository for SqliteTagTemplateRepository {
    async fn save(&self, template: &mut TagTemplate) -> Result<i64, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let name = template.name().to_string();
        let tag_ids = template.tag_ids().to_vec();

        let id = conn
            .interact(move |conn: &mut Connection| {
                conn.execute("BEGIN IMMEDIATE", [])?;

                let result = (|| {
                    conn.execute("INSERT INTO tag_templates (name) VALUES (?1)", [&name])?;
                    let template_id = conn.last_insert_rowid();

                    for tag_id in &tag_ids {
                        conn.execute(
                            "INSERT INTO template_tags (template_id, tag_id) VALUES (?1, ?2)",
                            (template_id, tag_id),
                        )?;
                    }

                    Ok::<i64, rusqlite::Error>(template_id)
                })();

                match result {
                    Ok(id) => {
                        conn.execute("COMMIT", [])?;
                        Ok(id)
                    }
                    Err(e) => {
                        conn.execute("ROLLBACK", [])?;
                        Err(e)
                    }
                }
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        template.set_id(id);
        Ok(id)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<TagTemplate>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let template_data = conn
                .query_row(
                    "SELECT id, name, created_at, updated_at FROM tag_templates WHERE id = ?1",
                    [id],
                    |row| {
                        Ok((
                            row.get::<_, i64>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, i64>(2)?,
                            row.get::<_, i64>(3)?,
                        ))
                    },
                )
                .optional()?;

            if let Some((id, name, created_at, updated_at)) = template_data {
                let mut stmt =
                    conn.prepare("SELECT tag_id FROM template_tags WHERE template_id = ?1")?;
                let tag_ids = stmt
                    .query_map([id], |row| row.get(0))?
                    .collect::<Result<Vec<i64>, _>>()?;

                Ok(Some(TagTemplate::reconstitute(
                    id, name, tag_ids, created_at, updated_at,
                )))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn find_all(&self) -> Result<Vec<TagTemplate>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, name, created_at, updated_at FROM tag_templates ORDER BY name ASC",
            )?;

            let templates_data = stmt
                .query_map([], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            let mut templates = Vec::new();
            for (id, name, created_at, updated_at) in templates_data {
                let mut tag_stmt =
                    conn.prepare("SELECT tag_id FROM template_tags WHERE template_id = ?1")?;
                let tag_ids = tag_stmt
                    .query_map([id], |row| row.get(0))?
                    .collect::<Result<Vec<i64>, _>>()?;

                templates.push(TagTemplate::reconstitute(
                    id, name, tag_ids, created_at, updated_at,
                ));
            }

            Ok::<Vec<TagTemplate>, rusqlite::Error>(templates)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn update(&self, template: &TagTemplate) -> Result<(), DomainError> {
        let id = template.id().ok_or_else(|| {
            DomainError::ValidationError("Cannot update template without ID".to_string())
        })?;

        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let name = template.name().to_string();
        let tag_ids = template.tag_ids().to_vec();

        conn.interact(move |conn: &mut Connection| {
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                let exists: bool = conn.query_row(
                    "SELECT COUNT(*) FROM tag_templates WHERE id = ?1",
                    [id],
                    |row| row.get::<_, i64>(0).map(|count| count > 0),
                )?;

                if !exists {
                    return Err(rusqlite::Error::QueryReturnedNoRows);
                }

                conn.execute(
                    "UPDATE tag_templates SET name = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (&name, id),
                )?;

                conn.execute("DELETE FROM template_tags WHERE template_id = ?1", [id])?;

                for tag_id in &tag_ids {
                    conn.execute(
                        "INSERT INTO template_tags (template_id, tag_id) VALUES (?1, ?2)",
                        (id, tag_id),
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

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        let deleted = conn
            .interact(move |conn: &mut Connection| {
                let rows = conn.execute("DELETE FROM tag_templates WHERE id = ?1", [id])?;
                Ok::<usize, rusqlite::Error>(rows)
            })
            .await
            .map_err(map_interact_error)?
            .map_err(map_db_error)?;

        if deleted == 0 {
            return Err(DomainError::TagTemplateNotFound(id.to_string()));
        }

        Ok(())
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
            DomainError::TagTemplateNotFound("Tag template not found".to_string())
        }
        _ => DomainError::ValidationError(format!("Database error: {}", e)),
    }
}
