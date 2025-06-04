use dioxus::prelude::*;

use crate::views::Route;
use crate::{
    hooks::use_api,
    views::auth::{
        components::{AuthContainer, Error},
        validation::validate_email,
    },
};

use super::components::Header;
use ui::{
    forms::{
        button::{SolidButton, TransparentButton},
        input::Input,
    },
    icons::{EnvelopeIcon, LockIcon},
    layout::Column,
};

#[component]
pub fn ResetPassword() -> Element {
    let api = use_api();
    let mut email = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let send_password_reset_code = use_callback(move |_| {
        if let Err(err) = validate_email(&email()) {
            error.set(Some(err));
        } 

        spawn(async move {
            match api.send_password_reset_code(&email()).await {
                Ok(_) => {
                    navigator().push(Route::ResetPasswordConfirm { email: email() });
                }
                Err(err) => {
                    error.set(Some(err.message()));
                }
            }
        });
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
                    onclick: send_password_reset_code,
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
