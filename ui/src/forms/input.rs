use dioxus::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum IconPosition {
    Left,
    Right,
}

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    #[props(extends = GlobalAttributes, extends = input)]
    attributes: Vec<Attribute>,
    title: Option<String>,
    icon: Option<Element>,
    icon_position: Option<IconPosition>,
    password: bool,
    onchange: Callback<String>,
}

#[allow(non_snake_case)]
#[component]
pub fn Input(props: Props) -> Element {
    let icon_div_style = match props.icon_position {
        Some(IconPosition::Left) => "left: 16px;",
        Some(IconPosition::Right) => "right: 16px;",
        _ => "",
    };

    rsx! {
      div {
        position: "relative",
        {
          if props.title.is_some() {
            rsx! {
              label {
                style: "padding_left: 2px; padding_right: 2px; position: absolute; top: 0; left: 14px; transform: translateY(-9px); z_index: 1;",
                {props.title}
              }
            }
          } else {
            return rsx!();
          }
        },
        {
          props.icon.is_some().then(|| rsx! {
            div {
              position: "absolute",
              style: "{icon_div_style} top: 50%; transform: translateY(-12px);",
              {props.icon}
            }
          })

        },
        {
          rsx! {
            input {
              padding_left: match props.icon_position {
                Some(IconPosition::Left) => 16 + 24 + 12,
                _ => 16,
              },
              padding_right: match props.icon_position {
                Some(IconPosition::Right) => 16 + 24 + 12,
                _ => 16,
              },
              height: 48,
              background_color: "transparent",
              color: "#000000",
              border_radius: 0,
              border_width: 1,
              border_color: "#000000",
              onchange: move |ev| props.onchange.call(ev.value().clone()),
              ..props.attributes
            }
          }
        }
      }
    }
}
