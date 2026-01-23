//! # Radio Component
//!
//! Renders a Radio component which can be clicked to mark a item from a list of single items

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    id: Option<String>,
    class_name: Option<String>,
    style: Option<String>,
    on_click: EventHandler<MouseEvent>,
    children: Element,
}

pub fn Radio(
    RadioProps {
        id,
        class_name,
        style,
        on_click,
        children,
    }: RadioProps,
) -> Element {
    rsx! {
        input {
            r#type: "radio",
            id: id,
            class: class_name,
            style: style,
            onclick: move |event| on_click.call(event),
            {children}
        }
    }
}
