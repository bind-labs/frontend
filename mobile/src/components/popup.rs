use dioxus::{document::document, prelude::*};
use ui::{
    forms::button::{TransparentButton, UnstyledButton},
    layout::{Column, Row},
};

use crate::views::Route;

#[derive(Debug, Clone, PartialEq)]
pub enum PopupState {
    Open(Element),
    Close,
}

pub fn use_popup_state_provider() -> Signal<PopupState> {
    use_context_provider(|| Signal::new(PopupState::Close))
}

pub fn use_popup_state() -> Signal<PopupState> {
    use_context::<Signal<PopupState>>()
}

// TODO: slide down animation
#[component]
pub fn Popup() -> Element {
    let mut state = use_popup_state();

    // Close popup when route changes
    let route = use_route::<Route>();
    let mut previous_route = use_signal(|| route.clone());
    if route.clone() != previous_route() {
        state.set(PopupState::Close);
        previous_route.set(route.clone());
    }

    // When the popup opens, store the element to be used later
    // When the popup closes, create a task that removes the element after 120ms
    // to allow us to slide the popup out
    let mut element = use_signal(|| None::<Element>);
    let mut transition_task = use_signal(|| None::<Task>);
    use_effect(move || match state() {
        PopupState::Open(element_) => {
            if let Some(task) = transition_task() {
                task.cancel();
                transition_task.set(None);
            }
            element.set(Some(element_.clone()));
        }
        PopupState::Close if element().is_some() => {
            // Close fully after 120ms
            transition_task.set(Some(spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(120)).await;
                element.set(None);
                transition_task.set(None);
            })));
        }
        _ => {}
    });

    match state() {
        PopupState::Open(element) => {
            rsx! {
                div { id: "popup-backdrop",
                    onclick: move |ev| {
                        state.set(PopupState::Close);
                        ev.stop_propagation();
                    },
                }
                dialog { id: "popup",
                    {element}
                }
            }
        }

        PopupState::Close => match element() {
            Some(element) => {
                rsx! { dialog { id: "popup", class: "slide-out",
                    {element}
                } }
            }
            None => rsx! {},
        },
    }
}

#[component]
pub fn PopupList(children: Element) -> Element {
    rsx! {
        Column { padding: "12px 0px", width: "100%", cross_align: "stretch",
            children
        }
    }
}

#[component]
pub fn PopupListItem(
    icon: Option<Element>,
    title: String,
    subtitle: Option<String>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let mut state = use_popup_state();

    rsx! {
        UnstyledButton { padding: "8px 24px", font_size: "18px", align_items: "center", justify_content: "flex-start", onclick: move |ev| {
            onclick.call(ev);
            state.set(PopupState::Close);
        },
            {icon}
            span { class: "title",
                {title}
            }
            {subtitle.map(|subtitle| rsx! {
                span { class: "subtitle",
                    {subtitle}
                }
            })}
        }
    }
}
