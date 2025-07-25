use dioxus::prelude::*;

use crate::{
    api::{types::user::UserRegisterRequest, ApiClient},
    hooks::{use_api, use_token},
    platform,
    views::{
        auth::components::{AuthContainer, Error, Header},
        Route,
    },
};
use ui::{
    forms::{
        button::{SolidButton, TransparentButton},
        code_input::CodeInput,
    },
    icons::EnvelopeIcon,
    layout::Column,
};

#[component]
pub fn VerifyEmail(email: String, username: String, password: String) -> Element {
    let api = use_api();
    let mut token = use_token();
    let mut code = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let register = use_callback(move |_| {
        if code().len() != 5 {
            error.set(Some("Code must be 5 digits long".to_string()));
            return;
        }
        error.set(None);

        let email = email.clone();
        let username = username.clone();
        let password = password.clone();

        spawn(async move {
            match api
                .register_user(&UserRegisterRequest {
                    email,
                    email_code: code(),
                    username,
                    password,
                })
                .await
            {
                Ok(response) => {
                    token.set(Some(response.token));
                    navigator().push(Route::Feed {});
                }
                Err(err) => {
                    error.set(Some(err.message()));
                }
            }
        });
    });

    rsx! {
        AuthContainer {
            gap: "48px",

            Header {
                subtitle: "Verify your",
                title: "Email",
                icon: rsx! {
                    EnvelopeIcon { size: 82 }
                },
            }

            CodeInput {
                length: 5,
                onchange: move |value| code.set(value),
            }

            Column { gap: "8px",
                Column { gap: "16px",
                    SolidButton { onclick: move |_| { register.call(()); },
                        "Submit"
                    }
                    SolidButton { onclick: move |_| { platform::open_email(); },
                        "Open Email App"
                    }
                }
                TransparentButton { onclick: move |_| {},
                    "Didn't receive the code? Resend"
                }
                TransparentButton { onclick: move |_| { navigator().push(Route::SignUp {}); },
                    "Go Back"
                }
                Error { error }
            }
        }
    }
}
