use dioxus::prelude::*;

pub mod auth;
pub mod dashboard;
pub mod reader;

use auth::*;
use dashboard::*;
use reader::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
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
        #[route("/list")]
        List {},
        #[route("/search")]
        Search {},

    // Reader
    #[layout(ReaderLayout)]
        #[route("/reader")]
        FeedReader {},
        #[route("/reader/parsed")]
        ParsedReader {},


}
