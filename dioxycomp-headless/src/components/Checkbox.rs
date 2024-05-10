//! # Checkbox Component
//!
//! Renders a Checkbox which can be clicked to toggle

#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Checkbox() -> Element {
    let mut state = use_signal(|| false);

    rsx!(
        input {
            r#type: "checkbox",
            name: "checkbox",
            style: "width:1em; height:1em;",
            onclick: move |_| state.toggle() ,
        },
        label{
            style: "padding-left: 1rem",
            r#for: "checkbox",
            "Check me"
        },
    )
}
