use dioxus::prelude::*;

use crate::views::auth::Route;

use super::components::Header;
use ui::{
    forms::{
        button::{SolidButton, TransparentButton, UnstyledButton},
        input::Input,
    },
    icons::{AppleIcon, EnvelopeIcon, GoogleIcon, LockIcon, UserIcon},
    layout::Column,
};

#[component]
pub fn SignUp() -> Element {
    let mut email = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        Column {
            align: "stretch",
            cross_align: "space-evenly",

            height: "100vh",
            width: "100%",
            max_width: "300px",
            margin: "auto",
            padding: "36px 16px",

            Header {
                subtitle: "Welcome to",
                title: "Bind",
            }

            // OAuth
            Column {
                gap: "24px",

                button {
                    class: "solid",
                    onclick: move |_| {
                        // login
                    },
                    GoogleIcon {},
                    "Sign up with Google",
                },
                button {
                    class: "solid",
                    onclick: move |_| {
                        // redirect to apple oauth
                    },
                    AppleIcon {},
                    "Sign up with Apple",
                }
            }

            hr { width: "100px" }

            // Form
            Column {
                gap: "24px",

                Input {
                    title: "Email",
                    placeholder: "Email",
                    icon: rsx! {
                        EnvelopeIcon {}
                    },
                    onchange: move |value| {
                        email.set(value);
                    }
                },
                Input {
                    title: "Username",
                    placeholder: "Username",
                    icon: rsx! {
                        UserIcon {}
                    },
                    onchange: move |value| {
                        username.set(value);
                    }
                },
                Input {
                    title: "Password",
                    placeholder: "Password",
                    icon: rsx! {
                        LockIcon {}
                    },
                    password: true,
                    onchange: move |value| {
                        password.set(value);
                    }
                }

                // Actions
                Column {
                    gap: "12px",
                    align: "stretch",

                    SolidButton {
                        onclick: move |_| {
                            // login
                        },
                        "Sign up",
                    },
                    TransparentButton {
                        onclick: move |_| {
                            navigator().push(Route::Login {});
                        },
                        "Login",
                    }
                }
            }
        }
    }
}
