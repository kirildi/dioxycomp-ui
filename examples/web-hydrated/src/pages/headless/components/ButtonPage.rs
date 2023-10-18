#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::SimpleButton::SimpleButton;

#[inline_props]
pub fn ButtonPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        h1 {
        "This is a button page"

        },
        p {
            SimpleButton {}
        }

    })
}
