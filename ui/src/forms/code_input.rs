use dioxus::prelude::*;
use std::{collections::HashMap, rc::Rc};

use crate::layout::Row;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    // number of characters in the input field
    length: usize,
    onchange: Callback<String>,
}

#[allow(non_snake_case)]
pub fn CodeInput(props: Props) -> Element {
    let mut code = use_signal(String::new);
    let mut input_refs =
        use_signal(|| HashMap::<usize, Rc<MountedData>>::with_capacity(props.length));

    use_effect(move || {
        props.onchange.call(code().clone());
    });

    rsx! {
        Row {
            gap: "12px",
            height: "80px",
            align_self: "center",

            for idx in 0..props.length {
                input {
                    type: "number",

                    value: code().get(idx..idx + 1),

                    onkeydown: move |e| {
                        match e.key() {
                            Key::Backspace => {
                                code.set(code()[0..code().len() - 1].to_string());
                            },
                            Key::Character(c) => {
                                if let Some(c) = c.parse::<u32>().ok() {
                                    if code().len() < 5 && c >= 0 && c <= 9 {
                                        code.set(format!("{}{}", code(), c));
                                    }
                                }
                            }
                            _ => {}
                        }

                        if let Some(input_to_focus) = input_refs().get(&code().len()).cloned() {
                            input_to_focus.set_focus(true);
                        }
                    },

                    onmounted: move |e| {
                        input_refs.write().insert(idx, e.data());
                    },

                    style: "border: 1px solid var(--text); background: rgba(255, 255, 255, 0.2); width: 60px; height: 80px; font-size: 48px; text-align: center; outline: none;",

                    min: 0,
                    max: 9
                }
            }

        }

    }
}
