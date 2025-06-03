use dioxus::prelude::*;
use ui::layout::Column;

#[component]
pub fn AuthContainer(gap: Option<String>, align: Option<String>, children: Element) -> Element {
    rsx! {
        Column {
            align: "stretch",

            height: "100%",
            width: "100%",
            max_width: "300px",
            margin: "auto",
            padding: "48px 16px 24px 16px",
            gap: gap.unwrap_or("32px".to_string()),

            {children}
        }
    }
}
