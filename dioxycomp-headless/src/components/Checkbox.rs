//! # Checkbox Component
//!
//! Renders a Checkbox which can be clicked to toggle

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    onpress: EventHandler<MouseEvent>,
    name: Option<String>,
    class_name: Option<String>,
    styles: Option<String>,
) -> Element {
    rsx!(input {
        r#type: "checkbox",
        name: name,
        class: class_name,
        style: styles,
        onclick: move |event| onpress.call(event),
    },)
}
