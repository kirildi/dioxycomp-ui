//! # Radio Component
//!
//! Renders a Radio component which can be clicked to mark a item from a list of single items

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Radio(
    onclick: EventHandler<MouseEvent>,
    id: Option<String>,
    class_name: Option<String>,
    styles: Option<String>,
) -> Element {
    rsx! {
        input {
            r#type: "radio",
            id: id,
            class: class_name,
            style: styles,
            onclick: move |event| onclick.call(event),
        }
    }
}
