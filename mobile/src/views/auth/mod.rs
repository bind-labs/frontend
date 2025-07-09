use std::ops::Deref;

use dioxus::prelude::*;

use crate::{hooks::use_token, views::Route};

mod components;
mod login;
mod reset_password;
mod reset_password_confirm;
mod sign_up;
mod validation;
mod verify_email;

pub use login::Login;
pub use reset_password::ResetPassword;
pub use reset_password_confirm::ResetPasswordConfirm;
pub use sign_up::SignUp;
pub use verify_email::VerifyEmail;

#[derive(Clone, Copy, Default)]
struct AuthFormContext {
    email: Signal<String>,
    username: Signal<String>,
    password: Signal<String>,
}
impl AuthFormContext {
    pub fn email(&self) -> String {
        (self.email)().clone()
    }
    pub fn set_email(&mut self, value: String) {
        self.email.set(value);
    }

    pub fn username(&self) -> String {
        (self.username)().clone()
    }
    pub fn set_username(&mut self, value: String) {
        self.username.set(value);
    }

    /// Prefers username over email
    pub fn email_or_username(&self) -> String {
        ((self.username)().is_empty())
            .then(|| (self.email)().clone())
            .unwrap_or((self.username)().clone())
    }
    pub fn set_email_or_username(&mut self, value: String) {
        if value.contains('@') {
            self.email.set(value);
            self.username.set("".to_string());
        } else {
            self.email.set("".to_string());
            self.username.set(value);
        }
    }

    pub fn password(&self) -> String {
        (self.password)().clone()
    }
    pub fn set_password(&mut self, value: String) {
        self.password.set(value);
    }
}

pub fn use_auth_form() -> AuthFormContext {
    use_context::<AuthFormContext>()
}

#[component]
pub fn AuthLayout() -> Element {
    use_context_provider(AuthFormContext::default);

    let mut token = use_token();
    use_effect(move || {
        if token().is_some() {
            navigator().push(Route::Feed {});
        }
    });

    rsx! { Outlet::<Route> {} }
}
