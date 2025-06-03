use dioxus::prelude::*;

mod container;
mod header;

pub use container::AuthContainer;
pub use header::Header;

#[component]
pub fn Error(error: Signal<Option<String>>) -> Element {
    if let Some(error) = error() {
        return rsx! {
            span { align_self: "center", color: "var(--text-error)", text_align: "center",
                "{error}"
            }
        };
    }

    rsx! {}
}
