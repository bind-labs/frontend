use dioxus::prelude::*;
use ui::layout::Column;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    subtitle: String,
    title: String,
}

#[component]
pub fn Header(props: Props) -> Element {
    rsx! {
        Column {
            align: "center",
            cross_align: "center",
            margin: "0px 0px 24px 0px",

            h2 {
                font_size: "30px",
                font_weight: "500",
                line_height: "40px",

                margin: "0px",

                {props.subtitle}
            }

            h1 {
                font_size: "64px",
                font_weight: "600",
                font_style: "italic",
                line_height: "64px",

                margin: "0px",

                {props.title}
            }
        }
    }
}
