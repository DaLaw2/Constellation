use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGroup {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub display_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub group_id: i64,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<i64>,
    pub modified_time: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_deleted: bool,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagTemplate {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}
