use dioxus::prelude::*;

use crate::{
    api::ApiClient,
    hooks::{use_api, use_keyboard_open, use_token},
    views::{
        auth::{
            components::{AuthContainer, Error},
            use_auth_form, AuthFormContext,
        },
        Route,
    },
};

use super::components::Header;
use ui::{
    forms::{
        button::{SolidButton, TransparentButton},
        input::Input,
    },
    icons::{AppleIcon, GoogleIcon, LockIcon, UserIcon},
    layout::Column,
};

#[component]
pub fn Login() -> Element {
    let api = use_api();

    let mut auth_form = use_auth_form();
    let mut error = use_signal(|| None::<String>);
    let mut token = use_token();

    let mut keyboard_open = use_keyboard_open();

    let login = use_callback(move |_| {
        let email = auth_form.email();
        let username = auth_form.username();
        let password = auth_form.password();

        // Validate
        if (email.is_empty() && username.is_empty()) || password.is_empty() {
            error.set(Some(
                "Email/username and password cannot be empty".to_string(),
            ));
            return;
        }

        // Login
        spawn(async move {
            match api
                .login_user(&auth_form.email_or_username(), &password)
                .await
            {
                Ok(response) => {
                    token.set(Some(response.token));
                    error.set(None);
                }
                Err(err) => error.set(Some(err.message())),
            };
        });
    });

    rsx! {
        AuthContainer {
            Header { subtitle: "Welcome back to", title: "Bind" }

            // OAuth
            if !keyboard_open() {
                Column { gap: "24px",

                    button { class: "solid", onclick: move |_| {},
                        GoogleIcon {}
                        "Login with Google"
                    }
                    button { class: "solid", onclick: move |_| {},
                        AppleIcon {}
                        "Login with Apple"
                    }
                }
            }

            hr { width: "100px" }

            // Form
            Column { gap: "24px",

                Input {
                    title: "Email or Username",
                    placeholder: "Email or Username",
                    icon: rsx! {
                        UserIcon {}
                    },
                    input_type: "email",
                    value: auth_form.email_or_username(),
                    onchange: move |value| {
                        auth_form.set_email_or_username(value);
                    },
                }
                Input {
                    title: "Password",
                    placeholder: "Password",
                    icon: rsx! {
                        LockIcon {}
                    },
                    input_type: "password",
                    value: auth_form.password(),
                    onchange: move |value| {
                        auth_form.set_password(value);
                    },
                }

                // Actions
                Column { gap: "12px", align: "stretch",

                    SolidButton { onclick: move |_| { login.call(()); }, "Login" }
                    TransparentButton {
                        onclick: move |_| {
                            navigator().push(Route::ResetPassword {});
                        },
                        "Reset Password"
                    }
                    TransparentButton {
                        onclick: move |_| {
                            navigator().push(Route::SignUp {});
                        },
                        "Sign up"
                    }
                    Error { error }
                }
            }
        }
    }
}
