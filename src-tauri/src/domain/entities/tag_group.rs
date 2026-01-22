//! TagGroup Entity
//!
//! Represents a group of related tags (e.g., "Priority", "Status").

use crate::domain::errors::DomainError;
use crate::domain::value_objects::Color;

/// Represents a group of related tags.
#[derive(Debug, Clone)]
pub struct TagGroup {
    id: Option<i64>,
    name: String,
    color: Option<Color>,
    display_order: i32,
    created_at: Option<i64>,
    updated_at: Option<i64>,
}

impl TagGroup {
    /// Creates a new TagGroup (not yet persisted).
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty.
    pub fn new(name: String, color: Option<Color>, display_order: i32) -> Result<Self, DomainError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Tag group name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name,
            color,
            display_order,
            created_at: None,
            updated_at: None,
        })
    }

    /// Reconstitutes a TagGroup from persistence.
    pub fn reconstitute(
        id: i64,
        name: String,
        color: Option<Color>,
        display_order: i32,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            color,
            display_order,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }

    // Getters

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    pub fn display_order(&self) -> i32 {
        self.display_order
    }

    pub fn created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<i64> {
        self.updated_at
    }

    // Domain behavior

    /// Updates the group's name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty.
    pub fn update_name(&mut self, name: String) -> Result<(), DomainError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Tag group name cannot be empty".to_string(),
            ));
        }
        self.name = name;
        Ok(())
    }

    /// Updates the group's color.
    pub fn update_color(&mut self, color: Option<Color>) {
        self.color = color;
    }

    /// Updates the group's display order.
    pub fn update_display_order(&mut self, order: i32) {
        self.display_order = order;
    }

    /// Sets the ID after persistence (used by repository).
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

impl PartialEq for TagGroup {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => self.name == other.name,
        }
    }
}

impl Eq for TagGroup {}
