//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::prelude::*;
use dioxus_elements::button;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    pub id: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub autofocus: Option<bool>,
    pub disabled: Option<bool>,
    pub styles: Option<String>,
    pub on_click: EventHandler<MouseEvent>,
    pub children: Element,
}

pub fn Button(
    ButtonProps {
        id,
        name,
        r#type,
        value,
        styles,
        disabled,
        autofocus,
        on_click,
        children,
    }: ButtonProps,
) -> Element {
    rsx! {
        button {
            id: id,
            name:  name,
            r#type: r#type,
            value: value,
            style:  styles,
            autofocus: autofocus,
            disabled:  disabled,
            onclick: move |event| on_click.call(event),
            {children}
        }
    }
}
