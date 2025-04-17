use dioxus::prelude::*;

mod login;
mod register;

use login::Login;
use register::Register;

#[derive(Debug, Clone, Routable, PartialEq)]
#[layout(AuthLayout)]
enum Route {
    #[route("/")]
    Login {},
    #[route("/register")]
    Register {},
}
