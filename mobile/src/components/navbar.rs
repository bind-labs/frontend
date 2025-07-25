use dioxus::{html::u::z_index, prelude::*};
use ui::forms::button::UnstyledButton;

use crate::views::Route;

#[component]
pub fn NavbarButton<Route: Routable + PartialEq + Clone>(
    icon: fn(active: bool) -> Element,
    to: Route,
) -> Element {
    let current_route = use_route::<Route>();
    let to_clone = to.clone();

    rsx! {
        UnstyledButton {
            display: "flex",
            align_items: "center",
            justify_content: "center",

            flex_grow: "1",
            padding: "0px 12px",

            onclick: move |_| { navigator().push(to_clone.clone()); },

            {icon(current_route == to)}
        }
    }
}

#[component]
pub fn NavbarButtonWithoutRoute(
    children: Element,
    onclick: EventHandler<MouseEvent>,
    onmounted: Option<EventHandler<MountedEvent>>,
) -> Element {
    rsx! {
        UnstyledButton {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            flex_grow: "1",
            padding: "0px 12px",
            onclick,
            {children}
        }
    }
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav {
            id: "navbar",
            {children}
        }
    }
}
