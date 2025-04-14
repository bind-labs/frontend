use super::IconProps;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn QueueIcon(props: IconProps) -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", width: props.size, height: props.size, fill: "none", stroke: props.color, stroke_width: 1.5,
            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z" }
        }
    }
}
