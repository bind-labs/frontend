pub struct FeedItem {
    /// Unique identifier of the feed item
    #[ormx(default)]
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
    #[schema(format = "date-time")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Full content of the item
    pub content: Option<String>,
    /// MIME type of the content
    pub content_type: Option<String>,
    /// Base URL for relative links in the content
    pub base_link: Option<String>,
    /// When the item was created in the system
    #[schema(format = "date-time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the item was last updated
    #[schema(format = "date-time")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
