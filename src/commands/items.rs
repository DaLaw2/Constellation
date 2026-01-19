use crate::db::models::Item;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::{Connection, OptionalExtension};
use tauri::State;

#[tauri::command]
pub async fn create_item(
    path: String,
    is_directory: bool,
    size: Option<i64>,
    modified_time: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Err(AppError::InvalidInput("Path cannot be empty".to_string()));
    }

    let conn = state.db_pool.get().await?;

    let id = conn
        .interact(move |conn: &mut Connection| {
            conn.execute(
                "INSERT INTO items (path, is_directory, size, modified_time) VALUES (?1, ?2, ?3, ?4)",
                (&path, &is_directory, &size, &modified_time),
            )?;
            Ok::<i64, rusqlite::Error>(conn.last_insert_rowid())
        })
        .await??;

    Ok(id)
}

#[tauri::command]
pub async fn get_item(id: i64, state: State<'_, AppState>) -> AppResult<Item> {
    let conn = state.db_pool.get().await?;

    let item = conn
        .interact(move |conn: &mut Connection| {
            let item = conn.query_row(
                "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                 FROM items
                 WHERE id = ?1 AND is_deleted = 0",
                [id],
                |row| {
                    Ok(Item {
                        id: row.get(0)?,
                        path: row.get(1)?,
                        is_directory: row.get(2)?,
                        size: row.get(3)?,
                        modified_time: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                        is_deleted: row.get(7)?,
                        deleted_at: row.get(8)?,
                    })
                },
            )?;
            Ok::<Item, rusqlite::Error>(item)
        })
        .await??;

    Ok(item)
}

#[tauri::command]
pub async fn get_item_by_path(path: String, state: State<'_, AppState>) -> AppResult<Option<Item>> {
    let conn = state.db_pool.get().await?;

    let item = conn
        .interact(move |conn: &mut Connection| {
            let result = conn.query_row(
                "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                 FROM items
                 WHERE path = ?1 AND is_deleted = 0",
                [&path],
                |row| {
                    Ok(Item {
                        id: row.get(0)?,
                        path: row.get(1)?,
                        is_directory: row.get(2)?,
                        size: row.get(3)?,
                        modified_time: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                        is_deleted: row.get(7)?,
                        deleted_at: row.get(8)?,
                    })
                },
            );

            match result {
                Ok(item) => Ok(Some(item)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e),
            }
        })
        .await??;

    Ok(item)
}

#[tauri::command]
pub async fn update_item(
    id: i64,
    path: Option<String>,
    size: Option<i64>,
    modified_time: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction for atomic multi-statement update
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if item exists
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM items WHERE id = ?1",
                [id],
                |row| row.get::<_, i64>(0).map(|count| count > 0),
            )?;

            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            if let Some(path) = path {
                let path = path.trim();
                if path.is_empty() {
                    return Err(rusqlite::Error::InvalidQuery);
                }
                conn.execute(
                    "UPDATE items SET path = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (path, id),
                )?;
            }

            if let Some(size) = size {
                conn.execute(
                    "UPDATE items SET size = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (size, id),
                )?;
            }

            if let Some(modified_time) = modified_time {
                conn.execute(
                    "UPDATE items SET modified_time = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (modified_time, id),
                )?;
            }

            Ok::<(), rusqlite::Error>(())
        })();

        // Commit on success, rollback on error
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
    .await??;

    Ok(())
}

#[tauri::command]
pub async fn soft_delete_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction for atomic soft delete
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if item exists and is not already deleted
            let item_deleted: Option<bool> = conn.query_row(
                "SELECT is_deleted FROM items WHERE id = ?1",
                [id],
                |row| row.get(0),
            ).optional()?;

            match item_deleted {
                None => return Err(rusqlite::Error::QueryReturnedNoRows),
                Some(true) => return Err(rusqlite::Error::InvalidQuery), // Already deleted
                Some(false) => {}
            }

            // Soft delete: Set is_deleted=1 and deleted_at atomically
            conn.execute(
                "UPDATE items SET is_deleted = 1, deleted_at = unixepoch(), updated_at = unixepoch() WHERE id = ?1",
                [id],
            )?;

            Ok::<(), rusqlite::Error>(())
        })();

        // Commit on success, rollback on error
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
    .await??;

    Ok(())
}

#[tauri::command]
pub async fn restore_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    let restored = conn
        .interact(move |conn: &mut Connection| {
            let rows = conn.execute(
                "UPDATE items SET is_deleted = 0, deleted_at = NULL, updated_at = unixepoch() WHERE id = ?1 AND is_deleted = 1",
                [id],
            )?;
            Ok::<usize, rusqlite::Error>(rows)
        })
        .await??;

    if restored == 0 {
        return Err(AppError::NotFound(format!("Deleted item with id {}", id)));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_deleted_items(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    let conn = state.db_pool.get().await?;

    let items = conn
        .interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, path, is_directory, size, modified_time, created_at, updated_at, is_deleted, deleted_at
                 FROM items
                 WHERE is_deleted = 1
                 ORDER BY deleted_at DESC"
            )?;

            let items = stmt
                .query_map([], |row| {
                    Ok(Item {
                        id: row.get(0)?,
                        path: row.get(1)?,
                        is_directory: row.get(2)?,
                        size: row.get(3)?,
                        modified_time: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                        is_deleted: row.get(7)?,
                        deleted_at: row.get(8)?,
                    })
                })?
                .collect::<Result<Vec<Item>, _>>()?;

            Ok::<Vec<Item>, rusqlite::Error>(items)
        })
        .await??;

    Ok(items)
}

#[tauri::command]
pub async fn permanently_delete_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    let deleted = conn
        .interact(move |conn: &mut Connection| {
            let rows = conn.execute("DELETE FROM items WHERE id = ?1", [id])?;
            Ok::<usize, rusqlite::Error>(rows)
        })
        .await??;

    if deleted == 0 {
        return Err(AppError::NotFound(format!("Item with id {}", id)));
    }

    Ok(())
}

#[tauri::command]
pub async fn add_tag_to_item(
    item_id: i64,
    tag_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        conn.execute(
            "INSERT OR IGNORE INTO item_tags (item_id, tag_id) VALUES (?1, ?2)",
            (item_id, tag_id),
        )?;
        Ok::<(), rusqlite::Error>(())
    })
    .await??;

    Ok(())
}

#[tauri::command]
pub async fn remove_tag_from_item(
    item_id: i64,
    tag_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        conn.execute(
            "DELETE FROM item_tags WHERE item_id = ?1 AND tag_id = ?2",
            (item_id, tag_id),
        )?;
        Ok::<(), rusqlite::Error>(())
    })
    .await??;

    Ok(())
}

#[tauri::command]
pub async fn get_tags_for_item(
    item_id: i64,
    state: State<'_, AppState>,
) -> AppResult<Vec<i64>> {
    let conn = state.db_pool.get().await?;

    let tag_ids = conn
        .interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT tag_id FROM item_tags WHERE item_id = ?1"
            )?;

            let tags = stmt
                .query_map([item_id], |row| row.get(0))?
                .collect::<Result<Vec<i64>, _>>()?;

            Ok::<Vec<i64>, rusqlite::Error>(tags)
        })
        .await??;

    Ok(tag_ids)
}
