//! # Select Component
//!
//! Renders a Select component which can be clicked to select option from a list

#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Select(cx: Scope) -> Element {
    let state = use_state(&cx, || (0, ""));

    cx.render(rsx!(
      select {
        style: "width: 9rem; height: 2rem; padding-left: 1rem; background-color: #404040;",
        onchange: move |_| state.set((0,"option 1")),
        option {
          id: 0,
          "option 1"
        },
        option {
          id: 1,
          "option 2"
        },
        option {
          id: 2,
          "option 3"
        }
    }))
}
