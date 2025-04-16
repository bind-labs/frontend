use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    pub children: Element,

    #[props(into)]
    pub position: Option<String>,
    #[props(into)]
    pub top: Option<String>,
    #[props(into)]
    pub right: Option<String>,
    #[props(into)]
    pub bottom: Option<String>,
    #[props(into)]
    pub left: Option<String>,

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

/// Row layout component.
///
/// # Usage
///
/// ```rust
/// use ui::layout::{Row, Align};
/// # use dioxus::prelude::*;
///
/// # fn MyComponent() -> Element {
/// rsx! {
///     Row {
///         align: "center",
///         wrap: true,
///         "Row children.."
///     }
/// }
/// # }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Row(props: Props) -> Element {
    rsx! {
        div {
            class: "dioxus-layout-row",
            display: "flex",
            flex_direction: match props.reverse {
                Some(true) => "row-reverse",
                _ => "row",
            },

            position: props.position,
            top: props.top,
            right: props.right,
            bottom: props.bottom,
            left: props.left,

            background: props.background,

            width: props.width,
            min_width: props.min_width,
            max_width: props.max_width,

            height: props.height,
            min_height: props.min_height,
            max_height: props.max_height,

            overflow: props.overflow,

            justify_content: props.align,
            align_items: props.cross_align,

            gap: props.gap,
            padding: props.padding,
            margin: props.margin,

            border: props.border,
            border_top: props.border_top,
            border_bottom: props.border_bottom,
            border_left: props.border_left,
            border_right: props.border_right,

            box_shadow: props.shadow,

            {props.children}
        }
    }
}
