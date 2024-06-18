//! # Select Component
//!
//! Renders a Select component which can be clicked to select option from a listbox

#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Select(
    id: Option<String>,
    value: String,
    class_name: Option<String>,
    styles: Option<String>,
    children: Element,
) -> Element {
    let mut selected: Signal<String> = use_signal(|| value);

    rsx! {
      select {
            value: "{selected()}",
            prevent_default: "onchange",
            style: styles,
            onchange: move |event| selected.set(event.value()),
            {children}
        }
    }
}
