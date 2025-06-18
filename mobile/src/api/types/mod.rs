use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod feed;
pub mod index;
pub mod list;
pub mod search;
pub mod tag;
pub mod user;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Icon {
    pub icon: String,
    pub color: String,
}

impl Icon {
    pub fn get_random_icon() -> Self {
        let mut rng = rand::rng();

        // Pick a random emoji from the Unicode range
        let emoji_code = rng.random_range(0x1F300..=0x1F64F);
        let icon = char::from_u32(emoji_code).unwrap_or('❓').to_string();

        // Generate a random hex color
        let color = format!("#{:06X}", rng.random_range(0x000000..=0xFFFFFF));

        Self { icon, color }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SortOrder {
    RecentlyUpdated,
    AsIs,
}

impl From<&str> for SortOrder {
    fn from(s: &str) -> Self {
        match s {
            "recently_updated" => SortOrder::RecentlyUpdated,
            "as_is" => SortOrder::AsIs,
            _ => SortOrder::RecentlyUpdated,
        }
    }
}

impl From<String> for SortOrder {
    fn from(s: String) -> Self {
        SortOrder::from(s.as_str())
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SortOrder::RecentlyUpdated => "recently_updated".to_string(),
            SortOrder::AsIs => "as_is".to_string(),
        };
        write!(formatter, "{}", s)
    }
}
