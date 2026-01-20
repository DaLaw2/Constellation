use crate::db::models::Item;
use crate::error::AppResult;
use crate::state::AppState;
use rusqlite::Connection;
use tauri::State;

/// Search mode for tag-based search
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    And,
    Or,
}

/// Search items by tags with AND logic (must have ALL specified tags)
#[tauri::command]
pub async fn search_items_by_tags_and(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<Item>> {
    if tag_ids.is_empty() {
        return Ok(Vec::new());
    }

    let conn = state.db_pool.get().await?;
    let tag_count = tag_ids.len() as i64;

    let items = conn
        .interact(move |conn: &mut Connection| {
            // Build placeholders for IN clause
            let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
            let placeholders_str = placeholders.join(", ");

            let sql = format!(
                "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                        i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                 FROM items i
                 INNER JOIN item_tags it ON i.id = it.item_id
                 WHERE it.tag_id IN ({}) AND i.is_deleted = 0
                 GROUP BY i.id
                 HAVING COUNT(DISTINCT it.tag_id) = ?
                 ORDER BY i.path ASC",
                placeholders_str
            );

            let mut stmt = conn.prepare(&sql)?;

            // Bind tag_ids and tag_count
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = tag_ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::ToSql>)
                .collect();
            params.push(Box::new(tag_count));

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();

            let items = stmt
                .query_map(params_refs.as_slice(), |row| {
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

/// Search items by tags with OR logic (must have ANY of the specified tags)
#[tauri::command]
pub async fn search_items_by_tags_or(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<Vec<Item>> {
    if tag_ids.is_empty() {
        return Ok(Vec::new());
    }

    let conn = state.db_pool.get().await?;

    let items = conn
        .interact(move |conn: &mut Connection| {
            // Build placeholders for IN clause
            let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
            let placeholders_str = placeholders.join(", ");

            let sql = format!(
                "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                        i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                 FROM items i
                 INNER JOIN item_tags it ON i.id = it.item_id
                 WHERE it.tag_id IN ({}) AND i.is_deleted = 0
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
                .query_map(params_refs.as_slice(), |row| {
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

/// Search items by filename (LIKE query on path)
#[tauri::command]
pub async fn search_items_by_filename(
    query: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<Item>> {
    let query = query.trim().to_string();
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let conn = state.db_pool.get().await?;

    let items = conn
        .interact(move |conn: &mut Connection| {
            let pattern = format!("%{}%", query);

            let mut stmt = conn.prepare(
                "SELECT id, path, is_directory, size, modified_time,
                        created_at, updated_at, is_deleted, deleted_at
                 FROM items
                 WHERE path LIKE ?1 AND is_deleted = 0
                 ORDER BY path ASC",
            )?;

            let items = stmt
                .query_map([&pattern], |row| {
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

/// Combined search: filter by tags (AND/OR) and optionally by filename
#[tauri::command]
pub async fn search_items(
    tag_ids: Vec<i64>,
    mode: SearchMode,
    filename_query: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<Item>> {
    let conn = state.db_pool.get().await?;

    let filename_pattern = filename_query
        .as_ref()
        .filter(|q| !q.trim().is_empty())
        .map(|q| format!("%{}%", q.trim()));

    let has_tags = !tag_ids.is_empty();
    let has_filename = filename_pattern.is_some();

    // If no search criteria, return empty
    if !has_tags && !has_filename {
        return Ok(Vec::new());
    }

    let tag_count = tag_ids.len() as i64;

    let items = conn
        .interact(move |conn: &mut Connection| {
            // Build the query based on what criteria we have
            let sql = if has_tags && has_filename {
                // Both tags and filename
                let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
                let placeholders_str = placeholders.join(", ");

                match mode {
                    SearchMode::And => format!(
                        "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.is_deleted = 0 AND i.path LIKE ?
                         GROUP BY i.id
                         HAVING COUNT(DISTINCT it.tag_id) = ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                    SearchMode::Or => format!(
                        "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.is_deleted = 0 AND i.path LIKE ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                }
            } else if has_tags {
                // Only tags
                let placeholders: Vec<String> = tag_ids.iter().map(|_| "?".to_string()).collect();
                let placeholders_str = placeholders.join(", ");

                match mode {
                    SearchMode::And => format!(
                        "SELECT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.is_deleted = 0
                         GROUP BY i.id
                         HAVING COUNT(DISTINCT it.tag_id) = ?
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                    SearchMode::Or => format!(
                        "SELECT DISTINCT i.id, i.path, i.is_directory, i.size, i.modified_time,
                                i.created_at, i.updated_at, i.is_deleted, i.deleted_at
                         FROM items i
                         INNER JOIN item_tags it ON i.id = it.item_id
                         WHERE it.tag_id IN ({}) AND i.is_deleted = 0
                         ORDER BY i.path ASC",
                        placeholders_str
                    ),
                }
            } else {
                // Only filename
                "SELECT id, path, is_directory, size, modified_time,
                        created_at, updated_at, is_deleted, deleted_at
                 FROM items
                 WHERE path LIKE ? AND is_deleted = 0
                 ORDER BY path ASC"
                    .to_string()
            };

            let mut stmt = conn.prepare(&sql)?;

            // Build params based on what we have
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
                .query_map(params_refs.as_slice(), |row| {
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
