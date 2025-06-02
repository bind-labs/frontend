use dioxus::prelude::*;
use ui::layout::Column;

use super::components::{Byline, ReaderDocumentHtml, Title};

#[component]
pub fn ParsedReader() -> Element {
    rsx! {
        Column { cross_align: "center", gap: "12px",
            Title {
                title: "Is it Lunacy to Put a Data Center on the Moon?",
                source: "Ieee Spectrum",
            }
            Byline {
                author: "Dina Genkina",
                published_at: "26 Feb 2025",
                duration: "4 min read",
            }
            ReaderDocumentHtml {}
        }
    }
}
