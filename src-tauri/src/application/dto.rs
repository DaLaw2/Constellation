//! Data Transfer Objects
//!
//! DTOs are used to transfer data between layers. They are serializable
//! and designed for external communication (e.g., Tauri commands).

use serde::{Deserialize, Serialize};

/// DTO for Item data transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDto {
    pub id: i64,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<i64>,
    pub modified_time: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// DTO for creating a new item.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateItemDto {
    pub path: String,
    pub is_directory: bool,
    pub size: Option<i64>,
    pub modified_time: Option<i64>,
}

/// DTO for updating an item.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateItemDto {
    pub path: Option<String>,
    pub size: Option<i64>,
    pub modified_time: Option<i64>,
}

/// DTO for TagGroup data transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGroupDto {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub display_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// DTO for creating a new tag group.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTagGroupDto {
    pub name: String,
    pub color: Option<String>,
}

/// DTO for updating a tag group.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTagGroupDto {
    pub name: Option<String>,
    pub color: Option<String>,
}

/// DTO for Tag data transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub id: i64,
    pub group_id: i64,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// DTO for creating a new tag.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTagDto {
    pub group_id: i64,
    pub value: String,
}

/// DTO for updating a tag.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTagDto {
    pub value: Option<String>,
}

/// DTO for TagTemplate data transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagTemplateDto {
    pub id: i64,
    pub name: String,
    pub tag_ids: Vec<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// DTO for creating a new tag template.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTagTemplateDto {
    pub name: String,
    pub tag_ids: Vec<i64>,
}

/// DTO for updating a tag template.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTagTemplateDto {
    pub name: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}

/// Search mode for tag-based queries.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    And,
    Or,
}

/// DTO for search criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteriaDto {
    pub tag_ids: Vec<i64>,
    pub mode: SearchMode,
    pub filename_query: Option<String>,
}

/// DTO for Search History data transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryDto {
    pub id: i64,
    pub criteria: SearchCriteriaDto, // Reuse SearchCriteriaDto or define similar output structure
    pub last_used_at: i64,
}
