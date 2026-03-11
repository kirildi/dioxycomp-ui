//! # Slider Component
//!
//! Renders a Slider component, which is usually used to control values in a specific range

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SliderProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
    #[props(default = Some(false))]
    pub state: Option<bool>,
    pub children: Element,
}

pub fn Slider(
    SliderProps {
        id,
        class_name,
        style,
        state,
        children,
    }: SliderProps,
) -> Element {
    let mut is_open = use_signal(|| false);

    match state {
        Some(st) => is_open.set(st),
        None => is_open.set(false),
    }

    rsx! {
        if is_open(){
            div {
                id: id,
                class: class_name,
                style: style,
                role: "dialog",
                aria_modal: "true",
                {children}
            },
        }
    }
}
