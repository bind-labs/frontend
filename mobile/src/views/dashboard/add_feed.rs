use dioxus::prelude::*;
use ui::{
    forms::input::{IconPosition, Input},
    icons::SearchIcon,
};

use crate::views::dashboard::components::{FeedList, Header};

#[component]
pub fn AddFeed() -> Element {
    let mut query = use_signal(String::new);

    rsx! {
        Header { title: "Add New Feed" }

        if query().is_empty() {
            FeedList { num: 16 }
        } else {
            FeedList { num: 0 }
        }

        section { padding: "24px 16px", border_top: "1px solid var(--text)",
            Input {
                icon: rsx! { SearchIcon {} },
                icon_position: IconPosition::Left,
                placeholder: "Search for a name, topic, or URL",
                value: query(),
                onchange: move |value| query.set(value),
            }
        }
    }
}
