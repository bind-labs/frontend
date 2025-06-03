use dioxus::prelude::*;

mod components;
mod login;
mod reset_password;
mod reset_password_confirm;
mod sign_up;
mod validation;
mod verify_email;

use login::Login;
use reset_password::ResetPassword;
use reset_password_confirm::ResetPasswordConfirm;
use sign_up::SignUp;
use verify_email::VerifyEmail;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // default to /sign-up
    #[redirect("/:..segments", |segments: Vec<String>| Route::SignUp {})]
    #[route("/sign-up")]
    SignUp {},
    #[route("/verify-email")]
    VerifyEmail {
        email: String,
        username: String,
        password: String,
    },
    #[route("/login")]
    Login {},
    #[route("/reset-password")]
    ResetPassword {},
    #[route("/reset-password-confirm")]
    ResetPasswordConfirm { email: String },
}
