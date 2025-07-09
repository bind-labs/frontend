use dioxus::prelude::*;

use crate::{
    components::{
        navbar::{Navbar, NavbarButton, NavbarButtonWithoutRoute},
        popup::{use_popup_state, Popup, PopupList, PopupListItem, PopupState},
    },
    hooks::{use_keyboard_open, use_token},
    platform::init_back_press_listener,
    views::{dashboard::components::DashboardNavbar, Route},
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
    let keyboard_open = use_keyboard_open();
    let mut token = use_token();

    tracing::info!("Dashboard screen");

    // TODO: better way to redirect to sign up if not logged in?
    use_effect(move || {
        if token().is_none() {
            navigator().push(Route::SignUp {});
        }
    });

    rsx! {
        div {
            id: "dashboard",

            main {
                Outlet::<Route> {}
                Popup {}
            }

            if !keyboard_open() {
                DashboardNavbar {}
            }
        }
    }
}
