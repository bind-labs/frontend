use dioxus::prelude::*;
use ui::forms::button::UnstyledButton;

use crate::views::dashboard::Route;

#[component]
pub fn NavbarButton<Route: Routable + PartialEq + Copy>(
    icon: fn(active: bool) -> Element,
    to: Route,
) -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        UnstyledButton {
            display: "flex",
            align_items: "center",
            justify_content: "center",

            flex_grow: "1",
            padding: "0px 12px",

            onclick: move |_| { navigator().push(to); },

            {icon(current_route == to)}
        }
    }
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav {
            id: "navbar",
            display: "flex",
            border_top: "1px solid #000000",
            padding: "16px 0px",
            {children}
        }
    }
}
