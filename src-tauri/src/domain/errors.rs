//! Domain Errors
//!
//! Domain-specific errors that represent business rule violations.

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum DomainError {
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),

    #[error("Invalid tag value: {0}")]
    InvalidTagValue(String),

    #[error("Invalid color format: {0}")]
    InvalidColor(String),

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("Tag not found: {0}")]
    TagNotFound(String),

    #[error("Tag group not found: {0}")]
    TagGroupNotFound(String),

    #[error("Tag template not found: {0}")]
    TagTemplateNotFound(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}
