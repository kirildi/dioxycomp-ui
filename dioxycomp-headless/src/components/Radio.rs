//! # Radio Component
//!
//! Renders a Radio component which can be clicked to mark a item from a list of single items

#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Radio() -> Element {
    let mut state = use_signal(|| false);

    rsx!(
      input {
        r#type: "radio",
        id: "radio-n",
        style: "width:1em; height:1em;",
        onclick: move |_| state.toggle(),
      }
      label {
        style: "padding-left: 1rem",
        r#for: "radio-n",
        "Selected"
      }
    )
}
