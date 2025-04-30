use super::IconProps;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn PlusIcon(props: IconProps) -> Element {
    rsx! {
        svg {
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
                d: "M12 4.5v15m7.5-7.5h-15",
            }
        }
    }
}
