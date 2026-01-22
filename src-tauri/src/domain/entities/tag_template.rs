//! TagTemplate Entity
//!
//! Represents a saved combination of tags that can be applied to items.

use crate::domain::errors::DomainError;

/// Represents a template containing a set of tags.
#[derive(Debug, Clone)]
pub struct TagTemplate {
    id: Option<i64>,
    name: String,
    tag_ids: Vec<i64>,
    created_at: Option<i64>,
    updated_at: Option<i64>,
}

impl TagTemplate {
    /// Creates a new TagTemplate (not yet persisted).
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty.
    pub fn new(name: String, tag_ids: Vec<i64>) -> Result<Self, DomainError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Template name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name,
            tag_ids,
            created_at: None,
            updated_at: None,
        })
    }

    /// Reconstitutes a TagTemplate from persistence.
    pub fn reconstitute(
        id: i64,
        name: String,
        tag_ids: Vec<i64>,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            tag_ids,
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

    pub fn tag_ids(&self) -> &[i64] {
        &self.tag_ids
    }

    pub fn created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<i64> {
        self.updated_at
    }

    // Domain behavior

    /// Updates the template's name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty.
    pub fn update_name(&mut self, name: String) -> Result<(), DomainError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Template name cannot be empty".to_string(),
            ));
        }
        self.name = name;
        Ok(())
    }

    /// Updates the template's tag IDs.
    pub fn update_tags(&mut self, tag_ids: Vec<i64>) {
        self.tag_ids = tag_ids;
    }

    /// Adds a tag to the template.
    pub fn add_tag(&mut self, tag_id: i64) {
        if !self.tag_ids.contains(&tag_id) {
            self.tag_ids.push(tag_id);
        }
    }

    /// Removes a tag from the template.
    pub fn remove_tag(&mut self, tag_id: i64) {
        self.tag_ids.retain(|&id| id != tag_id);
    }

    /// Sets the ID after persistence (used by repository).
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

impl PartialEq for TagTemplate {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => self.name == other.name,
        }
    }
}

impl Eq for TagTemplate {}
