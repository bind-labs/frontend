use dioxus::prelude::*;

use crate::{
    api::ApiClient,
    hooks::use_token,
    platform::use_persistent,
    views::auth::components::{AuthContainer, Error},
    views::Route,
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
    let mut token = use_token();
    let mut email_or_username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let login = use_callback(move |_| {
        if email_or_username().is_empty() || password().is_empty() {
            error.set(Some("Email and password cannot be empty".to_string()));
            return;
        }

        spawn(async move {
            let client = ApiClient::new("https://api.bind.sh".to_string());
            match client.login_user(&email_or_username(), &password()).await {
                Ok(response) => {
                    token.set(Some(response.token));
                    navigator().push(Route::Feed {});
                }
                Err(err) => {
                    error.set(Some(err.to_string()));
                }
            }
        });
    });

    rsx! {
        AuthContainer {
            Header { subtitle: "Welcome back to", title: "Bind" }

            // OAuth
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
                    onchange: move |value| {
                        email_or_username.set(value);
                    },
                }
                Input {
                    title: "Password",
                    placeholder: "Password",
                    icon: rsx! {
                        LockIcon {}
                    },
                    input_type: "password",
                    onchange: move |value| {
                        password.set(value);
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
