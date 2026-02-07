//! SQLite Settings Repository
//!
//! Implementation of SettingsRepository for SQLite.

use crate::domain::errors::DomainError;
use crate::domain::repositories::SettingsRepository;
use async_trait::async_trait;
use deadpool_sqlite::Pool;
use rusqlite::Connection;
use std::sync::Arc;

/// SQLite implementation of SettingsRepository.
pub struct SqliteSettingsRepository {
    pool: Arc<Pool>,
}

impl SqliteSettingsRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn get(&self, key: &str) -> Result<Option<String>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let key = key.to_string();

        conn.interact(move |conn: &mut Connection| {
            let result = conn
                .query_row("SELECT value FROM settings WHERE key = ?1", [&key], |row| {
                    row.get::<_, String>(0)
                })
                .optional();
            match result {
                Ok(value) => Ok(value),
                Err(e) => Err(e),
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn get_all(&self) -> Result<Vec<(String, String)>, DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;

        conn.interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
            let pairs = stmt
                .query_map([], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok::<Vec<(String, String)>, rusqlite::Error>(pairs)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let key = key.to_string();
        let value = value.to_string();

        conn.interact(move |conn: &mut Connection| {
            conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                (&key, &value),
            )?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(map_interact_error)?
        .map_err(map_db_error)
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        let conn = self.pool.get().await.map_err(map_pool_error)?;
        let key = key.to_string();

        conn.interact(move |conn: &mut Connection| {
            conn.execute("DELETE FROM settings WHERE key = ?1", [&key])?;
            Ok::<(), rusqlite::Error>(())
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
    DomainError::DatabaseError(format!("Database error: {}", e))
}
