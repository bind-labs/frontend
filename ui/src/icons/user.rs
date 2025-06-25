use super::IconProps;
use dioxus::prelude::*;

#[component]
pub fn UserIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: props.size,
            height: props.size,
            fill: "none",
            stroke: props.color,
            stroke_width: 1.5,
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z",
            }
        }
    }
}
