use std::rc::Rc;

use dioxus::prelude::*;

use crate::layout::Row;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    // number of characters in the input field
    length: usize,
    onchange: Callback<String>,
}

#[allow(non_snake_case)]
pub fn CodeInput(props: Props) -> Element {
    let mut value = use_signal(|| Vec::with_capacity(props.length));
    let mut input_refs = use_signal(|| Vec::<Rc<MountedData>>::with_capacity(props.length));
    rsx! {
        Row {
            gap: "12px",
            height: "80px",
            for idx in 0..props.length {
                input {
                    r#type: "number",
                    oninput: move |e| {
                        // We need to clean this up but it will work for now
                        // if the value is deleted then focus on previous
                        if e.value().len() == 0 {
                            if idx > 0 {
                                let input_to_focus = input_refs.read()[idx - 1].clone();
                                // input to focus is an rc
                                spawn(async move {
                                    input_to_focus.set_focus(true).await.unwrap()
                                });
                            }
                        }else {
                            if let Ok(new_value) = e.value().parse::<u32>() {
                                value.write()[idx] = new_value;
                                if idx < props.length - 1 {
                                    let input_to_focus = input_refs.read()[idx + 1].clone();
                                    // input to focus is an rc
                                    spawn(async move {
                                        input_to_focus.set_focus(true).await.unwrap()
                                    });
                                }
                            }
                        }
                    },
                    onmounted: move |e| {
                        input_refs.write().push(e.data())
                    },
                    style: "border: 1px solid #000;",
                    maxlength: 1, // every input only accepts 1 element
                }
            }

        }

    }
}
