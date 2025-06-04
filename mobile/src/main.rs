#![feature(lock_value_accessors)]

use api::types::auth::AuthUser;
use api::ApiClient;
use dioxus::dioxus_core::LaunchConfig;
use dioxus::mobile::wry::WebView;
use dioxus::mobile::{use_window, window, Config, WindowBuilder};
use dioxus::prelude::*;

mod api;
mod components;
mod hooks;
mod platform;
mod views;

use components::container::FixedSizeContainer;
use platform::use_platform_setup;
use views::Route;

const THEME_CSS: Asset = asset!("/assets/theme.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const SOURCE_SERIF_4_ITALIC: Asset = asset!("/assets/fonts/SourceSerif4Variable-Italic.otf.woff2");
const SOURCE_SERIF_4_ROMAN: Asset = asset!("/assets/fonts/SourceSerif4Variable-Roman.otf.woff2");
const IBM_PLEX_MONO: Asset = asset!("/assets/fonts/ibm-plex-mono-latin-400-normal.woff2");

fn main() {
    dioxus::LaunchBuilder::mobile()
        .with_cfg(Config::new().with_background_color((0xF2, 0xED, 0xE3, 0xFF)))
        .launch(App);
}

#[derive(Clone)]
struct AuthContext {
    user: Option<AuthUser>,
}

#[derive(Clone)]
struct AppContext {
    api_client: ApiClient,
}

#[component]
fn App() -> Element {
    use_platform_setup();

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { "@font-face {{ font-family: 'Source Serif 4'; src: url({SOURCE_SERIF_4_ROMAN}); }}" }
        style {
            "@font-face {{ font-family: 'Source Serif 4'; font-style: italic; src: url({SOURCE_SERIF_4_ITALIC}); }}"
        }
        style {
            "@font-face {{ font-family: 'IBM Plex Mono'; font-style: italic; src: url({IBM_PLEX_MONO}); }}"
        }

        // Routers
        FixedSizeContainer {
            Router::<Route> {}
        }
    }
}
