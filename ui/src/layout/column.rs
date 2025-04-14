use dioxus::prelude::*;

// ======================== Column Component ========================

/// Column properties.
///
/// Here you can find all properties that can be used with
/// [Column](crate::column::Column) component.
#[derive(Props, Clone, PartialEq)]
pub struct Props {
    pub body: Element,

    #[props(into)]
    pub background: Option<String>,

    #[props(into)]
    pub width: Option<String>,
    #[props(into)]
    pub min_width: Option<String>,
    #[props(into)]
    pub max_width: Option<String>,

    #[props(into)]
    pub height: Option<String>,
    #[props(into)]
    pub min_height: Option<String>,
    #[props(into)]
    pub max_height: Option<String>,

    #[props(into)]
    pub overflow: Option<String>,
    pub reverse: Option<bool>,
    pub wrap: Option<bool>,

    #[props(into)]
    pub align: Option<String>,
    #[props(into)]
    pub cross_align: Option<String>,

    #[props(into)]
    pub gap: Option<String>,
    #[props(into)]
    pub padding: Option<String>,
    #[props(into)]
    pub margin: Option<String>,

    #[props(into)]
    pub border: Option<String>,
    #[props(into)]
    pub border_top: Option<String>,
    #[props(into)]
    pub border_bottom: Option<String>,
    #[props(into)]
    pub border_left: Option<String>,
    #[props(into)]
    pub border_right: Option<String>,

    #[props(into)]
    pub shadow: Option<String>,
}

/// Column layout component.
///
/// # Usage
///
/// ```rust
/// use dioxus_layout::{Column, Align};
/// # use dioxus::prelude::*;
///
/// # fn MyComponent(cx: Scope) -> Element {
/// # cx.render(
/// rsx! {
///     Column {
///         align: "center",
///         wrap: true,
///         "Column children.."
///     }
/// }
/// # )
/// # }
/// ```
#[allow(non_snake_case)]
pub fn Column(props: Props) -> Element {
    rsx! {
        div {
            class: "dioxus-layout-column",
            display: "flex",
            flex_direction: match props.reverse {
                Some(true) => "column-reverse",
                _ => "column",
            },

            background: props.background,

            width: props.width,
            min_width: props.min_width,
            max_width: props.max_width,

            height: props.height,
            min_height: props.min_height,
            max_height: props.max_height,

            overflow: props.overflow,

            align_items: props.align,
            justify_content: props.cross_align,

            gap: props.gap,
            padding: props.padding,
            margin: props.margin,

            border: props.border,
            border_top: props.border_top,
            border_bottom: props.border_bottom,
            border_left: props.border_left,
            border_right: props.border_right,

            box_shadow: props.shadow,
            {props.body}
        }
    }
}
