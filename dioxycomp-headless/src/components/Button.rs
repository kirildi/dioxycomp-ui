//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    pub id: Option<String>,
    pub label: Option<String>,
    pub autofocus: Option<bool>,
    pub disabled: Option<bool>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub styles: Option<String>,
}

pub fn Button(button_props: ButtonProps) -> Element {
    let mut state = use_signal(|| false);
    rsx! {
        button {
            id: button_props.id,
            autofocus:  button_props.autofocus,
            disabled:  button_props.disabled,
            name:  button_props.name,
            r#type:  button_props.r#type,
            value: button_props.value,
            style:  button_props.styles,
            onclick: move |_| state.toggle(),
            "{button_props.label.unwrap()}"
        }
    }
}
