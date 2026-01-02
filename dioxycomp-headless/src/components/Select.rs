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
        style: styles,
        value: selected(),
        onchange: move |event| {
          event.prevent_default();
          selected.set(event.value()
        )},
        option {
          value: "option 2",
          "option 1"
        },
        option {
          value: "option 2",
          "option 2"
        },
        option {
          value: "option 3",
          "option 3"
        }
    }
}
