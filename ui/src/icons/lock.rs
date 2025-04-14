use super::IconProps;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn LockIcon(props: IconProps) -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24", width: props.size, height: props.size, fill: "none", stroke: props.color, stroke_width: 1.5,
            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" }
        }
    }
}
