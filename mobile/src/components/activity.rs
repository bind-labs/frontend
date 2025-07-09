use dioxus::prelude::*;
use ui::layout::Column;

#[derive(Props, Clone, PartialEq)]
pub struct ActivityGraphProps {
    pub points: Vec<f32>,

    #[props(default = 48)]
    pub width: usize,
    #[props(default = 0.07)]
    pub stroke_width: f32,
    #[props(default = "var(--text)", into)]
    pub color: String,
}

#[component]
pub fn ActivityGraph(props: ActivityGraphProps) -> Element {
    let ActivityGraphProps {
        points,
        width,
        stroke_width,
        color,
    } = props;

    let width_to_height_ratio = 3.; // 3x wider than tall
    let height = (width as f32 / width_to_height_ratio) as usize;
    let view_box = format!("0 0 {} {}", width_to_height_ratio, 1. + stroke_width);

    let path_data = points
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let ident = if i == 0 { "M" } else { "L" };
            let x = ((i as f32) / ((points.len() - 1) as f32) * width_to_height_ratio);
            // invert y axis since svg goes from top to bottom
            // add half the stroke width to avoid cut off at top/bottom
            let y = 1.0 - point + stroke_width / 2.;
            format!("{ident}{:.2},{:.2}", x, y)
        })
        .collect::<Vec<String>>()
        .join(" ");

    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", width: "{width}", height: "{height}", view_box,
            path { stroke: "{color}", stroke_width: "{stroke_width}", fill: "none", d: path_data }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ActivityProps {
    pub points: Vec<f32>,
    pub per_month: usize,

    #[props(default = 48)]
    pub width: usize,
    #[props(default = 0.07)]
    pub stroke_width: f32,
    #[props(default = "var(--text)", into)]
    pub color: String,

    #[props(default = false)]
    pub horizontal: bool,
}

#[component]
pub fn Activity(props: ActivityProps) -> Element {
    let ActivityProps {
        points,
        width,
        stroke_width,
        color,
        per_month,
        horizontal,
    } = props;

    rsx! {
        div { display: "flex", gap: "6px", flex_direction: if horizontal { "row" } else { "column" }, justify_content: "center", align_items: "center",
            ActivityGraph { points, width, stroke_width, color }
            span { font_size: "12px", "{per_month} / mo" }
        }
    }
}
