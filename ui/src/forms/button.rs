use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(extends = GlobalAttributes, extends = button)]
    pub attributes: Vec<Attribute>,
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}

#[component]
pub fn SolidButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "solid",
            onclick: move |evt| props.onclick.call(evt),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn UnstyledButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            onclick: move |evt| props.onclick.call(evt),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TransparentButton(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "transparent",
            onclick: move |evt| props.onclick.call(evt),
            ..props.attributes,
            {props.children}
        }
    }
}
