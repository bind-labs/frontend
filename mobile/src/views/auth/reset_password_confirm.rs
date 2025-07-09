use dioxus::prelude::*;

use super::components::{AuthContainer, Header};
use crate::{hooks::use_api, views::Route};
use ui::{
    forms::{
        button::{SolidButton, TransparentButton},
        code_input::CodeInput,
        input::Input,
    },
    icons::LockIcon,
    layout::Column,
};

#[component]
pub fn ResetPasswordConfirm(email: String) -> Element {
    let api = use_api();

    let mut password = use_signal(String::new);
    let mut code = use_signal(String::new);

    let mut error = use_signal(|| None::<String>);

    let reset_password = use_callback(move |_| {
        if password().is_empty() || code().is_empty() {
            error.set(Some("Password and code cannot be empty".to_string()));
            return;
        }

        let email = email.clone();
        spawn(async move {
            match api.reset_password(&email, &code(), &password()).await {
                Ok(_) => {
                    navigator().push(Route::Login {});
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

            CodeInput { length: 5, onchange: move |value| code.set(value) }

            Input {
                title: "Password",
                placeholder: "Password",
                icon: rsx! {
                    LockIcon {}
                },
                input_type: "password",
                value: password(),
                onchange: move |value| password.set(value),
            }

            Column { gap: "8px",
                SolidButton {
                    onclick: reset_password,
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
