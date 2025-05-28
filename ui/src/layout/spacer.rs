use dioxus::prelude::*;

// ======================== Column Component ========================

/// Spacer properties.
///
/// Defaults to horizontal
/// [Spacer](crate::spacer::Spacer) component.
#[derive(Props, Clone, PartialEq)]
pub struct Props {
    /// Constant length for this spacer. By default, the spacer will take all available space.
    #[props(into)]
    pub size: Option<String>,

    /// Maximum length for this spacer. By default, the spacer will take all available space.
    #[props(into)]
    pub max_size: Option<String>,
}

/// Spacer layout component. Defaults to horizontal
#[allow(non_snake_case)]
pub fn Spacer(props: Props) -> Element {
    let style = match (props.size, props.max_size) {
        (Some(size), _) => format!("width: {size}"),
        (_, Some(max_size)) => format!("max-width: {max_size}"),
        _ => "".to_string(),
    };

    rsx!(div {
        align_self: "stretch",
        flex_grow: 1.0,
        flex_shrink: 1.0,
        style: "{style}",
    })
}
