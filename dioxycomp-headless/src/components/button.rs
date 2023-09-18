#![allow(non_snake_case)]

//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

use dioxus::prelude::*;

fn Button(cx: Scope) -> Element {
    let state = use_state(&cx, || false);

    cx.render(rsx!(button {
        style: "width:4em; height:2em; padding: 1em; font-size: 3em;",
        onclick: move |_| state.set(!state),
        "OK",
    }))
}
