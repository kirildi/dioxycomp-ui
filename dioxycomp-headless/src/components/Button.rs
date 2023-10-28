//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Button(cx: Scope) -> Element {
    let state = use_state(&cx, || false);

    cx.render(rsx!(button {
        style: "width:3em; height:2em; font-size: 1em; border:1px solid #fef",
        onclick: move |_| state.set(!state),
        "OK",
    }))
}