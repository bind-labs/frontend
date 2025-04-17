use dioxus::prelude::*;

use crate::components::navbar::{Navbar, NavbarButton};
use ui::icons::{Bars3Icon, BookmarkIcon, PlusIcon, QueueIcon, SearchIcon};

mod components;
mod feed;
mod list;
mod search;

use components::Header;
use feed::Feed;
use list::List;
use search::Search;

#[derive(Debug, Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(DashboardLayout)]
        //  if the current location doesn't match any of the other routes, redirect to "/feed"
        #[redirect("/:..segments", |segments: Vec<String>| Route::Feed {})]
        #[route("/feed")]
        Feed {},
        #[route("/list")]
        List {},
        #[route("/search")]
        Search {},
}

#[allow(non_snake_case)]
fn DashboardLayout() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            display: "grid",
            grid_template_rows: "auto 1fr auto",
            height: "100vh",
            width: "100vw",

            Header { title: "Read Later", additional: Some("(16 items)".to_string()), onsettings: move || tracing::info!("Settings clicked") }

            main {
                overflow: "auto",
                Outlet::<Route> {}
            }

            Navbar {
                NavbarButton {
                    to: Route::Search {},
                    icon: |solid| rsx! { SearchIcon { solid } }
                }
                NavbarButton {
                    to: Route::Feed {},
                    icon: |solid| rsx! { QueueIcon { solid } }
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! { PlusIcon { solid } }
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! { BookmarkIcon { solid } }
                }
                NavbarButton {
                    to: Route::List {},
                    icon: |solid| rsx! { Bars3Icon { solid } }
                }
            }
        }
    }
}
