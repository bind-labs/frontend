use dioxus::prelude::*;

use crate::views::auth::{
    components::{AuthContainer, Error},
    validation::validate_email,
};
use crate::views::Route;

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
    let mut email = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let reset_password = use_callback(move |_| {
        if let Err(err) = validate_email(&email()) {
            error.set(Some(err));
        } else {
            error.set(None);
            navigator().push(Route::ResetPasswordConfirm { email: email() });
        }
    });

    rsx! {
        AuthContainer {
            Header {
                title: "Password",
                subtitle: "Reset your",
                icon: rsx! {
                    LockIcon { size: 82 }
                },
            }

            Input {
                title: "Email",
                placeholder: "Email",
                icon: rsx! {
                    EnvelopeIcon {}
                },
                input_type: "email",
                onchange: move |value| email.set(value),
            }

            Column { gap: "8px",
                SolidButton {
                    onclick: reset_password,
                    "Reset Password"
                }
                TransparentButton { onclick: move |_| { navigator().push(Route::Login {}); },
                    "Login"
                }
                Error { error }
            }
        }
    }
}
