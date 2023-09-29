#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {nav{
        class: "h-16",
    }})
}
