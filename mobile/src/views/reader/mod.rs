use dioxus::prelude::*;

mod feed_reader;
mod parsed_reader;

use crate::components::navbar::{Navbar, NavbarButton};
use ui::icons::{Bars3Icon, BookmarkIcon, PlusIcon, QueueIcon, SearchIcon};

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
fn ReaderLayout() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            display: "grid",
            grid_template_rows: "auto 1fr auto",
            height: "100vh",
            width: "100vw",

            main { overflow: "auto", Outlet::<Route> {} }

            Navbar {

            }
        }
    }
}
