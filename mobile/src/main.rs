use components::header::Header;
use dioxus::dioxus_core::LaunchConfig;
use dioxus::mobile::wry::WebView;
use dioxus::mobile::{use_window, Config, WindowBuilder};
use dioxus::prelude::*;

use ui::Navbar;
use views::*;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum AuthRoute {
    #[route("/")]
    Register {},
    #[route("/login")]
    Login {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const SOURCE_SERIF_4_ITALIC: Asset = asset!("/assets/fonts/SourceSerif4Variable-Italic.otf.woff2");
const SOURCE_SERIF_4_ROMAN: Asset = asset!("/assets/fonts/SourceSerif4Variable-Roman.otf.woff2");

fn main() {
    dioxus::LaunchBuilder::mobile()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_title("Bind")))
        .launch(App);
}

fn set_android_flags() {
    if cfg!(target_os = "android") {
        use dioxus::mobile::wry::prelude::dispatch;
        dispatch(|env, activity, _webview| {
            // Get the window
            let window = env
                .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
                .unwrap()
                .l()
                .unwrap();

            // Set window flags
            const FLAG_KEEP_SCREEN_ON: i32 = 128;
            env.call_method(&window, "addFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()])
                .unwrap();

            // Use dark icons for status bar
            let insets_controller = env
                .call_method(
                    &window,
                    "getInsetsController",
                    "()Landroid/view/WindowInsetsController;",
                    &[],
                )
                .unwrap()
                .l()
                .unwrap();

            if !insets_controller.is_null() {
                let appearance_flag = 0x00000008; // APPEARANCE_LIGHT_STATUS_BARS
                env.call_method(
                    &insets_controller,
                    "setSystemBarsAppearance",
                    "(II)V",
                    &[appearance_flag.into(), appearance_flag.into()],
                )
                .unwrap();
            }

            // Set status bar color
            let color = 0xFFF2EDE3u32 as i32; // ARGB
            env.call_method(&window, "setStatusBarColor", "(I)V", &[color.into()])
                .unwrap();

            // Set navigation bar color
            env.call_method(&window, "setNavigationBarColor", "(I)V", &[color.into()])
                .unwrap();
        });
    }
}

#[component]
fn App() -> Element {
    let mut is_logged_in = use_signal(|| false);

    use_effect(|| {
        set_android_flags();
    });

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { "@font-face {{ font-family: 'Source Serif 4'; src: url({SOURCE_SERIF_4_ROMAN}); }}" }
        style { "@font-face {{ font-family: 'Source Serif 4'; font-style: italic; src: url({SOURCE_SERIF_4_ITALIC}); }}" }

        Header { title: "Read Later", additional: Some("(16 items)".to_string()), onsettings: move || tracing::info!("Settings clicked") }
        button { onclick: move |_| is_logged_in.set(!is_logged_in()), "Login" }
        if is_logged_in() {
            Router::<Route> {}
        } else {
            Router::<AuthRoute> {}
        }
    }
}
