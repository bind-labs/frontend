use super::SortOrder;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    query: String,
    sort: SortOrder,
}
