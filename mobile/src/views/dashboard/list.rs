use super::components::FeedList;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn List() -> Element {
    rsx! {
        FeedList {}
    }
}
