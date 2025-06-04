use dioxus::prelude::*;
mod components;
mod feed_reader;
mod parsed_reader;

use crate::{
    components::navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
    hooks::use_token,
    platform::share_feed_item,
    views::Route,
};
use ui::icons::{
    ArrowTopRightOnSquareIcon, Bars3Icon, BookmarkIcon, NewspaperIcon, PlusIcon, QueueIcon,
    SearchIcon, ShareIcon, TextSettingsIcon,
};

pub use feed_reader::FeedReader;
pub use parsed_reader::ParsedReader;

#[component]
pub fn ReaderLayout() -> Element {
    // TODO: better way to redirect to sign up if not logged in?
    let nav = use_navigator();
    let token = use_token();

    use_effect(move || {
        if token().is_none() {
            nav.push(Route::SignUp {});
        }
    });

    rsx! {
        div {
            display: "grid",
            grid_template_rows: "1fr auto",
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
