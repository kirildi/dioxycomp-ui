//! # Select Component
//!
//! Renders a Select component which can be clicked to select option from a listbox

#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Select() -> Element {
    let mut selected = use_signal(|| "option 1".to_string());
    let styles: String =
        String::from("width: 9rem; height: 2rem; padding-left: 1rem; background-color: #404040;");

    rsx!(
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
    })
}
