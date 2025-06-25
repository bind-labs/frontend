use super::IconProps;
use dioxus::prelude::*;

#[component]
pub fn SearchIcon(props: IconProps) -> Element {
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
                d: "m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z",
            }
        }
    }
}
