#![allow(non_snake_case)]

use super::Nav::Nav;

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header{
        class: "container flex h-16 bg-neutral-800",
        Nav {}
        }
    })
}
