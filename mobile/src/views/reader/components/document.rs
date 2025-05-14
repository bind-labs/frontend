use dioxus::prelude::*;

#[component]
pub fn ReaderDocument() -> Element {
    rsx! {
        div { class: "reader-document", "Reader Document" }
    }
}

#[component]
pub fn ReaderDocumentHtml() -> Element {
    rsx! {
        div { class: "reader-document-html", "Reader Document HTML" }
    }
}
