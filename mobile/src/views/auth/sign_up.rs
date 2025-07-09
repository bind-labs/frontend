use dioxus::{mobile::window, prelude::*};
use regex::Regex;

use super::components::Header;
use crate::hooks::{use_api, use_keyboard_open, use_token};
use crate::views::auth::components::{AuthContainer, Error};
use crate::views::auth::use_auth_form;
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
    let api = use_api();
    let nav = use_navigator();
    let mut auth_form = use_auth_form();
    let mut error = use_signal(|| None::<String>);
    let mut token = use_token();

    tracing::info!("Sign up screen");

    let keyboard_open = use_keyboard_open();

    let send_email_verification = use_callback(move |_| {
        if let Err(err) = validate_email(&auth_form.email()) {
            error.set(Some(err));
        } else if let Err(err) = validate_username(&auth_form.username()) {
            error.set(Some(err));
        } else if let Err(err) = validate_password(
            &auth_form.email(),
            &auth_form.username(),
            &auth_form.password(),
        ) {
            error.set(Some(err));
        } else {
            error.set(None);
            spawn(async move {
                match api.send_email_verification(&auth_form.email()).await {
                    Ok(_) => {
                        nav.push(Route::VerifyEmail {
                            email: auth_form.email(),
                            username: auth_form.username(),
                            password: auth_form.password(),
                        });
                    }
                    Err(err) => {
                        error.set(Some(err.message()));
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
                    value: auth_form.email(),
                    onchange: move |value| { auth_form.set_email(value) },
                }
                Input {
                    title: "Username",
                    placeholder: "Username",
                    icon: rsx! {
                        UserIcon {}
                    },
                    value: auth_form.username(),
                    onchange: move |value| {
                        auth_form.set_username(value);
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
                Column { gap: "8px", align: "stretch",

                    SolidButton {
                        onclick: move |_| { send_email_verification.call(()) },
                        "Sign up"
                    }
                    TransparentButton {
                        onclick: move |_| {
                            nav.push(Route::Login {});
                        },
                        "Login"
                    }
                    Error { error }
                }
            }
        }
    }
}
