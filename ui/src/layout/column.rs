use super::{spacer::LayoutDirection, *};
use css_style::{
    box_align::*, flexbox::*, prelude::*, Background, Border, Display, FlexDirection, Gap, Margin,
    Padding,
};
use dioxus::prelude::*;

// ======================== Column Component ========================

/// Column properties.
///
/// Here you can find all properties that can be used with
/// [Column](crate::column::Column) component.
#[derive(Props, Clone, PartialEq)]
pub struct Props {
    pub body: Element,

    /// Control the length of the Column
    ///
    /// The default is `Length::from(1.0)`
    #[props(default = Some(Length::from(1.0)), strip_option, into)]
    pub length: Option<Length>,

    /// Control the minimum length of the Column
    ///
    /// The default is `Length::MinContent`
    #[props(default = Some(Length::MinContent), strip_option, into)]
    pub min_length: Option<Length>,

    /// Control the maximum length of the Column
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub max_length: Option<Length>,

    /// Control the width of the Column
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub width: Option<Length>,

    /// Control the minimum width of the Column
    ///
    /// The default is `Length::MinContent`
    #[props(default = Some(Length::MinContent), strip_option, into)]
    pub min_width: Option<Length>,

    /// Control the maximum width of the Column
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub max_width: Option<Length>,

    /// Expand factor used to expand this column in direction relevant to it's
    /// parent layout direction.
    ///
    /// When the parent is `Row` it will expand horizontally, when the parent is
    /// `Column` it will expand vertically.
    ///
    /// Note: This only works when this `Column` inside another layout (e.g. Row/Column).
    ///
    /// The default is `None`
    #[props(optional)]
    pub expand_by: Option<f32>,

    /// Shrink factor used to shrink this column in direction relevant to it's
    /// parent layout direction when needed.
    ///
    /// When the parent is `Row` it will shrink horizontally, when the parent is
    /// `Column` it will shrink vertically.
    ///
    /// Note: This only works when this `Column` inside another layout (e.g. Row/Column).
    ///
    /// The default is `None`
    #[props(optional)]
    pub shrink_by: Option<f32>,

    /// Make this layout inline
    ///
    /// The default is `false`
    #[props(default = false)]
    pub inline: bool,

    /// Reverse the order of the children
    ///
    /// The default is `false`
    #[props(default = false)]
    pub reverse: bool,

    /// Wrap into another column when there is no more vertical space.
    ///
    /// The default is `false`
    #[props(default = false)]
    pub wrap: bool,

    /// Align the children inside this column in main direction (horizontally).
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub align: Option<Align>,

    /// Align the children inside this column in the cross direction
    /// (vertically).
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub cross_align: Option<CrossAlign>,

    /// Align this column when it's inside another layout, the alignment
    /// direction is relevant to the parent layout direction
    ///
    /// When the parent is `Row` it will align horizontally, when the parent is
    /// `Column` it will align vertically.
    ///
    /// Note: This only works when this `Column` inside another layout (e.g. Row/Column).
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub align_self: Option<AlignSelf>,

    /// Gap between children.
    ///
    /// This take `Gap` value, which can take either one value that defines the
    /// gap for both the columns (if there is any) and rows, or two values one
    /// for rows and the other for columns.
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub gap: Option<Gap>,

    /// Reverse columns if there is more than one column within this `Column`.
    ///
    /// This only works when used with `wrap=true`.
    ///
    /// The default is `false`
    #[props(default = false)]
    pub reverse_columns: bool,

    /// Align columns in the cross direction (horizontally) if there is more
    /// than one column within this `Column`.
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub align_columns: Option<AlignColumns>,

    /// Overflow behavior for this Row
    ///
    /// By default any child that get oversized will be visible and may overlap
    /// with other UI components. Change this property if you like to make the
    /// content scrollable or make the oversized hidden/cliped.
    ///
    /// The default is `Overflow::visible()`
    #[props(default = Overflow::visible(), into)]
    pub overflow: Overflow,

    /// Padding for the `Column`
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub padding: Option<Padding>,

    /// Margin for the `Column`
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub margin: Option<Margin>,

    /// Background for the `Column`
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub background: Option<Background>,

    /// Border for the `Column`
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub border: Option<Border>,

    /// Shadow for the `Column`
    ///
    /// The default is `None`
    #[props(optional, into)]
    pub shadow: Option<Shadow>,
}

/// Column layout component.
///
/// See [column properties docs](crate::column::Props) for more details on
/// how to use them.
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
///         align: Align::Center,
///         wrap: true,
///         "Column children.."
///     }
/// }
/// # )
/// # }
/// ```
#[allow(non_snake_case)]
pub fn Column(props: Props) -> Element {
    use_context_provider(|| LayoutDirection::Column);
    let style = style()
        .display(match props.inline {
            true => Display::InlineFlex,
            false => Display::Flex,
        })
        .and_size(|s| {
            s.try_width(props.width.clone())
                .try_min_width(props.min_width.clone())
                .try_max_width(props.max_width.clone())
                .try_height(props.length.clone())
                .try_min_height(props.min_length.clone())
                .try_max_height(props.max_length.clone())
        })
        .flex_direction(match props.reverse {
            true => FlexDirection::ColumnReverse,
            false => FlexDirection::Column,
        })
        .try_flex_wrap(match (props.wrap, props.reverse_columns) {
            (true, true) => Some(Wrap::WrapReverse),
            (true, false) => Some(Wrap::Wrap),
            _ => None,
        })
        .try_justify_content(props.align)
        .try_align_items(props.cross_align)
        .try_align_content(props.align_columns)
        .try_align_self(props.align_self)
        .try_flex_grow(props.expand_by)
        .try_flex_shrink(props.shrink_by)
        .try_gap(props.gap.clone())
        .try_padding(props.padding.clone())
        .try_margin(props.margin.clone())
        .try_background(props.background.clone())
        .try_border(props.border.clone())
        .try_box_shadow(props.shadow.clone())
        .insert("box-sizing", "border-box")
        .merge(props.overflow)
        .to_string();

    rsx! {
        div {
            class: "dioxus-layout-column",
            style: "{style}",
            {props.body}
        }
    }
}
