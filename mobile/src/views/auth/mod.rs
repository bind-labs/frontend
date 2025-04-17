use dioxus::prelude::*;

mod components;
mod login;
mod sign_up;

use login::Login;
use sign_up::SignUp;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // default to /sign-up
    #[redirect("/:..segments", |segments: Vec<String>| Route::SignUp {})]
    #[route("/sign-up")]
    SignUp {},
    #[route("/login")]
    Login {},
}
