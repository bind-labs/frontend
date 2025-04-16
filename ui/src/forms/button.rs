use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(extends = GlobalAttributes, extends = button)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[allow(non_snake_case)]
#[component]
pub fn SolidButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            style: "background-color: #000000; color: #ffffff; border-radius: 0; border-width: 1px; border-color: #000000; padding: 16px;",
            ..props.attributes,
            {props.children}
        }
    }
}



#[allow(non_snake_case)]
#[component]
pub fn UnstyledButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            style: "appearance: none; border: none; outline: none; background-color: transparent; color: inherit; padding: 0; margin: 0;",
            ..props.attributes,
            {props.children}
        }
    }
}


