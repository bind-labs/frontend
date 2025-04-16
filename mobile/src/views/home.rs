use dioxus::prelude::*;
use ui::{layout::Row, Hero};

use crate::components::feed_list::FeedList;

#[component]
pub fn Home() -> Element {
    rsx! {
        FeedList { }
    }
}
