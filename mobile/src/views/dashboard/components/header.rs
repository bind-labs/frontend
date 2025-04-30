use dioxus::prelude::*;
use ui::icons::Cog6Tooth;
use ui::layout::*;

#[allow(non_snake_case)]
#[component]
pub fn Header(title: String, additional: Option<String>, onsettings: EventHandler<()>) -> Element {
    rsx! {
        header {
            display: "flex",
            padding: "14px 16px",
            justify_content: "space-between",
            align_items: "center",
            border_bottom: "1px solid black",

            Row { gap: "8px", cross_align: "baseline",
                span { font_size: "18px", font_weight: 600, "{title}" }
                {additional.map(|additional| rsx! {
                    span { font_size: "14px", color: "#666666", "{additional}" }
                })}
            }

            Cog6Tooth {}
        }
    }
}
