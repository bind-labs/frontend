use super::*;
use css_style::{prelude::*, AlignSelf};
use dioxus::prelude::*;

// ======================== Column Component ========================

/// Spacer properties.
///
/// Here you can find all properties that can be used with
/// [Spacer](crate::spacer::Spacer) component.
#[derive(Props, Clone, PartialEq)]
pub struct Props {
    /// Minimum length for this spacer.
    ///
    /// By default the spacer will take the available space, this property can
    /// only control minimum length for the spacer.
    #[props(optional, into)]
    pub min: Option<Length>,
}

#[allow(non_snake_case)]
pub fn Spacer(props: Props) -> Element {
    let layout_direction = use_context::<Option<LayoutDirection>>();
    let min = props.min;
    let style = style()
        .align_self(AlignSelf::Stretch)
        .flex_grow(1.0)
        .flex_shrink(1.0)
        .and_size(|s| match layout_direction {
            Some(LayoutDirection::Column) => s.max_height(1.0).try_min_height(min.clone()),
            Some(LayoutDirection::Row) => s.max_width(1.0).try_min_width(min.clone()),
            None => s
                .max_width(1.0)
                .max_height(1.0)
                .try_min_height(min.clone())
                .try_min_width(min.clone()),
        });
    rsx!(div { style: "{style}" })
}

// ======================

#[derive(Debug, Clone)]
pub(crate) enum LayoutDirection {
    Row,
    Column,
}
