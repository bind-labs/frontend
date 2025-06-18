use dioxus::prelude::*;

use crate::{
    components::{
        navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
        popup::{use_popup_state, Popup, PopupState},
    },
    hooks::use_token,
    platform::init_back_press_listener,
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
    let mut popup_state = use_popup_state();

    let nav = use_navigator();
    let mut token = use_token();

    // TODO: better way to redirect to sign up if not logged in?
    use_effect(move || {
        if token().is_none() {
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
                onsettings: move || {
                    tracing::info!("Settings clicked");
                    token.set(None);
                },
            }

            main { overflow: "auto", position: "relative",
                Outlet::<Route> {}
                Popup {}
            }


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
                NavbarButtonWithoutRoute {
                    onclick: move |_| {
                        match popup_state() {
                            PopupState::Open(_) => {
                                popup_state.set(PopupState::Close);
                            }
                            PopupState::Close => popup_state.set(PopupState::Open(rsx!{ div { "Hello" } })),
                        }
                    },
                    PlusIcon {},
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
