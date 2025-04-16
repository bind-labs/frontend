use dioxus::prelude::*;
use ui::{
    forms::{
        button::{SolidButton, UnstyledButton},
        input::Input,
    },
    icons::EnvelopeIcon,
    Header,
};

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        Header {
            subtitle: "Welcome to",
            title: "Bind",
        }
        div {
            // oauth buttons here
        }
        div {
            Input {
                title: "Username or Email",
                icon: rsx! {
                    EnvelopeIcon { size: 24 }
                },
                placeholder: "Username or Email",
                password: false,
                onchange: move |value| {
                    username.set(value);
                }

            },
            Input {
                title: "Password",
                icon: rsx! {
                    EnvelopeIcon { size: 24 }
                },
                password: true,
                onchange: move |value| {
                    password.set(value);
                }
            }
        }
        div {
            SolidButton {
                "Login"
            },
            UnstyledButton {
                "Reset Password"
            },
            UnstyledButton {
                "Sign up"
            }
        }
    }
}
