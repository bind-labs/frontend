use dioxus::prelude::*;

use super::components::FeedItemList;
use crate::{hooks::use_token, views::dashboard::components::Header};

#[component]
pub fn List(id: usize) -> Element {
    let mut token = use_token();

    rsx! {
        Header {
            title: "Read Later",
            additional: "(16 items)",
            onsettings: move |_| {
                tracing::info!("Settings clicked");
                token.set(None);
            },
        }

        FeedItemList { num: 0 }
    }
}
