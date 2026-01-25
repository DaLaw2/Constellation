use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Database pool error: {0}")]
    Pool(#[from] deadpool_sqlite::PoolError),

    #[error("Database interaction error: {0}")]
    Interaction(#[from] deadpool_sqlite::InteractError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),

    #[error("Domain error: {0}")]
    Domain(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
