use dioxus::prelude::*;
use ui::layout::{Column, Row};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    author: Option<String>,
    published_at: Option<String>,
    duration: String,
}

#[component]
pub fn Byline(props: Props) -> Element {
    rsx! {
        Column { gap: "4px",
            if let Some(author) = props.author {
                address {
                    class: "author",
                    font_family: "IBM Plex Mono",
                    text_transform: "uppercase",
                    font_size: "12px",
                    "By "
                    a {
                        rel: "author",
                        // href: "https://www.example.com",
                        font_style: "normal",
                        font_family: "IBM Plex Mono",
                        text_decoration: "underline",
                        {author}
                    }
                }
            }
            Row { gap: "8px",
                if let Some(published_at) = props.published_at {
                    time {
                        font_family: "IBM Plex Mono",
                        text_transform: "uppercase",
                        font_size: "12px",
                        {published_at}
                    }
                    span {
                        class: "separator",
                        font_family: "IBM Plex Mono",
                        font_size: "12px",
                        "|"
                    }
                }
                span {
                    class: "duration",
                    font_family: "IBM Plex Mono",
                    text_transform: "uppercase",
                    font_size: "12px",
                    {props.duration}
                }
            }
        }
    }
}
