//! Item Entity
//!
//! Represents a file or directory that can be tagged.

use crate::domain::value_objects::FilePath;

/// Represents a file or directory item in the system.
#[derive(Debug, Clone)]
pub struct Item {
    id: Option<i64>,
    path: FilePath,
    is_directory: bool,
    size: Option<i64>,
    modified_time: Option<i64>,
    created_at: Option<i64>,
    updated_at: Option<i64>,
}

impl Item {
    /// Creates a new Item (not yet persisted).
    pub fn new(
        path: FilePath,
        is_directory: bool,
        size: Option<i64>,
        modified_time: Option<i64>,
    ) -> Self {
        Self {
            id: None,
            path,
            is_directory,
            size,
            modified_time,
            created_at: None,
            updated_at: None,
        }
    }

    /// Reconstitutes an Item from persistence.
    pub fn reconstitute(
        id: i64,
        path: FilePath,
        is_directory: bool,
        size: Option<i64>,
        modified_time: Option<i64>,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id: Some(id),
            path,
            is_directory,
            size,
            modified_time,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }

    // Getters

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn path(&self) -> &FilePath {
        &self.path
    }

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn size(&self) -> Option<i64> {
        self.size
    }

    pub fn modified_time(&self) -> Option<i64> {
        self.modified_time
    }

    pub fn created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<i64> {
        self.updated_at
    }

    // Domain behavior

    /// Updates the item's path.
    pub fn update_path(&mut self, path: FilePath) {
        self.path = path;
    }

    /// Updates the item's size.
    pub fn update_size(&mut self, size: Option<i64>) {
        self.size = size;
    }

    /// Updates the item's modified time.
    pub fn update_modified_time(&mut self, modified_time: Option<i64>) {
        self.modified_time = modified_time;
    }

    /// Sets the ID after persistence (used by repository).
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => self.path == other.path,
        }
    }
}

impl Eq for Item {}
