//! # Radio Component
//!
//! Renders a Radio component which can be clicked to mark a item from a list of single items

#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Radio(cx: Scope) -> Element {
    let state = use_state(&cx, || false);

    cx.render(rsx!(input {
        r#type: "radio",
        id: "radio-n",
        style: "width:1em; height:1em;",
        onclick: move |_| state.set(!state),
      }
      label {
        style: "padding-left: 1rem",
        r#for: "radio-n",
        "Selected"
      }
    ))
}
