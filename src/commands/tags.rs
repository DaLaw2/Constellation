use crate::db::models::Tag;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::Connection;
use tauri::State;

#[tauri::command]
pub async fn create_tag(
    group_id: i64,
    value: String,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(AppError::InvalidInput(
            "Tag value cannot be empty".to_string(),
        ));
    }

    let conn = state.db_pool.get().await?;

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
        .await??;

    Ok(id)
}

#[tauri::command]
pub async fn get_tags_by_group(group_id: i64, state: State<'_, AppState>) -> AppResult<Vec<Tag>> {
    let conn = state.db_pool.get().await?;

    let tags = conn
        .interact(move |conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags
                 WHERE group_id = ?1
                 ORDER BY value ASC",
            )?;

            let tags = stmt
                .query_map([group_id], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        group_id: row.get(1)?,
                        value: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await??;

    Ok(tags)
}

#[tauri::command]
pub async fn get_all_tags(state: State<'_, AppState>) -> AppResult<Vec<Tag>> {
    let conn = state.db_pool.get().await?;

    let tags = conn
        .interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags
                 ORDER BY group_id ASC, value ASC",
            )?;

            let tags = stmt
                .query_map([], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        group_id: row.get(1)?,
                        value: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await??;

    Ok(tags)
}

#[tauri::command]
pub async fn update_tag(
    id: i64,
    value: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction for atomic update operation
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if tag exists
            let exists: bool =
                conn.query_row("SELECT COUNT(*) FROM tags WHERE id = ?1", [id], |row| {
                    row.get::<_, i64>(0).map(|count| count > 0)
                })?;

            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            if let Some(value) = value {
                let value = value.trim();
                if value.is_empty() {
                    return Err(rusqlite::Error::InvalidQuery);
                }
                conn.execute(
                    "UPDATE tags SET value = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (value, id),
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
pub async fn delete_tag(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    let deleted = conn
        .interact(move |conn: &mut Connection| {
            let rows = conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
            Ok::<usize, rusqlite::Error>(rows)
        })
        .await??;

    if deleted == 0 {
        return Err(AppError::NotFound(format!("Tag with id {}", id)));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_tag_usage_counts(
    state: State<'_, AppState>,
) -> AppResult<std::collections::HashMap<i64, i64>> {
    let conn = state.db_pool.get().await?;

    let counts = conn
        .interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT tag_id, COUNT(*) as count 
                 FROM item_tags 
                 GROUP BY tag_id",
            )?;

            let rows =
                stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;

            let mut map = std::collections::HashMap::new();
            for row in rows {
                let (tag_id, count) = row?;
                map.insert(tag_id, count);
            }

            Ok::<std::collections::HashMap<i64, i64>, rusqlite::Error>(map)
        })
        .await??;

    Ok(counts)
}

#[tauri::command]
pub async fn search_tags(
    query: String,
    group_id: Option<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<Tag>> {
    let query = query.trim().to_string();
    let conn = state.db_pool.get().await?;

    let tags = conn
        .interact(move |conn: &mut Connection| {
            // If query is empty and no group specified, return empty
            if query.is_empty() && group_id.is_none() {
                return Ok(Vec::new());
            }

            let sql = if let Some(gid) = group_id {
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags
                 WHERE group_id = ?1 AND value LIKE ?2
                 ORDER BY value ASC
                 LIMIT 10"
            } else {
                "SELECT id, group_id, value, created_at, updated_at
                 FROM tags
                 WHERE value LIKE ?1
                 ORDER BY value ASC
                 LIMIT 10"
            };

            let pattern = format!("%{}%", query);

            let mut stmt = conn.prepare(sql)?;

            let tags = if let Some(gid) = group_id {
                stmt.query_map((gid, &pattern), |row| {
                    Ok(Tag {
                        id: row.get::<_, i64>(0)?,
                        group_id: row.get::<_, i64>(1)?,
                        value: row.get::<_, String>(2)?,
                        created_at: row.get::<_, i64>(3)?,
                        updated_at: row.get::<_, i64>(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?
            } else {
                stmt.query_map((&pattern,), |row| {
                    Ok(Tag {
                        id: row.get::<_, i64>(0)?,
                        group_id: row.get::<_, i64>(1)?,
                        value: row.get::<_, String>(2)?,
                        created_at: row.get::<_, i64>(3)?,
                        updated_at: row.get::<_, i64>(4)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?
            };

            Ok::<Vec<Tag>, rusqlite::Error>(tags)
        })
        .await??;

    Ok(tags)
}
