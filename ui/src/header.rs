use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    subtitle: String,
    title: String,
}

#[allow(non_snake_case)]
#[component]
pub fn Header(props: Props) -> Element {
    rsx! {
      div {
        display: "flex",
        align_items: "center",
        justify_content: "center",
          h2 {
            font_size: 30,
            line_height: 40,
            {props.subtitle}
          }

          h1 {
            font_size: 40,
            line_height: 50,
            {props.title}
          }
      }
    }
}
