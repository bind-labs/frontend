use dioxus::prelude::*;

use crate::views::auth::Route;

use super::components::Header;
use ui::{
    forms::{
        button::{SolidButton, TransparentButton, UnstyledButton},
        code_input::CodeInput,
        input::Input,
    },
    icons::{AppleIcon, EnvelopeIcon, GoogleIcon, LockIcon, UserIcon},
    layout::Column,
};

#[component]
pub fn SignUp() -> Element {
    let mut email_sent = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut code = use_signal(String::new);

    rsx! {
        if email_sent() {
            Column {
                height: "100%",
                width: "100%",
                margin: "auto",
                gap: "16px",
                align: "center",

                Header {
                    subtitle: "Verify your",
                    title: "Email",
                    icon: rsx! {
                        EnvelopeIcon { size: 82 }
                    },
                }

                CodeInput {
                    length: 5,
                    onchange: move |value| {
                        code.set(value);
                    },
                }

                SolidButton { onclick: move |_| {}, "Open Email App" }
                TransparentButton {
                    onclick: move |_| {},
                    "Didn't receive the code? Resend"
                }
            }
        } else {
            Column {
                align: "stretch",
                cross_align: "space-evenly",

                height: "100%",
                width: "100%",
                max_width: "300px",
                margin: "auto",
                padding: "36px 16px",


                Header { subtitle: "Welcome to", title: "Bind" }



                // OAuth
                Column { gap: "24px",

                    button { class: "solid", onclick: move |_| {},
                        GoogleIcon {}
                        "Sign up with Google"
                    }
                    button { class: "solid", onclick: move |_| {},
                        AppleIcon {}
                        "Sign up with Apple"
                    }
                }

                hr { width: "100px" }

                // Form
                Column { gap: "24px",

                    Input {
                        title: "Email",
                        placeholder: "Email",
                        icon: rsx! {
                            EnvelopeIcon {}
                        },
                        onchange: move |value| {
                            email.set(value);
                        },
                    }
                    Input {
                        title: "Username",
                        placeholder: "Username",
                        icon: rsx! {
                            UserIcon {}
                        },
                        onchange: move |value| {
                            username.set(value);
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

                        SolidButton {
                            onclick: move |_| {
                                email_sent.set(true);
                            },
                            "Sign up"
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
    }
}
