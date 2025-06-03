use dioxus::prelude::*;

use crate::views::auth::{components::AuthContainer, Route};

use super::components::Header;
use ui::{
    forms::{
        button::{SolidButton, TransparentButton, UnstyledButton},
        code_input::CodeInput,
        input::Input,
    },
    icons::{EnvelopeIcon, LockIcon},
    layout::Column,
};

#[component]
pub fn ResetPassword() -> Element {
    let mut email_sent = use_signal(|| true);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut code = use_signal(String::new);
    rsx! {
        AuthContainer {
            Header {
                title: "Password",
                subtitle: "Reset your",
                icon: rsx! {
                    LockIcon { size: 82 }
                },
            }
            Column {
                max_width: "300px",
                width: "100%",
                gap: "16px",
                padding: "36px 16px",
                Input {
                    title: "Email",
                    placeholder: "Email",
                    icon: rsx! {
                        EnvelopeIcon {}
                    },
                    input_type: "email",
                    onchange: move |value| email.set(value),
                }

                if email_sent() {
                    Input {
                        title: "Password",
                        placeholder: "Password",
                        icon: rsx! {
                            LockIcon {}
                        },
                        input_type: "password",
                        onchange: move |value| password.set(value),
                    }
                }
            }

            if email_sent() {
                CodeInput { length: 5, onchange: move |value| code.set(value) }
            }


            Column { gap: "8px",
                SolidButton { onclick: move |_| { if email_sent() {} else { email_sent.set(true) } },
                    "Reset Password"
                }

                TransparentButton {

                    onclick: move |_| {
                        navigator().push(Route::Login {});
                    },
                    "Login"
                }
            }
        }
    }
}
