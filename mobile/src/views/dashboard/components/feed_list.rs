use dioxus::prelude::*;
use reqwest::Url;
use ui::layout::*;

use crate::components::activity::Activity;

#[derive(Props, Clone, PartialEq)]
pub struct FeedListItemProps {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub link: Url,
}

#[component]
pub fn FeedListItem(props: FeedListItemProps) -> Element {
    let FeedListItemProps {
        title,
        description,
        image_url,
        link,
    } = props;

    let link_pretty = format!(
        "{} > {}",
        link.host_str().unwrap_or_default().replace("www.", ""),
        link.path_segments()
            .map(|segments| segments.collect::<Vec<&str>>().join(" > "))
            .unwrap_or_default()
    );

    rsx! {
        div { display: "grid", grid_template_rows: "auto auto", row_gap: "8px", grid_template_columns: "auto auto", column_gap: "8px",
            padding: "8px 16px",
            border_bottom: "1px solid var(--text-secondary)",

            img { src: image_url, max_width: "48px", max_height: "48px", width: "100%", align_self: "center" }
            Column {
                span { font_size: "18px",
                    "{title}"
                }
                span { font_size: "14px", color: "var(--text-secondary)",
                    "{link_pretty}"
                }
            }
            Activity { points: vec![0.5, 0.8, 0., 0.5, 0.4, 1., 0.1], per_month: 24 }
            span { font_size: "14px", text_overflow: "ellipsis", overflow: "hidden", style: "display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 2;",
                "{description}"
            }
        }
    }
}

#[component]
pub fn FeedList(num: usize) -> Element {
    rsx! {
        Column {
            for i in 0..num {
                FeedListItem {
                    key: "{i}",
                    image_url: "https://www.nasa.gov/wp-content/themes/nasa/assets/images/nasa-logo@2x.png",
                    title: "NASA",
                    link: Url::parse("https://www.nasa.gov/rss/dyn/breaking_news.rss").unwrap(),
                    description: "Official National Aeronautics and Space Administration Website",
                }
            }
        }
    }
}
