//! Item Application Service
//!
//! Orchestrates item-related operations.

use crate::application::dto::{CreateItemDto, ItemDto, TagDto, UpdateItemDto};
use crate::domain::entities::Item;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{ItemRepository, TagRepository};
use crate::domain::value_objects::FilePath;
use std::sync::Arc;

/// Service for item operations.
pub struct ItemService {
    item_repo: Arc<dyn ItemRepository>,
    tag_repo: Arc<dyn TagRepository>,
}

impl ItemService {
    pub fn new(item_repo: Arc<dyn ItemRepository>, tag_repo: Arc<dyn TagRepository>) -> Self {
        Self {
            item_repo,
            tag_repo,
        }
    }

    /// Creates a new item.
    pub async fn create(&self, dto: CreateItemDto) -> Result<i64, DomainError> {
        let path = FilePath::new(&dto.path)?;
        let mut item = Item::new(path, dto.is_directory, dto.size, dto.modified_time);
        self.item_repo.save(&mut item).await
    }

    /// Gets an item by ID.
    pub async fn get_by_id(&self, id: i64) -> Result<Option<ItemDto>, DomainError> {
        let item = self.item_repo.find_by_id(id).await?;
        Ok(item.map(Self::to_dto))
    }

    /// Gets an item by path.
    pub async fn get_by_path(&self, path: &str) -> Result<Option<ItemDto>, DomainError> {
        // Validate path first
        let validated_path = FilePath::new(path)?;
        let item = self.item_repo.find_by_path(validated_path.as_str()).await?;
        Ok(item.map(Self::to_dto))
    }

    /// Updates an item.
    pub async fn update(&self, id: i64, dto: UpdateItemDto) -> Result<(), DomainError> {
        let item = self
            .item_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::ItemNotFound(id.to_string()))?;

        let mut item = item;

        if let Some(path) = dto.path {
            let path = FilePath::new(&path)?;
            item.update_path(path);
        }

        if dto.size.is_some() {
            item.update_size(dto.size);
        }

        if dto.modified_time.is_some() {
            item.update_modified_time(dto.modified_time);
        }

        self.item_repo.update(&item).await
    }

    /// Deletes an item.
    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        self.item_repo.delete(id).await
    }

    /// Adds a tag to an item.
    pub async fn add_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError> {
        self.item_repo.add_tag(item_id, tag_id).await
    }

    /// Removes a tag from an item.
    pub async fn remove_tag(&self, item_id: i64, tag_id: i64) -> Result<(), DomainError> {
        self.item_repo.remove_tag(item_id, tag_id).await
    }

    /// Gets all tags for an item.
    pub async fn get_tags(&self, item_id: i64) -> Result<Vec<TagDto>, DomainError> {
        let tags = self.tag_repo.find_by_item(item_id).await?;
        Ok(tags.into_iter().map(TagDto::from).collect())
    }

    /// Replaces all tags for an item.
    pub async fn update_tags(&self, item_id: i64, tag_ids: Vec<i64>) -> Result<(), DomainError> {
        self.item_repo.replace_tags(item_id, tag_ids).await
    }

    fn to_dto(item: Item) -> ItemDto {
        ItemDto {
            id: item.id().unwrap_or(0),
            path: item.path().to_string(),
            is_directory: item.is_directory(),
            size: item.size(),
            modified_time: item.modified_time(),
            created_at: item.created_at().unwrap_or(0),
            updated_at: item.updated_at().unwrap_or(0),
        }
    }
}

impl From<crate::domain::entities::Tag> for TagDto {
    fn from(tag: crate::domain::entities::Tag) -> Self {
        TagDto {
            id: tag.id().unwrap_or(0),
            group_id: tag.group_id(),
            value: tag.value().to_string(),
            created_at: tag.created_at().unwrap_or(0),
            updated_at: tag.updated_at().unwrap_or(0),
        }
    }
}
