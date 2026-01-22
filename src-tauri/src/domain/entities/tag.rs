//! Tag Entity
//!
//! Represents a tag that can be applied to items.

use crate::domain::errors::DomainError;
use crate::domain::value_objects::TagValue;

/// Represents a tag within a group.
#[derive(Debug, Clone)]
pub struct Tag {
    id: Option<i64>,
    group_id: i64,
    value: TagValue,
    created_at: Option<i64>,
    updated_at: Option<i64>,
}

impl Tag {
    /// Creates a new Tag (not yet persisted).
    pub fn new(group_id: i64, value: TagValue) -> Self {
        Self {
            id: None,
            group_id,
            value,
            created_at: None,
            updated_at: None,
        }
    }

    /// Reconstitutes a Tag from persistence.
    pub fn reconstitute(
        id: i64,
        group_id: i64,
        value: TagValue,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id: Some(id),
            group_id,
            value,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }

    // Getters

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn group_id(&self) -> i64 {
        self.group_id
    }

    pub fn value(&self) -> &TagValue {
        &self.value
    }

    pub fn created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<i64> {
        self.updated_at
    }

    // Domain behavior

    /// Updates the tag's value.
    pub fn update_value(&mut self, value: TagValue) {
        self.value = value;
    }

    /// Moves the tag to a different group.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn move_to_group(&mut self, group_id: i64) -> Result<(), DomainError> {
        if group_id <= 0 {
            return Err(DomainError::ValidationError(
                "Invalid group ID".to_string(),
            ));
        }
        self.group_id = group_id;
        Ok(())
    }

    /// Sets the ID after persistence (used by repository).
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => self.group_id == other.group_id && self.value == other.value,
        }
    }
}

impl Eq for Tag {}
