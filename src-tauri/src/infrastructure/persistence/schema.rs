//! Database Schema
//!
//! Defines the database schema and initialization logic.

use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite::{Connection, Result};
use std::path::Path;

/// Initializes the database and returns a connection pool.
pub async fn init_database(
    db_path: &Path,
) -> std::result::Result<Pool, Box<dyn std::error::Error>> {
    // Create database file if it doesn't exist
    if !db_path.exists() {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let cfg = Config::new(db_path);
    // SAFETY: Pool builder creation only fails if the configuration is invalid,
    // which would be a programming error. The Config::new() call above is valid.
    let pool = cfg
        .builder(Runtime::Tokio1)
        .expect("Failed to create pool builder")
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Initialize schema on first connection
    let conn = pool.get().await?;
    conn.interact(|conn: &mut Connection| {
        initialize_schema(conn)?;
        migrate_tag_group_order(conn)?;
        Ok::<(), rusqlite::Error>(())
    })
    .await??;

    Ok(pool)
}

/// Initializes the database schema.
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Tag Groups table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT,
            display_order INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
        [],
    )?;

    // Tags table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            value TEXT NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
            FOREIGN KEY (group_id) REFERENCES tag_groups(id) ON DELETE CASCADE,
            UNIQUE(group_id, value)
        )",
        [],
    )?;

    // Items table (files and folders)
    // Note: is_deleted and deleted_at columns are deprecated but kept for backward compatibility
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            is_directory BOOLEAN NOT NULL,
            size INTEGER,
            modified_time INTEGER,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
            is_deleted BOOLEAN NOT NULL DEFAULT 0,
            deleted_at INTEGER
        )",
        [],
    )?;

    // Item-Tags junction table (many-to-many)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tags (
            item_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            PRIMARY KEY (item_id, tag_id),
            FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Tag Templates table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag_templates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
        [],
    )?;

    // Template-Tags junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS template_tags (
            template_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (template_id, tag_id),
            FOREIGN KEY (template_id) REFERENCES tag_templates(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_items_path ON items(path)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_items_is_directory ON items(is_directory)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_items_is_deleted ON items(is_deleted)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_group_id ON tags(group_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_value ON tags(value)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_tags_item_id ON item_tags(item_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_tags_tag_id ON item_tags(tag_id)",
        [],
    )?;

    // Enable WAL mode for better concurrency
    let _mode: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
    conn.execute("PRAGMA synchronous=NORMAL", [])?;
    conn.execute("PRAGMA cache_size=-32000", [])?; // 32MB cache
    conn.execute("PRAGMA foreign_keys=ON", [])?;
    conn.execute("PRAGMA temp_store=MEMORY", [])?;

    Ok(())
}

/// Migrates existing tag groups to have sequential display_order values.
pub fn migrate_tag_group_order(conn: &Connection) -> Result<()> {
    let needs_migration: bool = conn.query_row(
        "SELECT COUNT(*) > 1 FROM tag_groups WHERE display_order = 0",
        [],
        |row| row.get::<_, i64>(0).map(|count| count > 1),
    )?;

    if needs_migration {
        conn.execute(
            "UPDATE tag_groups
             SET display_order = (
                 SELECT COUNT(*)
                 FROM tag_groups t2
                 WHERE t2.name < tag_groups.name
                    OR (t2.name = tag_groups.name AND t2.id < tag_groups.id)
             ),
             updated_at = unixepoch()",
            [],
        )?;
    }

    Ok(())
}
