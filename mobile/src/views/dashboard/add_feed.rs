use dioxus::prelude::*;

use crate::views::dashboard::components::Header;

#[component]
pub fn AddFeed() -> Element {
    rsx! {
        Header { title: "Add New Feed" }

        div {
            "Add Feed"
        }
    }
}
