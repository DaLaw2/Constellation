//! FilePath Value Object
//!
//! Represents a validated file system path with security constraints.

use crate::domain::errors::DomainError;
use std::path::{Component, PathBuf};

/// A validated file path that is safe from path traversal attacks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath {
    value: String,
}

impl FilePath {
    /// Fallback value for invalid file path data from database.
    /// This should only be used in repository layer when recovering from corrupted data.
    const INVALID: &'static str = "[INVALID_PATH]";

    /// Creates a new FilePath after validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path is empty
    /// - The path contains path traversal patterns (../, ./, etc.)
    pub fn new(path: impl Into<String>) -> Result<Self, DomainError> {
        let path = path.into();
        let trimmed = path.trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidFilePath(
                "Path cannot be empty".to_string(),
            ));
        }

        Self::validate_no_traversal(trimmed)?;

        Ok(Self {
            value: trimmed.to_string(),
        })
    }

    /// Creates a FilePath without validation.
    ///
    /// # Safety
    ///
    /// This bypasses validation and should ONLY be used in the repository layer
    /// as a fallback when recovering from corrupted database data.
    /// The caller must ensure the path is non-empty.
    pub(crate) fn new_unchecked(value: String) -> Self {
        Self { value }
    }

    /// Returns the fallback invalid file path value.
    /// Used by repositories when encountering corrupted data.
    pub(crate) fn invalid() -> Self {
        Self {
            value: Self::INVALID.to_string(),
        }
    }

    /// Validates that the path does not contain traversal patterns.
    fn validate_no_traversal(path: &str) -> Result<(), DomainError> {
        let path_buf = PathBuf::from(path);

        // Check for path traversal patterns in components
        for component in path_buf.components() {
            match component {
                Component::ParentDir => {
                    return Err(DomainError::InvalidFilePath(
                        "Path traversal not allowed: '..' detected".to_string(),
                    ));
                }
                Component::CurDir => {
                    return Err(DomainError::InvalidFilePath(
                        "Path traversal not allowed: '.' detected".to_string(),
                    ));
                }
                _ => {}
            }
        }

        // Also check raw string for encoded or hidden traversal patterns
        if path.contains("..") || path.contains("./") || path.contains(".\\") {
            return Err(DomainError::InvalidFilePath(
                "Path traversal patterns not allowed".to_string(),
            ));
        }

        Ok(())
    }

    /// Returns the path as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the FilePath and returns the inner string.
    pub fn into_string(self) -> String {
        self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_path_is_accepted() {
        let path = FilePath::new("C:\\Users\\test\\file.txt");
        assert!(path.is_ok());
    }

    #[test]
    fn empty_path_is_rejected() {
        let path = FilePath::new("");
        assert!(matches!(path, Err(DomainError::InvalidFilePath(_))));
    }

    #[test]
    fn path_traversal_is_rejected() {
        let path = FilePath::new("../secret/file.txt");
        assert!(matches!(path, Err(DomainError::InvalidFilePath(_))));
    }

    #[test]
    fn current_dir_is_rejected() {
        let path = FilePath::new("./file.txt");
        assert!(matches!(path, Err(DomainError::InvalidFilePath(_))));
    }
}
