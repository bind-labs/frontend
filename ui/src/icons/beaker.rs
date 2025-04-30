use super::IconProps;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn BeakerIcon(props: IconProps) -> Element {
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
                d: "M9.75 3.104v5.714a2.25 2.25 0 0 1-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 0 1 4.5 0m0 0v5.714a2.25 2.25 0 0 0 .659 1.591L19.5 14.5M14.25 3.104c.251.023.501.05.75.082M19.5 14.5v-2.25a2.25 2.25 0 0 0-2.25-2.25h-15A2.25 2.25 0 0 0 0 12.25v2.25m19.5 0a2.25 2.25 0 0 1-2.25 2.25H6.75A2.25 2.25 0 0 1 4.5 16.5m15 0-2-1.176a3 3 0 0 0-2.872 0L12 16.5m7.5 0-2.024-.993a2.5 2.5 0 0 0-2.349.013L12 16.5m-5.5 0-2-1.176a3 3 0 0 0-2.872 0L0 16.5m10 0-2-1.176a3 3 0 0 0-2.173-.132C6.29 15.323 6.76 16.5 8 16.5c.308 0 .995-.114 2-1.176Z",
            }
        }
    }
}
