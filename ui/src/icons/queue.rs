use super::IconProps;
use dioxus::prelude::*;

#[component]
pub fn QueueIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "icon",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: props.size,
            height: props.size,
            fill: props.fill(),
            stroke: props.stroke(),
            stroke_width: 1.5,
            if props.solid {
                path { d: "M5.625 3.75a2.625 2.625 0 1 0 0 5.25h12.75a2.625 2.625 0 0 0 0-5.25H5.625ZM3.75 11.25a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75ZM3 15.75a.75.75 0 0 1 .75-.75h16.5a.75.75 0 0 1 0 1.5H3.75a.75.75 0 0 1-.75-.75ZM3.75 18.75a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75Z" }
            } else {
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z",
                }
            }
        }
    }
}
