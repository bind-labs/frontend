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
pub fn Login() -> Element {
    let mut email_or_username = use_signal(String::new);
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
                    password: true,
                    onchange: move |value| {
                        password.set(value);
                    },
                }

                // Actions
                Column { gap: "12px", align: "stretch",

                    SolidButton { onclick: move |_| {}, "Login" }
                    TransparentButton { onclick: move |_| {}, "Reset Password" }
                    TransparentButton {
                        onclick: move |_| {
                            navigator().push(Route::SignUp {});
                        },
                        "Sign up"
                    }
                }
            }
        }
    }
}
