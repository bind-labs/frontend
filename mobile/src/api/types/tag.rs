use super::Icon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateTagRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateTagRequest {
    pub title: Option<String>,
    pub children_to_add: Option<Vec<TagChild>>,
    pub children_to_remove: Option<Vec<TagChild>>,
}

// ----------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagChild {
    #[serde(rename = "type")]
    pub type_: TagChildType,
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagChildType {
    Feed,
    Index,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserTag {
    pub id: i32,
    pub owner: i32,
    pub title: String,
    pub children: Vec<TagChild>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
