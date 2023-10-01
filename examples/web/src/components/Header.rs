#![allow(non_snake_case)]

use super::Nav::Nav;

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header{
            class: "container flex h-24 content-between bg-neutral-800",
            div {
                class: "ml-0"
                //TODO Convert to LOGO component
            },
            Nav {}
        }
    })
}
