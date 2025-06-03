use dioxus::prelude::*;

use crate::{
    platform,
    views::auth::{
        components::{AuthContainer, Header},
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
    let mut code = use_signal(String::new);

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
                SolidButton { onclick: move |_| { platform::open_email(); },
                    "Open Email App"
                }
                TransparentButton { onclick: move |_| {},
                    "Didn't receive the code? Resend"
                }
                TransparentButton { onclick: move |_| { navigator().push(Route::SignUp {}); },
                    "Go Back"
                }
            }
        }
    }
}
