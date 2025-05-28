use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateFeedRequest {
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedItemEnclosure {
    /// URL of the media file
    pub url: String,
    /// Size of the media file in bytes
    pub length: i32,
    /// MIME type of the media file
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeedFormat {
    /// Atom feed format
    Atom,
    /// RSS feed format
    Rss,
    /// JSON feed format
    Json,
}

impl FeedFormat {
    pub fn from_content_type(content_type: &str) -> Option<Self> {
        match content_type {
            "application/rss+xml" | "application/rss" | "text/xml" | "text/rss+xml" => {
                Some(Self::Rss)
            }
            "application/atom+xml" | "applcation/atom" | "text/atom+xml" | "text/atom" => {
                Some(Self::Atom)
            }
            "application/json" | "text/json" => Some(Self::Json),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeedStatus {
    /// Feed is active and being updated regularly
    Active,
    /// Feed is completed (e.g., a podcast series that has ended)
    Completed,
    /// Feed is temporarily suspended
    Suspended,
    /// Feed has broken links or cannot be fetched
    Broken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    /// Unique identifier of the feed
    pub id: i32,
    /// Current status of the feed
    pub status: FeedStatus,
    /// Format of the feed (RSS, Atom, JSON)
    pub format: FeedFormat,
    /// URL of the feed
    pub link: String,
    /// Domain of the feed (extracted from the URL)
    pub domain: Option<String>,
    /// Title of the feed
    pub title: String,
    /// Description of the feed
    pub description: String,
    /// URL to the feed's icon
    pub icon: Option<String>,
    /// Language of the feed (e.g., "en-us")
    pub language: Option<String>,
    /// Hours when the feed should not be fetched
    pub skip_hours: Vec<i32>,
    /// Days of the week when the feed should not be fetched
    pub skip_days_of_week: Vec<i32>,
    /// Minimum time to cache the feed for
    pub ttl_in_minutes: Option<i32>,
    /// ETag header from the last update
    pub etag: Option<String>,
    /// When the feed was created in the system
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Time of the last update to the content
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Time of the last fetch
    pub fetched_at: chrono::DateTime<chrono::Utc>,
    /// Time of the last successful fetch
    pub successful_fetch_at: chrono::DateTime<chrono::Utc>,
    /// Time to fetch the feed next
    pub next_fetch_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedItem {
    /// Unique identifier of the feed item
    pub id: i64,
    /// Globally unique identifier of the item (from the feed)
    pub guid: String,
    /// ID of the feed this item belongs to
    pub feed_id: i32,
    /// Position of the item in the feed
    pub index_in_feed: i32,
    /// Title of the feed item
    pub title: String,
    /// Link to the full article
    pub link: Option<String>,
    /// Summary or description of the item
    pub description: Option<String>,
    /// Attached media file (if any)
    pub enclosure: Option<FeedItemEnclosure>,
    /// Categories or tags for the item
    pub categories: Vec<String>,
    /// Link to the comments section
    pub comments_link: Option<String>,
    /// When the item was published
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Full content of the item
    pub content: Option<String>,
    /// MIME type of the content
    pub content_type: Option<String>,
    /// Base URL for relative links in the content
    pub base_link: Option<String>,
    /// When the item was created in the system
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the item was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
