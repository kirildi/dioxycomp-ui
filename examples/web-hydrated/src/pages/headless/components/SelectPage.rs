#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Select::Select;

#[inline_props]
pub fn SelectPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        h1 {
        "This is a select page"

        },
        p {
            Select {}
        }

    })
}
