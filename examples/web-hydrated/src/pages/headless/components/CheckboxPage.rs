#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Checkbox::Checkbox;

#[inline_props]
pub fn CheckboxPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        h1 {
        "This is a checkbox page"

        },
        p {
            Checkbox {}
        }

    })
}
