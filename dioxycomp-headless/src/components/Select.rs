//! # Select Component
//!
//! Renders a Select component which can be clicked to select option from a listbox

#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    pub id: Option<String>,
    pub value: String,
    pub class_name: Option<String>,
    pub styles: Option<String>,
    pub children: Element,
}

pub fn Select(
    SelectProps {
        id,
        value,
        class_name,
        styles,
        children,
    }: SelectProps,
) -> Element {
    let mut selected: Signal<String> = use_signal(|| value);
    rsx! {
      select {
        id: id,
        class: class_name,
        style: styles,
        value: selected,
        onchange: move |event| {
          event.prevent_default();
            selected.set(event.value()
        )},
        {children}
      }
    }
}
