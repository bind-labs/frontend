use dioxus::prelude::*;

mod components;
mod login;
mod reset_password;
mod sign_up;
use login::Login;
use reset_password::ResetPassword;
use sign_up::SignUp;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // default to /sign-up
    #[redirect("/:..segments", |segments: Vec<String>| Route::ResetPassword {})]
    #[route("/sign-up")]
    SignUp {},
    #[route("/login")]
    Login {},
    #[route("/reset-password")]
    ResetPassword {},
}
