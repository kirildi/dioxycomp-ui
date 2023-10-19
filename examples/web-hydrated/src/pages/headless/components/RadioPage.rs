#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Radio::Radio;

#[inline_props]
pub fn RadioPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        h1 {
        "This is a radio component page"

        },
        p {
            Radio {}
        }

    })
}
