#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Button(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        h1 {
        "This is a button page"

        },

    })
}
