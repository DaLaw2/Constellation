pub mod models;
pub mod schema;

use deadpool_sqlite::{Config, Pool, Runtime};
use rusqlite::Connection;
use std::path::Path;

pub async fn init_database(db_path: &Path) -> Result<Pool, Box<dyn std::error::Error>> {
    // Create database file if it doesn't exist
    if !db_path.exists() {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let cfg = Config::new(db_path);
    // Create the pool with a post_create hook to enable foreign keys
    let pool = cfg
        .builder(Runtime::Tokio1)
        .expect("Failed to create pool builder")
        /*
        .post_create(|conn, _metrics| {
            Box::pin(async move {
                conn.interact(|conn| conn.execute_batch("PRAGMA foreign_keys = ON;"))
                    .await
                    .map_err(|_| deadpool::managed::HookError::Message("Interaction failed".into()))?
                    .map_err(|e| deadpool::managed::HookError::Backend(e))
            })
        })
        */
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Initialize schema on first connection
    let conn = pool.get().await?;
    conn.interact(|conn: &mut Connection| {
        schema::initialize_schema(conn)?;
        // Run migration to fix existing tag group orders
        schema::migrate_tag_group_order(conn)?;
        Ok::<(), rusqlite::Error>(())
    })
    .await??;

    Ok(pool)
}
