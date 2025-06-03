use dioxus::{mobile::window, prelude::*};
use regex::Regex;

use super::components::Header;
use crate::hooks::use_keyboard_open;
use crate::views::auth::components::{AuthContainer, Error};
use crate::views::auth::validation::{validate_email, validate_password, validate_username};
use crate::{api::ApiClient, views::Route};

use ui::{
    forms::{
        button::{SolidButton, TransparentButton, UnstyledButton},
        code_input::CodeInput,
        input::Input,
    },
    icons::{AppleIcon, EnvelopeIcon, GoogleIcon, LockIcon, UserIcon},
    layout::Column,
};

#[component]
pub fn SignUp() -> Element {
    let navigator = use_navigator();
    let mut email = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let keyboard_open = use_keyboard_open();

    let sign_up = use_callback(move |_| {
        if let Err(err) = validate_email(&email()) {
            error.set(Some(err));
        } else if let Err(err) = validate_username(&username()) {
            error.set(Some(err));
        } else if let Err(err) = validate_password(&password()) {
            error.set(Some(err));
        } else {
            error.set(None);
            spawn(async move {
                let client = ApiClient::new(String::from("https://api.bind.sh"));
                match client.send_email_verification(&email()).await {
                    Ok(_) => {
                        navigator.push(Route::VerifyEmail {
                            email: email(),
                            username: username(),
                            password: password(),
                        });
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                    }
                }
            });
        }
    });

    rsx! {
        AuthContainer {
            Header { subtitle: "Welcome to", title: "Bind" }

            // OAuth
            if !keyboard_open() {
                Column { gap: "24px",

                    button { class: "solid", onclick: move |_| {},
                        GoogleIcon {}
                        "Sign up with Google"
                    }
                    button { class: "solid", onclick: move |_| {},
                        AppleIcon {}
                        "Sign up with Apple"
                    }
                }

                hr { width: "100px" }
            }

            // Form
            Column { gap: "16px",

                Input {
                    title: "Email",
                    placeholder: "Email",
                    icon: rsx! {
                        EnvelopeIcon {}
                    },
                    input_type: "email",
                    onchange: move |value| { email.set(value) },
                }
                Input {
                    title: "Username",
                    placeholder: "Username",
                    icon: rsx! {
                        UserIcon {}
                    },
                    onchange: move |value| {
                        username.set(value);
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
                Column { gap: "8px", align: "stretch",

                    SolidButton {
                        onclick: move |_| { sign_up.call(()) },
                        "Sign up"
                    }
                    TransparentButton {
                        onclick: move |_| {
                            navigator.push(Route::Login {});
                        },
                        "Login"
                    }
                    Error { error }
                }
            }
        }
    }
}
