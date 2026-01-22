//! TagGroup Application Service
//!
//! Orchestrates tag group-related operations.

use crate::application::dto::{CreateTagGroupDto, TagGroupDto, UpdateTagGroupDto};
use crate::domain::entities::TagGroup;
use crate::domain::errors::DomainError;
use crate::domain::repositories::TagGroupRepository;
use crate::domain::value_objects::Color;
use std::sync::Arc;

/// Service for tag group operations.
pub struct TagGroupService {
    repo: Arc<dyn TagGroupRepository>,
}

impl TagGroupService {
    pub fn new(repo: Arc<dyn TagGroupRepository>) -> Self {
        Self { repo }
    }

    /// Creates a new tag group.
    pub async fn create(&self, dto: CreateTagGroupDto) -> Result<i64, DomainError> {
        let color = match dto.color {
            Some(c) => Some(Color::new(c)?),
            None => None,
        };

        // Get current max display_order
        let groups = self.repo.find_all().await?;
        let max_order = groups.iter().map(|g| g.display_order()).max().unwrap_or(0);

        let mut group = TagGroup::new(dto.name, color, max_order + 1)?;
        self.repo.save(&mut group).await
    }

    /// Gets all tag groups.
    pub async fn get_all(&self) -> Result<Vec<TagGroupDto>, DomainError> {
        let groups = self.repo.find_all().await?;
        Ok(groups.into_iter().map(Self::to_dto).collect())
    }

    /// Gets a tag group by ID.
    pub async fn get_by_id(&self, id: i64) -> Result<Option<TagGroupDto>, DomainError> {
        let group = self.repo.find_by_id(id).await?;
        Ok(group.map(Self::to_dto))
    }

    /// Updates a tag group.
    pub async fn update(&self, id: i64, dto: UpdateTagGroupDto) -> Result<(), DomainError> {
        let mut group = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::TagGroupNotFound(id.to_string()))?;

        if let Some(name) = dto.name {
            group.update_name(name)?;
        }

        if let Some(color_str) = dto.color {
            let color = if color_str.is_empty() {
                None
            } else {
                Some(Color::new(color_str)?)
            };
            group.update_color(color);
        }

        self.repo.update(&group).await
    }

    /// Deletes a tag group.
    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }

    /// Reorders tag groups.
    pub async fn reorder(&self, orders: Vec<(i64, i32)>) -> Result<(), DomainError> {
        self.repo.reorder(orders).await
    }

    fn to_dto(group: TagGroup) -> TagGroupDto {
        TagGroupDto {
            id: group.id().unwrap_or(0),
            name: group.name().to_string(),
            color: group.color().map(|c| c.to_string()),
            display_order: group.display_order(),
            created_at: group.created_at().unwrap_or(0),
            updated_at: group.updated_at().unwrap_or(0),
        }
    }
}
