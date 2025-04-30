use dioxus::{prelude::*};
use ui::layout::{Column, Row};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    title: String,
    source: String,
    author: Option<String>,
    published_at: Option<String>,
    duration: String,
}

#[component]
pub fn Header(props: Props) -> Element {
    rsx! {
        Column { align: "flex-start", cross_align: "center", gap: "12px",
            Column { gap: "4px",
                h3 {
                    font_size: "12px",
                    font_weight: "500",
                    line_height: "14px",
                    font_family: "IBM Plex Mono",
                    margin: "0px",
                    text_transform: "uppercase",
                    {props.source}
                }

                h1 {
                    font_size: "28px",
                    font_weight: "600",
                    line_height: "32px",

                    margin: "0px",

                    {props.title}
                }
            }


            Column {
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
                Row { gap: "8px", align: "center",
                    if let Some(published_at) = props.published_at {
                        time {
                            font_family: "IBM Plex Mono",
                            text_transform: "uppercase",
                            font_size: "12px",
                            {published_at}
                        }
                    }
                    span {
                        class: "separator",
                        font_family: "IBM Plex Mono",
                        font_size: "12px",
                        "|"
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
}
