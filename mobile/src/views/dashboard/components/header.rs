use dioxus::prelude::*;
use ui::forms::button::UnstyledButton;
use ui::icons::Cog6Tooth;
use ui::layout::*;

use crate::views::Route;

#[component]
pub fn Header(
    title: String,
    additional: Option<String>,
    onsettings: Option<EventHandler>,
) -> Element {
    rsx! {
        header {
            display: "flex",
            padding: "0px 16px",
            justify_content: "space-between",
            align_items: "center",
            border_bottom: "1px solid var(--text)",

            Row { gap: "8px", cross_align: "baseline", padding: "14px 0px",
                span { font_size: "18px", font_weight: 600, "{title}" }
                {additional.map(|additional| rsx! {
                    span { font_size: "14px", color: "#666666", "{additional}" }
                })}
            }

            {onsettings.map(|onsettings| rsx! { UnstyledButton { onclick: move |_| { onsettings.call(()); }, padding: "14px 0px",
                Cog6Tooth {}
            } })}
        }
    }
}
