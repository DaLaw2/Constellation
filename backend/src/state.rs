use deadpool_sqlite::Pool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppConfig {
    pub db_path: String,
}

pub struct AppState {
    pub db_pool: Arc<Pool>,
    pub config: Arc<RwLock<AppConfig>>,
}

impl AppState {
    pub fn new(db_pool: Pool, config: AppConfig) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
            config: Arc::new(RwLock::new(config)),
        }
    }
}
