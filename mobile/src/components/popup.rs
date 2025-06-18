use dioxus::{document::document, prelude::*};

use crate::views::Route;

#[derive(Debug, Clone)]
pub enum PopupState {
    Open(Element),
    Close,
}

pub fn use_popup_state_provider() -> Signal<PopupState> {
    use_context_provider(|| Signal::new(PopupState::Close))
}

pub fn use_popup_state() -> Signal<PopupState> {
    use_context::<Signal<PopupState>>()
}

#[component]
pub fn Popup() -> Element {
    let mut state = use_popup_state();

    // Close popup when route changes
    let route = use_route::<Route>();
    let mut previous_route = use_signal(|| route.clone());
    if route.clone() != previous_route() {
        state.set(PopupState::Close);
        previous_route.set(route.clone());
    }

    match state() {
        PopupState::Open(element) => {
            rsx! {
                dialog {
                    id: "popup",
                    onclick: |ev| ev.stop_propagation(),
                    {element}
                }
            }
        }
        PopupState::Close => rsx! {},
    }
}
