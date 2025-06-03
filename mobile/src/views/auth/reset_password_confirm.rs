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
pub fn ResetPasswordConfirm(email: String) -> Element {
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

            CodeInput { length: 5, onchange: move |value| code.set(value) }

            Input {
                title: "Password",
                placeholder: "Password",
                icon: rsx! {
                    LockIcon {}
                },
                input_type: "password",
                onchange: move |value| password.set(value),
            }

            Column { gap: "8px",
                SolidButton {
                    onclick: move |_| {
                        // TODO:
                    },
                    "Reset Password"
                }

                TransparentButton { onclick: move |_| { navigator().push(Route::ResetPassword {}); },
                    "Go Back"
                }

                TransparentButton { onclick: move |_| { navigator().push(Route::Login {}); },
                    "Login"
                }
            }
        }
    }
}
