use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod auth;
pub mod dashboard;
pub mod reader;

use auth::*;
use dashboard::*;
use reader::*;

use crate::{
    components::popup::{use_popup_state_provider, PopupState},
    platform::init_back_press_listener,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[layout(AuthLayout)]
            // Auth
            #[route("/auth/sign-up")]
            SignUp {},
            // TODO: better way to send this data to the route?
            // https://github.com/DioxusLabs/dioxus/issues/3874
            #[route("/auth/verify-email?:email&:username&:password")]
            VerifyEmail {
                email: String,
                username: String,
                password: String,
            },
            #[route("/auth/login")]
            Login {},
            #[route("/auth/reset-password")]
            ResetPassword {},
            #[route("/auth/reset-password-confirm?:email")]
            ResetPasswordConfirm { email: String },

        // Dashboard
        #[layout(DashboardLayout)]
            #[route("/")]
            Feed {},
            #[route("/list/:id")]
            List { id: usize },
            #[route("/search")]
            Search {},
            #[route("/add-feed")]
            AddFeed {},

        // Reader
        #[layout(ReaderLayout)]
            #[route("/reader")]
            FeedReader {},
            #[route("/reader/parsed")]
            ParsedReader {},
}

#[component]
fn RootLayout() -> Element {
    let mut popup_state = use_popup_state_provider();

    // Handle back events
    use_future(move || async move {
        let mut rx = init_back_press_listener();
        loop {
            if let Ok(()) = rx.try_recv() {
                if matches!(*popup_state.read(), PopupState::Open(_)) {
                    popup_state.set(PopupState::Close);
                } else {
                    navigator().go_back();
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}
