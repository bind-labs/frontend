use dioxus::prelude::*;

use super::components::FeedItemList;
use crate::views::dashboard::components::Header;

#[allow(non_snake_case)]
pub fn Search() -> Element {
    rsx! {
        Header { title: "Search" }
        FeedItemList { num: 6 }
    }
}
