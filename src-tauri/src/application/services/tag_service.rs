//! Tag Application Service
//!
//! Orchestrates tag-related operations.

use crate::application::dto::{CreateTagDto, TagDto, UpdateTagDto};
use crate::domain::entities::Tag;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{TagGroupRepository, TagRepository};
use crate::domain::value_objects::TagValue;
use std::collections::HashMap;
use std::sync::Arc;

/// Service for tag operations.
pub struct TagService {
    tag_repo: Arc<dyn TagRepository>,
    group_repo: Arc<dyn TagGroupRepository>,
}

impl TagService {
    pub fn new(tag_repo: Arc<dyn TagRepository>, group_repo: Arc<dyn TagGroupRepository>) -> Self {
        Self {
            tag_repo,
            group_repo,
        }
    }

    /// Creates a new tag.
    pub async fn create(&self, dto: CreateTagDto) -> Result<i64, DomainError> {
        // Verify group exists
        if !self.group_repo.exists(dto.group_id).await? {
            return Err(DomainError::TagGroupNotFound(dto.group_id.to_string()));
        }

        let value = TagValue::new(&dto.value)?;
        let mut tag = Tag::new(dto.group_id, value);
        self.tag_repo.save(&mut tag).await
    }

    /// Gets tags by group.
    pub async fn get_by_group(&self, group_id: i64) -> Result<Vec<TagDto>, DomainError> {
        let tags = self.tag_repo.find_by_group(group_id).await?;
        Ok(tags.into_iter().map(Self::to_dto).collect())
    }

    /// Gets all tags.
    pub async fn get_all(&self) -> Result<Vec<TagDto>, DomainError> {
        let tags = self.tag_repo.find_all().await?;
        Ok(tags.into_iter().map(Self::to_dto).collect())
    }

    /// Gets a tag by ID.
    #[allow(dead_code)]
    pub async fn get_by_id(&self, id: i64) -> Result<Option<TagDto>, DomainError> {
        let tag = self.tag_repo.find_by_id(id).await?;
        Ok(tag.map(Self::to_dto))
    }

    /// Updates a tag (value and/or group).
    pub async fn update(&self, id: i64, dto: UpdateTagDto) -> Result<(), DomainError> {
        let mut tag = self
            .tag_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::TagNotFound(id.to_string()))?;

        if let Some(value) = dto.value {
            let value = TagValue::new(&value)?;
            tag.update_value(value);
        }

        if let Some(group_id) = dto.group_id {
            // Verify target group exists
            if !self.group_repo.exists(group_id).await? {
                return Err(DomainError::TagGroupNotFound(group_id.to_string()));
            }
            tag.move_to_group(group_id)?;
        }

        self.tag_repo.update(&tag).await
    }

    /// Merges a source tag into a target tag.
    ///
    /// Reassigns all item associations from source to target (with deduplication),
    /// then deletes the source tag.
    pub async fn merge(&self, source_id: i64, target_id: i64) -> Result<(), DomainError> {
        // Verify both tags exist
        self.tag_repo
            .find_by_id(source_id)
            .await?
            .ok_or_else(|| DomainError::TagNotFound(source_id.to_string()))?;

        self.tag_repo
            .find_by_id(target_id)
            .await?
            .ok_or_else(|| DomainError::TagNotFound(target_id.to_string()))?;

        // Reassign all item associations
        self.tag_repo.reassign_items(source_id, target_id).await?;

        // Delete the source tag
        self.tag_repo.delete(source_id).await
    }

    /// Deletes a tag.
    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        self.tag_repo.delete(id).await
    }

    /// Searches tags.
    pub async fn search(
        &self,
        query: &str,
        group_id: Option<i64>,
    ) -> Result<Vec<TagDto>, DomainError> {
        let tags = self.tag_repo.search(query, group_id, 10).await?;
        Ok(tags.into_iter().map(Self::to_dto).collect())
    }

    /// Gets usage counts for all tags.
    pub async fn get_usage_counts(&self) -> Result<HashMap<i64, i64>, DomainError> {
        self.tag_repo.get_usage_counts().await
    }

    fn to_dto(tag: Tag) -> TagDto {
        TagDto {
            id: tag.id().unwrap_or(0),
            group_id: tag.group_id(),
            value: tag.value().to_string(),
            created_at: tag.created_at().unwrap_or(0),
            updated_at: tag.updated_at().unwrap_or(0),
        }
    }
}
