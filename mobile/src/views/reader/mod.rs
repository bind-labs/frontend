use dioxus::{mobile::window, prelude::*};
mod components;
mod feed_reader;
mod parsed_reader;

use crate::{
    components::navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
    share::share_feed_item,
};
use ui::icons::{
    ArrowTopRightOnSquareIcon, Bars3Icon, BookmarkIcon, NewspaperIcon, PlusIcon, QueueIcon,
    SearchIcon, ShareIcon, TextSettingsIcon,
};

use feed_reader::FeedReader;
use parsed_reader::ParsedReader;

#[derive(Debug, Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(ReaderLayout)]
        // default to /default
        #[redirect("/:..segments", |segments: Vec<String>| Route::FeedReader {})]
        #[route("/")]
        FeedReader {},
        #[route("/parsed")]
        ParsedReader {},

}

#[allow(non_snake_case)]
pub fn ReaderLayout() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            display: "grid",
            grid_template_rows: "auto 1fr auto",
            height: "100vh",
            width: "100vw",

            main { overflow: "auto", padding: "16px", Outlet::<Route> {} }

            Navbar {
                NavbarButtonWithoutRoute { onclick: move |_| {},
                    span {
                        font_family: "IBM Plex Mono",
                        font_size: "24px",
                        line_height: "20px",
                        "Aa"
                    }
                }
                NavbarButton {
                    to: Route::FeedReader {},
                    icon: |solid| rsx! {
                        NewspaperIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::ParsedReader {},
                    icon: |solid| rsx! {
                        ArrowTopRightOnSquareIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::ParsedReader {},
                    icon: |solid| rsx! {
                        BookmarkIcon { solid }
                    },
                }
                NavbarButtonWithoutRoute {
                    onclick: move |_| {
                        share_feed_item("https://example.com".to_string(), "testing".to_string());
                    },
                    ShareIcon {
                    }
                }
            }
        }
    }
}
