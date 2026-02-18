//! # Checkbox Component
//!
//! Renders a Checkbox which can be clicked to toggle

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]

pub struct CheckboxProps {
    pub id: Option<String>,
    pub name: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
    pub on_click: EventHandler<MouseEvent>,
    pub children: Element,
}

pub fn Checkbox(
    CheckboxProps {
        id,
        name,
        class_name,
        style,
        on_click,
        children,
    }: CheckboxProps,
) -> Element {
    rsx!(input {
        id: id,
        r#type: "checkbox",
        name: name,
        class: class_name,
        style: style,
        onclick: move |event| on_click.call(event),
        {children}
    },)
}
