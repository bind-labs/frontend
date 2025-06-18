use super::components::FeedList;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Search() -> Element {
    rsx! {
        FeedList { num: 6 }
    }
}
