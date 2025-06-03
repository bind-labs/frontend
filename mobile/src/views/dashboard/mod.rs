use dioxus::prelude::*;

use crate::{
    components::navbar::{Navbar, NavbarButton},
    hooks::use_token,
    views::Route,
};
use ui::icons::{Bars3Icon, BookmarkIcon, PlusIcon, QueueIcon, SearchIcon};

mod components;
mod feed;
mod list;
mod search;

use components::Header;
pub use feed::Feed;
pub use list::List;
pub use search::Search;

#[component]
pub fn DashboardLayout() -> Element {
    // TODO: better way to redirect to sign up if not logged in?
    let nav = use_navigator();
    let token = use_token();

    use_effect(move || {
        if token.get().is_none() {
            nav.push(Route::SignUp {});
        }
    });

    rsx! {
        div {
            display: "grid",
            grid_template_rows: "auto 1fr auto",
            height: "100vh",
            width: "100vw",

            Header {
                title: "Read Later",
                additional: Some("(16 items)".to_string()),
                onsettings: move || tracing::info!("Settings clicked"),
            }

            main { overflow: "auto", Outlet::<Route> {} }

            Navbar {
                NavbarButton {
                    to: Route::Search {},
                    icon: |solid| rsx! {
                        SearchIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::Feed {},
                    icon: |solid| rsx! {
                        QueueIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! {
                        PlusIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! {
                        BookmarkIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! {
                        Bars3Icon { solid }
                    },
                }
            }
        }
    }
}
