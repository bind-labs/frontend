use dioxus::prelude::*;
use ui::layout::{Column, Row};

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    title: String,
    source: String,
}

#[component]
pub fn Title(props: Props) -> Element {
    rsx! {
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
    }
}
