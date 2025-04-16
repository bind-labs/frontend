use components::header::Header;
use dioxus::dioxus_core::LaunchConfig;
use dioxus::mobile::wry::WebView;
use dioxus::prelude::*;

use ui::Navbar;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const SOURCE_SERIF_4_ITALIC: Asset = asset!("/assets/fonts/SourceSerif4Variable-Italic.otf.woff2");
const SOURCE_SERIF_4_ROMAN: Asset = asset!("/assets/fonts/SourceSerif4Variable-Roman.otf.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { "@font-face {{ font-family: 'Source Serif 4'; src: url({SOURCE_SERIF_4_ROMAN}); }}" }
        style { "@font-face {{ font-family: 'Source Serif 4'; font-style: italic; src: url({SOURCE_SERIF_4_ITALIC}); }}" }

        Header { title: "Read Later", additional: Some("(16 items)".to_string()), onsettings: move || tracing::info!("Settings clicked") }
        Router::<Route> {}
    }
}
