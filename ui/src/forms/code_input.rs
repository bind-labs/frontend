use dioxus::{logger::tracing, prelude::*};
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
    let mut value: Signal<Vec<u32>> = use_signal(|| Vec::with_capacity(props.length));
    let mut input_refs =
        use_signal(|| HashMap::<usize, Rc<MountedData>>::with_capacity(props.length));

    rsx! {
        Row {
            gap: "12px",
            height: "80px",
            align_self: "center",

            for idx in 0..props.length {
                input {
                    type: "number",

                    oninput: move |e| {
                        // Focus previous input
                        if e.value().len() == 0 {
                            if let Some(input_to_focus) = input_refs().get(&(idx - 1)).cloned() {
                                input_to_focus.set_focus(true);
                            }
                        // Focus next input
                        } else if let Ok(new_value) = e.value().parse::<u32>() {
                            if let Some(input_to_focus) = input_refs().get(&(idx + 1)).cloned() {
                                input_to_focus.set_focus(true);
                            }
                        }
                    },
                    onmounted: move |e| {
                        input_refs.write().insert(idx, e.data());
                    },

                    style: "border: 1px solid var(--text); background: rgba(255, 255, 255, 0.2); width: 60px; height: 80px; font-size: 48px; text-align: center; outline: none;",

                    maxlength: 1, // every input only accepts 1 number
                }
            }

        }

    }
}
