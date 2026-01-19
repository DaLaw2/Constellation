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
    let pool = cfg.create_pool(Runtime::Tokio1)?;

    // Initialize schema on first connection
    let conn = pool.get().await?;
    conn.interact(|conn: &mut Connection| schema::initialize_schema(conn))
        .await??;

    Ok(pool)
}
