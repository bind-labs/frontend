use super::components::document::ReaderDocument;
use super::components::header::Header;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn FeedReader() -> Element {
    rsx! {

        Header {
            title: "Is it Lunacy to Put a Data Center on the Moon?",
            source: "Ieee Spectrum",
            author: "Dina Genkina",
            published_at: "26 Feb 2025",
            duration: "4 min read",
        }
        ReaderDocument {}
    }
}
