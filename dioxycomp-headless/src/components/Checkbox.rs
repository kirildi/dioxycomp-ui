//! # Checkbox Component
//!
//! Renders a Checkbox which can be clicked to toggle

#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Checkbox(cx: Scope) -> Element {
    let state = use_state(&cx, || false);

    cx.render(rsx!(
        input {
            r#type: "checkbox",
            name: "checkbox",
            style: "width:1em; height:1em;",
            onclick: move |_| state.set(!state),
        },
        label{
            style: "padding-left: 1rem",
            r#for: "checkbox",
            "Check me"
        },
    ))
}
