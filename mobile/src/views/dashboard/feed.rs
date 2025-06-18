use super::components::FeedList;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Feed() -> Element {
    rsx! {
        FeedList { num: 1 }
    }
}
