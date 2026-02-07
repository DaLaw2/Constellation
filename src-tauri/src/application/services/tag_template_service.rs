//! TagTemplate Application Service
//!
//! Orchestrates tag template-related operations.

use crate::application::dto::{CreateTagTemplateDto, TagTemplateDto, UpdateTagTemplateDto};
use crate::domain::entities::TagTemplate;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{ItemRepository, TagTemplateRepository};
use std::sync::Arc;

/// Service for tag template operations.
pub struct TagTemplateService {
    template_repo: Arc<dyn TagTemplateRepository>,
    item_repo: Arc<dyn ItemRepository>,
}

impl TagTemplateService {
    pub fn new(
        template_repo: Arc<dyn TagTemplateRepository>,
        item_repo: Arc<dyn ItemRepository>,
    ) -> Self {
        Self {
            template_repo,
            item_repo,
        }
    }

    /// Creates a new tag template.
    pub async fn create(&self, dto: CreateTagTemplateDto) -> Result<i64, DomainError> {
        let mut template = TagTemplate::new(dto.name, dto.tag_ids)?;
        self.template_repo.save(&mut template).await
    }

    /// Gets all templates.
    pub async fn get_all(&self) -> Result<Vec<TagTemplateDto>, DomainError> {
        let templates = self.template_repo.find_all().await?;
        Ok(templates.into_iter().map(Self::to_dto).collect())
    }

    /// Gets a template by ID.
    #[allow(dead_code)]
    pub async fn get_by_id(&self, id: i64) -> Result<Option<TagTemplateDto>, DomainError> {
        let template = self.template_repo.find_by_id(id).await?;
        Ok(template.map(Self::to_dto))
    }

    /// Updates a template.
    pub async fn update(&self, id: i64, dto: UpdateTagTemplateDto) -> Result<(), DomainError> {
        let mut template = self
            .template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::TagTemplateNotFound(id.to_string()))?;

        if let Some(name) = dto.name {
            template.update_name(name)?;
        }

        if let Some(tag_ids) = dto.tag_ids {
            template.update_tags(tag_ids);
        }

        self.template_repo.update(&template).await
    }

    /// Deletes a template.
    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        self.template_repo.delete(id).await
    }

    /// Applies a template to an item (adds all template tags to the item).
    pub async fn apply_to_item(&self, template_id: i64, item_id: i64) -> Result<(), DomainError> {
        let template = self
            .template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::TagTemplateNotFound(template_id.to_string()))?;

        // Get existing tags and merge with template tags
        let existing_tag_ids = self.item_repo.get_tag_ids(item_id).await?;
        let mut all_tags = existing_tag_ids;

        for tag_id in template.tag_ids() {
            if !all_tags.contains(tag_id) {
                all_tags.push(*tag_id);
            }
        }

        self.item_repo.replace_tags(item_id, all_tags).await
    }

    fn to_dto(template: TagTemplate) -> TagTemplateDto {
        TagTemplateDto {
            id: template.id().unwrap_or(0),
            name: template.name().to_string(),
            tag_ids: template.tag_ids().to_vec(),
            created_at: template.created_at().unwrap_or(0),
            updated_at: template.updated_at().unwrap_or(0),
        }
    }
}
