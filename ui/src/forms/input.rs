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
    #[props(default = false)]
    password: bool,
    onchange: Callback<String>,
}

#[allow(non_snake_case)]
pub fn Input(props: Props) -> Element {
    let title = props.title.map(|title| -> Element {
        rsx! {
            label {
                position: "absolute",

                top: "0",
                transform: "translateY(-50%)",

                left: "14px",
                padding_left: "2px",
                padding_right: "2px",

                z_index: "1",

                background: "var(--bg)",

                {title}
            }
        }
    });

    let icon = props.icon.map(|icon| -> Element {
        rsx! {
            div {
                position: "absolute",

                top: "50%",
                // TODO: why +2px?
                transform: "translateY(calc(-50% + 2px))",

                left: match props.icon_position {
                    Some(IconPosition::Left) => "16px",
                    None | Some(IconPosition::Right) => "unset",
                },
                right: match props.icon_position {
                    Some(IconPosition::Left) => "unset",
                    None | Some(IconPosition::Right) => "16px",
                },

                {icon}
            }
        }
    });

    rsx! {
      div {
        display: "flex",
        position: "relative",

        {title}
        {icon}

        input {
          padding: "12px",
          padding_left: match props.icon_position {
            Some(IconPosition::Left) => "calc(16px + 24px + 12px)",
            _ => "16px",
          },
          padding_right: match props.icon_position {
            Some(IconPosition::Right) => "calc(16px + 24px + 12px)",
            _ => "16px",
          },
          flex_grow: "1",
          height: "48px",
          background_color: "transparent",
          color: "#000000",
          border: "1px solid black",
          outline: "none",
          onchange: move |ev| props.onchange.call(ev.value().clone()),
          ..props.attributes
        }
      }
    }
}
