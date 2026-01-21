#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{Tag, TagGroup};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_foreign_keys_enabled() {
        // Setup temporary database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = init_database(&db_path).await.expect("Failed to init db");

        // Create a tag group
        let conn = pool.get().await.unwrap();
        let group_id: i64 = conn
            .interact(|conn| {
                conn.execute(
                    "INSERT INTO tag_groups (name, color, sort_order) VALUES (?1, ?2, ?3)",
                    rusqlite::params!["Test Group", "#ff0000", 0],
                )?;
                Ok(conn.last_insert_rowid())
            })
            .await
            .unwrap()
            .unwrap();

        // Create a tag linked to the group
        let tag_id: i64 = conn
            .interact(move |conn| {
                conn.execute(
                    "INSERT INTO tags (name, group_id, sort_order) VALUES (?1, ?2, ?3)",
                    rusqlite::params!["Test Tag", group_id, 0],
                )?;
                Ok(conn.last_insert_rowid())
            })
            .await
            .unwrap()
            .unwrap();

        // Verify tag exists
        let tag_exists: bool = conn
            .interact(move |conn| {
                let mut stmt = conn
                    .prepare("SELECT COUNT(*) FROM tags WHERE id = ?1")
                    .unwrap();
                let count: i64 = stmt
                    .query_row(rusqlite::params![tag_id], |row| row.get(0))
                    .unwrap();
                Ok(count > 0)
            })
            .await
            .unwrap()
            .unwrap();
        assert!(tag_exists, "Tag should exist before deletion");

        // Delete the group
        conn.interact(move |conn| {
            conn.execute(
                "DELETE FROM tag_groups WHERE id = ?1",
                rusqlite::params![group_id],
            )
        })
        .await
        .unwrap()
        .unwrap();

        // Verify tag is gone (Cascade)
        let tag_still_exists: bool = conn
            .interact(move |conn| {
                let mut stmt = conn
                    .prepare("SELECT COUNT(*) FROM tags WHERE id = ?1")
                    .unwrap();
                let count: i64 = stmt
                    .query_row(rusqlite::params![tag_id], |row| row.get(0))
                    .unwrap();
                Ok(count > 0)
            })
            .await
            .unwrap()
            .unwrap();

        assert!(!tag_still_exists, "Tag should have been deleted by cascade");
    }
}
