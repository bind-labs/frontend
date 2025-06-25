use dioxus::prelude::*;
use ui::layout::*;

#[derive(Props, Clone, PartialEq)]
pub struct FeedItemListItemProps {
    pub image_url: String,
    pub title: String,
    pub description: String,
    pub feed_name: String,
    pub created_at: String,
    pub unread: bool,
}

#[component]
pub fn FeedItemListItem(props: FeedItemListItemProps) -> Element {
    let FeedItemListItemProps {
        image_url,
        title,
        description,
        feed_name,
        created_at,
        unread,
    } = props;

    rsx! {
        Row {
            padding: "10px 16px",
            border_bottom: "1px solid var(--text-secondary)",
            gap: "12px",
            cross_align: "center",

            img { src: image_url, width: "80px", height: "80px" }
            Column { gap: "2px",
                span {
                    font_size: "16px",
                    color: if unread { "var(--text)" } else { "var(--text-tertiary)" },
                    "{title}"
                }
                span {
                    font_size: "12px",
                    color: if unread { "var(--text-secondary)" } else { "var(--text-tertiary)" },
                    "({feed_name}) {created_at}"
                }
                span {
                    font_size: "12px",
                    color: if unread { "var(--text-secondary)" } else { "var(--text-tertiary)" },
                    "{description}"
                }
            }
        }
    }
}

#[component]
pub fn FeedItemList(num: usize) -> Element {
    rsx! {
        Column {
            for i in 0..num {
                FeedItemListItem {
                    key: "{i}",
                    image_url: "https://upload.wikimedia.org/wikipedia/en/d/d1/Plasticbeach452.jpg",
                    title: "On Melancholy Hill",
                    description: "On Melancholy Hill is the third single from British virtual band Gorillaz's third studio album, Plastic Beach.",
                    feed_name: "Genius",
                    created_at: "28 minutes ago",
                    unread: i % 2 == 0,
                }
            }
        }
    }
}
