use crate::db::models::{Tag, TagTemplate};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::Connection;
use tauri::State;

/// Tag template with its associated tags
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TagTemplateWithTags {
    pub id: i64,
    pub name: String,
    pub tags: Vec<Tag>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[tauri::command]
pub async fn create_tag_template(
    name: String,
    tag_ids: Vec<i64>,
    state: State<'_, AppState>,
) -> AppResult<i64> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::InvalidInput(
            "Template name cannot be empty".to_string(),
        ));
    }

    let conn = state.db_pool.get().await?;

    let id = conn
        .interact(move |conn: &mut Connection| {
            // Begin transaction
            conn.execute("BEGIN IMMEDIATE", [])?;

            let result = (|| {
                // Insert template
                conn.execute("INSERT INTO tag_templates (name) VALUES (?1)", [&name])?;
                let template_id = conn.last_insert_rowid();

                // Insert template-tag associations
                for tag_id in tag_ids {
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
        .await??;

    Ok(id)
}

#[tauri::command]
pub async fn get_tag_templates(state: State<'_, AppState>) -> AppResult<Vec<TagTemplateWithTags>> {
    let conn = state.db_pool.get().await?;

    let templates = conn
        .interact(move |conn: &mut Connection| {
            // Get all templates
            let mut stmt = conn.prepare(
                "SELECT id, name, created_at, updated_at FROM tag_templates ORDER BY name ASC",
            )?;

            let template_rows = stmt
                .query_map([], |row| {
                    Ok(TagTemplate {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        created_at: row.get(2)?,
                        updated_at: row.get(3)?,
                    })
                })?
                .collect::<Result<Vec<TagTemplate>, _>>()?;

            // For each template, get its tags
            let mut templates_with_tags = Vec::new();

            for template in template_rows {
                let mut tag_stmt = conn.prepare(
                    "SELECT t.id, t.group_id, t.value, t.created_at, t.updated_at
                     FROM tags t
                     INNER JOIN template_tags tt ON tt.tag_id = t.id
                     WHERE tt.template_id = ?1
                     ORDER BY t.value ASC",
                )?;

                let tags = tag_stmt
                    .query_map([template.id], |row| {
                        Ok(Tag {
                            id: row.get(0)?,
                            group_id: row.get(1)?,
                            value: row.get(2)?,
                            created_at: row.get(3)?,
                            updated_at: row.get(4)?,
                        })
                    })?
                    .collect::<Result<Vec<Tag>, _>>()?;

                templates_with_tags.push(TagTemplateWithTags {
                    id: template.id,
                    name: template.name,
                    tags,
                    created_at: template.created_at,
                    updated_at: template.updated_at,
                });
            }

            Ok::<Vec<TagTemplateWithTags>, rusqlite::Error>(templates_with_tags)
        })
        .await??;

    Ok(templates)
}

#[tauri::command]
pub async fn apply_tag_template(
    item_id: i64,
    template_id: i64,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if item exists
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM items WHERE id = ?1 AND is_deleted = 0",
                [item_id],
                |row| row.get::<_, i64>(0).map(|count| count > 0),
            )?;

            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            // Get tags from template
            let mut stmt =
                conn.prepare("SELECT tag_id FROM template_tags WHERE template_id = ?1")?;

            let tag_ids: Vec<i64> = stmt
                .query_map([template_id], |row| row.get(0))?
                .collect::<Result<Vec<i64>, _>>()?;

            // Add tags to item (INSERT OR IGNORE to handle duplicates)
            for tag_id in tag_ids {
                conn.execute(
                    "INSERT OR IGNORE INTO item_tags (item_id, tag_id) VALUES (?1, ?2)",
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
    .await??;

    Ok(())
}

#[tauri::command]
pub async fn delete_tag_template(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    let deleted = conn
        .interact(move |conn: &mut Connection| {
            let rows = conn.execute("DELETE FROM tag_templates WHERE id = ?1", [id])?;
            Ok::<usize, rusqlite::Error>(rows)
        })
        .await??;

    if deleted == 0 {
        return Err(AppError::NotFound(format!("Tag template with id {}", id)));
    }

    Ok(())
}

#[tauri::command]
pub async fn update_tag_template(
    id: i64,
    name: Option<String>,
    tag_ids: Option<Vec<i64>>,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        // Begin transaction
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if template exists
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM tag_templates WHERE id = ?1",
                [id],
                |row| row.get::<_, i64>(0).map(|count| count > 0),
            )?;

            if !exists {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            // Update name if provided
            if let Some(name) = name {
                let name = name.trim();
                if name.is_empty() {
                    return Err(rusqlite::Error::InvalidQuery);
                }
                conn.execute(
                    "UPDATE tag_templates SET name = ?1, updated_at = unixepoch() WHERE id = ?2",
                    (name, id),
                )?;
            }

            // Update tags if provided
            if let Some(tag_ids) = tag_ids {
                // Delete existing associations
                conn.execute("DELETE FROM template_tags WHERE template_id = ?1", [id])?;

                // Insert new associations
                for tag_id in tag_ids {
                    conn.execute(
                        "INSERT INTO template_tags (template_id, tag_id) VALUES (?1, ?2)",
                        (id, tag_id),
                    )?;
                }

                conn.execute(
                    "UPDATE tag_templates SET updated_at = unixepoch() WHERE id = ?1",
                    [id],
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
    .await??;

    Ok(())
}
