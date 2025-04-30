use dioxus::{document::document, mobile::window, prelude::*};

#[component]
pub fn FixedSizeContainer(children: Element) -> Element {
    rsx! {
        div {
            class: "fixed-size-container",
            style: "height: 100vh; width: 100vw;",
            {children}
        }
    }
}
