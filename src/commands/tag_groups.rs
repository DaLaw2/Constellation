use crate::db::models::TagGroup;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::Connection;
use tauri::State;

#[tauri::command]
pub async fn create_tag_group(
    name: String,
    color: Option<String>,
    display_order: Option<i32>,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::InvalidInput(
            "Tag group name cannot be empty".to_string(),
        ));
    }

    let conn = state.db_pool.get().await?;
    let display_order = display_order.unwrap_or(0);

    let id = conn
        .interact(move |conn: &mut Connection| {
            conn.execute(
                "INSERT INTO tag_groups (name, color, display_order) VALUES (?1, ?2, ?3)",
                (&name, &color, &display_order),
            )?;
            Ok::<i64, rusqlite::Error>(conn.last_insert_rowid())
        })
        .await??;

    Ok(id)
}

#[tauri::command]
pub async fn get_tag_groups(state: State<'_, AppState>) -> AppResult<Vec<TagGroup>> {
    let conn = state.db_pool.get().await?;

    let groups = conn
        .interact(|conn: &mut Connection| {
            let mut stmt = conn.prepare(
                "SELECT id, name, color, display_order, created_at, updated_at
                 FROM tag_groups
                 ORDER BY display_order ASC, name ASC",
            )?;

            let groups = stmt
                .query_map([], |row| {
                    Ok(TagGroup {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        color: row.get(2)?,
                        display_order: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok::<Vec<TagGroup>, rusqlite::Error>(groups)
        })
        .await??;

    Ok(groups)
}

#[tauri::command]
pub async fn update_tag_group(
    id: i64,
    name: Option<String>,
    color: Option<String>,
    display_order: Option<i32>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction for atomic multi-statement update
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if tag group exists
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM tag_groups WHERE id = ?1",
                [id],
                |row| row.get::<_, i64>(0).map(|count| count > 0),
            )?;

            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            // Update fields that are provided
            if let Some(name) = name {
                let name = name.trim();
                if name.is_empty() {
                    return Err(rusqlite::Error::InvalidQuery);
                }
                conn.execute(
                    "UPDATE tag_groups SET name = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (name, id),
                )?;
            }

            if let Some(color) = color {
                conn.execute(
                    "UPDATE tag_groups SET color = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (color, id),
                )?;
            }

            if let Some(display_order) = display_order {
                conn.execute(
                    "UPDATE tag_groups SET display_order = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (display_order, id),
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
pub async fn delete_tag_group(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    let deleted = conn
        .interact(move |conn: &mut Connection| {
            let rows = conn.execute("DELETE FROM tag_groups WHERE id = ?1", [id])?;
            Ok::<usize, rusqlite::Error>(rows)
        })
        .await??;

    if deleted == 0 {
        return Err(AppError::NotFound(format!("Tag group with id {}", id)));
    }

    Ok(())
}
