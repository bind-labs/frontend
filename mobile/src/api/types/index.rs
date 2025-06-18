use super::{Icon, SortOrder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateIndexRequest {
    query: String,
    sort: SortOrder,
    title: String,
    description: Option<String>,
    icon: Icon,
}

#[derive(Debug, Serialize)]
pub struct UpdateIndexRequest {
    query: Option<String>,
    sort: Option<SortOrder>,
    title: Option<String>,
    description: Option<String>,
    icon: Option<Icon>,
}

// ----------

/// Represents an index created by a user.
/// An **Index** can be thought of as a custom search over a certain set of feeds.
#[derive(Clone, Debug, Deserialize)]
pub struct UserIndex {
    pub id: i32,
    pub owner: i32,

    pub query: String,
    pub sort: String,

    pub title: String,
    pub description: Option<String>,
    pub icon: Icon,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
