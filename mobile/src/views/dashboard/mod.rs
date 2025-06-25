use dioxus::prelude::*;

use crate::{
    components::{
        navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
        popup::{use_popup_state, Popup, PopupList, PopupListItem, PopupState},
    },
    hooks::use_token,
    platform::init_back_press_listener,
    views::Route,
};
use ui::icons::{Bars3Icon, BookmarkIcon, PlusIcon, QueueIcon, SearchIcon};

mod add_feed;
mod components;
mod feed;
mod list;
mod search;

use components::Header;

pub use add_feed::AddFeed;
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
            grid_template_rows: "1fr auto",
            height: "100vh",
            width: "100vw",

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
                            PopupState::Close => popup_state.set(PopupState::Open(
                                rsx! { PopupList {
                                    PopupListItem {
                                        icon: rsx! { QueueIcon {} },
                                        title: "Add New Feed",
                                        onclick: move |_| { nav.push(Route::AddFeed {}); },
                                    }
                                    PopupListItem {
                                        icon: rsx! { BookmarkIcon {} },
                                        title: "Create New List",
                                        onclick: move |_| { nav.push(Route::AddFeed {}); },
                                    }
                                    PopupListItem {
                                        icon: rsx! { SearchIcon {} },
                                        title: "Create New Index",
                                        onclick: move |_| { nav.push(Route::AddFeed {}); },
                                    }
                                } }
                            )),
                        }
                    },
                    PlusIcon {},
                }
                NavbarButton {
                    to: Route::List { id: 1 },
                    icon: |solid| rsx! {
                        BookmarkIcon { solid }
                    },
                }
                NavbarButton {
                    to: Route::List { id: 2 },
                    icon: |solid| rsx! {
                        Bars3Icon { solid }
                    },
                }
            }
        }
    }
}
