use dioxus::prelude::*;

use super::components::FeedItemList;
use crate::{hooks::use_token, views::dashboard::components::Header};

#[component]
pub fn Feed() -> Element {
    let mut token = use_token();

    rsx! {
        Header {
            title: "Some Feed",
            additional: "(Updated 12 mins ago)",
            onsettings: move |_| {
                tracing::info!("Settings clicked");
                token.set(None);
            },
        }

        FeedItemList { num: 1 }
    }
}
