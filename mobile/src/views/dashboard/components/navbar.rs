use dioxus::prelude::*;
use ui::icons::{Bars3Icon, BookmarkIcon, PlusIcon, QueueIcon, SearchIcon};

use crate::{
    components::{
        navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
        popup::{use_popup_state, PopupList, PopupListItem, PopupState},
    },
    views::Route,
};

#[component]
pub fn DashboardNavbar() -> Element {
    let mut popup_state = use_popup_state();
    let nav = use_navigator();

    rsx! {
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
